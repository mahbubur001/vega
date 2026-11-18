//! Stores synced files in one secret GitHub Gist per user.

use std::collections::BTreeMap;
use std::sync::Arc;

use anyhow::{Context as _, Result, anyhow, bail};
use http_client::{AsyncBody, HttpClient, Method, Response, StatusCode};
use serde::Deserialize;
use serde_json::json;

use crate::github::{api_request, read_body};

/// Identifies VEGA's sync gist among the user's gists. Matched by file name,
/// not description, because users can edit descriptions.
pub(crate) const MARKER_FILE: &str = "vega-sync.json";
const DESCRIPTION: &str = "VEGA Settings Sync (managed by VEGA, please don't edit)";
const PAGE_SIZE: usize = 100;
const MAX_PAGES: usize = 30;

/// A sync gist and the contents of its files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Gist {
    pub id: String,
    pub files: BTreeMap<String, String>,
    pub etag: Option<String>,
}

pub(crate) enum Fetched {
    /// The gist hasn't changed since the given ETag.
    NotModified,
    Found(Gist),
    /// The gist was deleted.
    Missing,
}

#[derive(Deserialize)]
struct GistResponse {
    id: String,
    public: bool,
    files: BTreeMap<String, GistFileResponse>,
}

#[derive(Deserialize)]
struct GistFileResponse {
    content: Option<String>,
    #[serde(default)]
    truncated: bool,
    raw_url: Option<String>,
}

#[derive(Deserialize)]
struct GistSummary {
    id: String,
    updated_at: String,
    files: BTreeMap<String, serde_json::Value>,
}

/// Finds the user's existing sync gist, picking the most recently updated one
/// if there are several.
pub(crate) async fn find_sync_gist(
    http_client: &Arc<dyn HttpClient>,
    token: &str,
) -> Result<Option<String>> {
    let mut newest: Option<GistSummary> = None;
    for page in 1..=MAX_PAGES {
        let path = format!("/gists?per_page={PAGE_SIZE}&page={page}");
        let request = api_request(Method::GET, &path, token).body(AsyncBody::empty())?;
        let mut response = http_client.send(request).await?;
        let body = read_body(&mut response).await?;
        check_status(&response, &body, "listing gists")?;

        let gists: Vec<GistSummary> =
            serde_json::from_slice(&body).context("parsing the gist list")?;
        let page_len = gists.len();
        for gist in gists {
            if gist.files.contains_key(MARKER_FILE)
                && newest
                    .as_ref()
                    .is_none_or(|newest| gist.updated_at > newest.updated_at)
            {
                newest = Some(gist);
            }
        }
        if page_len < PAGE_SIZE {
            break;
        }
    }
    Ok(newest.map(|gist| gist.id))
}

/// Fetches the gist, or reports that it is unchanged since `etag`.
pub(crate) async fn fetch_gist(
    http_client: &Arc<dyn HttpClient>,
    token: &str,
    id: &str,
    etag: Option<&str>,
) -> Result<Fetched> {
    let mut request = api_request(Method::GET, &format!("/gists/{id}"), token);
    if let Some(etag) = etag {
        request = request.header("If-None-Match", etag);
    }
    let mut response = http_client.send(request.body(AsyncBody::empty())?).await?;
    let body = read_body(&mut response).await?;
    match response.status() {
        StatusCode::NOT_MODIFIED => return Ok(Fetched::NotModified),
        StatusCode::NOT_FOUND => return Ok(Fetched::Missing),
        _ => check_status(&response, &body, "reading the sync gist")?,
    }

    let etag = response
        .headers()
        .get("ETag")
        .and_then(|value| value.to_str().ok())
        .map(str::to_string);
    let mut gist = parse_gist(http_client, token, &body).await?;
    gist.etag = etag;
    Ok(Fetched::Found(gist))
}

/// Creates a new secret sync gist holding `files`.
pub(crate) async fn create_gist(
    http_client: &Arc<dyn HttpClient>,
    token: &str,
    files: &BTreeMap<String, String>,
) -> Result<Gist> {
    let files: serde_json::Map<String, serde_json::Value> = files
        .iter()
        .map(|(name, content)| (name.clone(), json!({ "content": content })))
        .collect();
    let body = json!({ "description": DESCRIPTION, "public": false, "files": files });
    let request = api_request(Method::POST, "/gists", token)
        .header("Content-Type", "application/json")
        .body(AsyncBody::from(serde_json::to_vec(&body)?))?;
    let mut response = http_client.send(request).await?;
    let body = read_body(&mut response).await?;
    check_status(&response, &body, "creating the sync gist")?;
    parse_gist(http_client, token, &body).await
}

/// Writes `changes` to the gist. A `None` value deletes that file.
pub(crate) async fn update_gist(
    http_client: &Arc<dyn HttpClient>,
    token: &str,
    id: &str,
    changes: &BTreeMap<String, Option<String>>,
) -> Result<Gist> {
    let files: serde_json::Map<String, serde_json::Value> = changes
        .iter()
        .map(|(name, content)| {
            let value = match content {
                Some(content) => json!({ "content": content }),
                None => serde_json::Value::Null,
            };
            (name.clone(), value)
        })
        .collect();
    let body = json!({ "files": files });
    let request = api_request(Method::PATCH, &format!("/gists/{id}"), token)
        .header("Content-Type", "application/json")
        .body(AsyncBody::from(serde_json::to_vec(&body)?))?;
    let mut response = http_client.send(request).await?;
    let body = read_body(&mut response).await?;
    check_status(&response, &body, "updating the sync gist")?;
    parse_gist(http_client, token, &body).await
}

async fn parse_gist(http_client: &Arc<dyn HttpClient>, token: &str, body: &[u8]) -> Result<Gist> {
    let response: GistResponse = serde_json::from_slice(body).context("parsing the sync gist")?;
    if response.public {
        // Never sync into a public gist, even if someone made it public.
        bail!("The sync gist is public. Make it secret on GitHub, or delete it, to keep syncing.");
    }

    let mut files = BTreeMap::new();
    for (name, file) in response.files {
        let content = match (file.content, file.truncated, file.raw_url) {
            (_, true, Some(raw_url)) => fetch_raw(http_client, token, &raw_url).await?,
            (Some(content), _, _) => content,
            (None, _, _) => continue,
        };
        files.insert(name, content);
    }

    Ok(Gist {
        id: response.id,
        files,
        etag: None,
    })
}

async fn fetch_raw(http_client: &Arc<dyn HttpClient>, token: &str, url: &str) -> Result<String> {
    let request = http_client::Request::builder()
        .method(Method::GET)
        .uri(url)
        .header("Authorization", format!("Bearer {token}"))
        .body(AsyncBody::empty())?;
    let mut response = http_client.send(request).await?;
    let body = read_body(&mut response).await?;
    check_status(&response, &body, "downloading a large gist file")?;
    String::from_utf8(body).context("gist file isn't valid UTF-8")
}

fn check_status(response: &Response<AsyncBody>, body: &[u8], action: &str) -> Result<()> {
    let status = response.status();
    if status.is_success() {
        return Ok(());
    }
    let remaining = response
        .headers()
        .get("x-ratelimit-remaining")
        .and_then(|value| value.to_str().ok());
    if status == StatusCode::TOO_MANY_REQUESTS
        || (status == StatusCode::FORBIDDEN && remaining == Some("0"))
    {
        bail!("GitHub's rate limit was reached while {action}. VEGA will try again later.");
    }
    if status == StatusCode::UNAUTHORIZED {
        bail!("GitHub rejected the sign-in while {action}. Please sign in again.");
    }
    let message = serde_json::from_slice::<serde_json::Value>(body)
        .ok()
        .and_then(|value| value.get("message")?.as_str().map(str::to_string))
        .unwrap_or_default();
    Err(anyhow!(
        "GitHub returned {status} while {action}. {message}"
    ))
}
