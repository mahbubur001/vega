use anyhow::{Context as _, Result, anyhow, bail};
use futures::AsyncReadExt;
use http_client::{AsyncBody, HttpClient, HttpRequestExt, Request, Url};
use serde::{Deserialize, Deserializer};
use std::{sync::Arc, time::Duration};

const GITHUB_API_URL: &str = "https://api.github.com";
const GITHUB_WEB_URL: &str = "https://github.com";
const GITHUB_RELEASE_REQUEST_TIMEOUT: Duration = Duration::from_secs(10);

pub struct GitHubLspBinaryVersion {
    pub name: String,
    pub url: String,
    pub digest: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GithubRelease {
    pub tag_name: String,
    #[serde(rename = "prerelease")]
    pub pre_release: bool,
    pub assets: Vec<GithubReleaseAsset>,
    pub tarball_url: String,
    pub zipball_url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GithubReleaseAsset {
    pub name: String,
    pub browser_download_url: String,
    #[serde(default, deserialize_with = "deserialize_sha256_digest")]
    pub digest: Option<String>,
}

fn deserialize_sha256_digest<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    const PREFIX: &str = "sha256:";

    let mut digest = Option::<String>::deserialize(deserializer)?;
    if let Some(digest) = digest.as_mut()
        && digest.starts_with(PREFIX)
    {
        digest.replace_range(..PREFIX.len(), "");
    }
    Ok(digest)
}

/// The latest release of `repo_name_with_owner`. VEGA: when the GitHub API
/// refuses (anonymous calls are limited to 60 an hour per IP) or fails, reads
/// the latest stable release from github.com's web pages, which aren't limited.
pub async fn latest_github_release(
    repo_name_with_owner: &str,
    require_assets: bool,
    pre_release: bool,
    http: Arc<dyn HttpClient>,
) -> anyhow::Result<GithubRelease> {
    match latest_from_api(
        repo_name_with_owner,
        require_assets,
        pre_release,
        http.clone(),
    )
    .await
    {
        Ok(release) => Ok(release),
        Err(api_error) if !pre_release => {
            log::warn!(
                "GitHub API failed for {repo_name_with_owner} ({api_error:#}); reading the release from github.com"
            );
            let release =
                latest_from_web(repo_name_with_owner, http)
                    .await
                    .map_err(|web_error| {
                        api_error.context(format!("github.com fallback also failed: {web_error:#}"))
                    })?;
            if require_assets && release.assets.is_empty() {
                bail!("the latest release of {repo_name_with_owner} has no assets");
            }
            Ok(release)
        }
        Err(api_error) => Err(api_error),
    }
}

async fn latest_from_api(
    repo_name_with_owner: &str,
    require_assets: bool,
    pre_release: bool,
    http: Arc<dyn HttpClient>,
) -> anyhow::Result<GithubRelease> {
    let url = format!("{GITHUB_API_URL}/repos/{repo_name_with_owner}/releases");

    let request = github_api_request(&url)?;

    let mut response = http
        .send(request)
        .await
        .context("error fetching latest release")?;

    let mut body = Vec::new();
    response
        .body_mut()
        .read_to_end(&mut body)
        .await
        .context("error reading latest release")?;

    if response.status().is_client_error() {
        let text = String::from_utf8_lossy(body.as_slice());
        bail!(
            "status error {}, response: {text:?}",
            response.status().as_u16()
        );
    }

    let releases = match serde_json::from_slice::<Vec<GithubRelease>>(body.as_slice()) {
        Ok(releases) => releases,

        Err(err) => {
            log::error!("Error deserializing: {err:?}");
            log::error!(
                "GitHub API response text: {:?}",
                String::from_utf8_lossy(body.as_slice())
            );
            anyhow::bail!("error deserializing latest release: {err:?}");
        }
    };

    releases
        .into_iter()
        .filter(|release| !require_assets || !release.assets.is_empty())
        .find(|release| release.pre_release == pre_release)
        .context("finding a prerelease")
}

/// The release tagged `tag`. VEGA: falls back to github.com's web pages when
/// the GitHub API refuses (rate limit) or fails.
pub async fn get_release_by_tag_name(
    repo_name_with_owner: &str,
    tag: &str,
    http: Arc<dyn HttpClient>,
) -> anyhow::Result<GithubRelease> {
    match release_by_tag_from_api(repo_name_with_owner, tag, http.clone()).await {
        Ok(release) => Ok(release),
        Err(api_error) => {
            log::warn!(
                "GitHub API failed for {repo_name_with_owner} {tag} ({api_error:#}); reading the release from github.com"
            );
            release_from_web(repo_name_with_owner, tag, http)
                .await
                .map_err(|web_error| {
                    api_error.context(format!("github.com fallback also failed: {web_error:#}"))
                })
        }
    }
}

/// The latest stable release's tag, from where github.com/<repo>/releases/latest
/// redirects (`…/releases/tag/<tag>`), then its assets.
async fn latest_from_web(
    repo_name_with_owner: &str,
    http: Arc<dyn HttpClient>,
) -> anyhow::Result<GithubRelease> {
    let url = format!("{GITHUB_WEB_URL}/{repo_name_with_owner}/releases/latest");
    let request = Request::get(&url)
        .follow_redirects(http_client::RedirectPolicy::NoFollow)
        .timeout(GITHUB_RELEASE_REQUEST_TIMEOUT)
        .body(AsyncBody::default())?;
    let response = http
        .send(request)
        .await
        .context("error fetching the latest release page")?;
    let location = response
        .headers()
        .get("location")
        .and_then(|value| value.to_str().ok())
        .with_context(|| {
            format!(
                "{url} didn't redirect to a release (status {})",
                response.status()
            )
        })?;
    let tag = tag_from_release_url(location)
        .with_context(|| format!("{url} redirected to {location:?}, not a release"))?;
    release_from_web(repo_name_with_owner, &tag, http).await
}

/// A release built from github.com's asset list for `tag`
/// (`/<repo>/releases/expanded_assets/<tag>`).
async fn release_from_web(
    repo_name_with_owner: &str,
    tag: &str,
    http: Arc<dyn HttpClient>,
) -> anyhow::Result<GithubRelease> {
    let mut url = Url::parse(&format!(
        "{GITHUB_WEB_URL}/{repo_name_with_owner}/releases/expanded_assets"
    ))?;
    url.path_segments_mut()
        .map_err(|()| anyhow!("cannot modify url path segments"))?
        .push(tag);
    let request = Request::get(url.as_str())
        .follow_redirects(http_client::RedirectPolicy::FollowAll)
        .timeout(GITHUB_RELEASE_REQUEST_TIMEOUT)
        .body(AsyncBody::default())?;
    let mut response = http
        .send(request)
        .await
        .context("error fetching the release assets page")?;
    let mut body = Vec::new();
    response.body_mut().read_to_end(&mut body).await?;
    if !response.status().is_success() {
        bail!("{url} answered {}", response.status().as_u16());
    }
    let html = String::from_utf8_lossy(&body);
    Ok(GithubRelease {
        tag_name: tag.to_string(),
        pre_release: false,
        assets: assets_from_html(repo_name_with_owner, &html),
        tarball_url: build_asset_url(repo_name_with_owner, tag, AssetKind::TarGz)?,
        zipball_url: build_asset_url(repo_name_with_owner, tag, AssetKind::Zip)?,
    })
}

/// `0.10.0` from `https://github.com/tamasfe/taplo/releases/tag/0.10.0`.
fn tag_from_release_url(url: &str) -> Option<String> {
    let (_, tag) = url.split_once("/releases/tag/")?;
    let tag = tag.split(['?', '#']).next()?.trim_end_matches('/');
    let tag = percent_decode(tag);
    (!tag.is_empty()).then_some(tag)
}

/// The download links (`href="/<repo>/releases/download/<tag>/<name>"`) on a
/// github.com asset list, in page order, without repeats.
fn assets_from_html(repo_name_with_owner: &str, html: &str) -> Vec<GithubReleaseAsset> {
    let prefix = format!("href=\"/{repo_name_with_owner}/releases/download/");
    let mut assets: Vec<GithubReleaseAsset> = Vec::new();
    let mut rest = html;
    while let Some(start) = rest.find(&prefix) {
        let after = &rest[start + "href=\"".len()..];
        let Some(end) = after.find('"') else { break };
        let path = after[..end].replace("&amp;", "&");
        rest = &after[end..];
        let Some(name) = path.rsplit('/').next().map(percent_decode) else {
            continue;
        };
        if name.is_empty() || assets.iter().any(|asset| asset.name == name) {
            continue;
        }
        assets.push(GithubReleaseAsset {
            name,
            browser_download_url: format!("{GITHUB_WEB_URL}{path}"),
            digest: None,
        });
    }
    assets
}

fn percent_decode(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%'
            && i + 2 < bytes.len()
            && let Ok(byte) = u8::from_str_radix(&text[i + 1..i + 3], 16)
        {
            out.push(byte);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

async fn release_by_tag_from_api(
    repo_name_with_owner: &str,
    tag: &str,
    http: Arc<dyn HttpClient>,
) -> anyhow::Result<GithubRelease> {
    let url = format!("{GITHUB_API_URL}/repos/{repo_name_with_owner}/releases/tags/{tag}");

    let request = github_api_request(&url)?;

    let mut response = http
        .send(request)
        .await
        .context("error fetching latest release")?;

    let mut body = Vec::new();
    let status = response.status();
    response
        .body_mut()
        .read_to_end(&mut body)
        .await
        .context("error reading latest release")?;

    if status.is_client_error() {
        let text = String::from_utf8_lossy(body.as_slice());
        bail!(
            "status error {}, response: {text:?}",
            response.status().as_u16()
        );
    }

    let release = serde_json::from_slice::<GithubRelease>(body.as_slice()).map_err(|err| {
        log::error!("Error deserializing: {err:?}");
        log::error!(
            "GitHub API response text: {:?}",
            String::from_utf8_lossy(body.as_slice())
        );
        anyhow!("error deserializing GitHub release: {err:?}")
    })?;

    Ok(release)
}

fn github_api_request(url: &str) -> Result<Request<AsyncBody>> {
    Request::get(url)
        .follow_redirects(http_client::RedirectPolicy::FollowAll)
        .timeout(GITHUB_RELEASE_REQUEST_TIMEOUT)
        .when_some(std::env::var("GITHUB_TOKEN").ok(), |builder, token| {
            builder.header("Authorization", format!("Bearer {}", token))
        })
        .body(Default::default())
        .map_err(Into::into)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AssetKind {
    TarGz,
    TarBz2,
    Gz,
    Zip,
}

pub fn build_asset_url(repo_name_with_owner: &str, tag: &str, kind: AssetKind) -> Result<String> {
    let mut url = Url::parse(&format!(
        "https://github.com/{repo_name_with_owner}/archive/refs/tags",
    ))?;
    // We're pushing this here, because tags may contain `/` and other characters
    // that need to be escaped.
    let asset_filename = format!(
        "{tag}.{extension}",
        extension = match kind {
            AssetKind::TarGz => "tar.gz",
            AssetKind::TarBz2 => "tar.bz2",
            AssetKind::Gz => "gz",
            AssetKind::Zip => "zip",
        }
    );
    url.path_segments_mut()
        .map_err(|()| anyhow!("cannot modify url path segments"))?
        .push(&asset_filename);
    Ok(url.to_string())
}

#[cfg(test)]
mod tests {
    use super::{
        AssetKind, GITHUB_RELEASE_REQUEST_TIMEOUT, GithubReleaseAsset, build_asset_url,
        github_api_request,
    };
    use http_client::RequestTimeout;

    #[test]
    fn test_asset_digest_deserialization() {
        let parse = |json: &str| {
            serde_json::from_str::<GithubReleaseAsset>(json)
                .unwrap()
                .digest
        };

        assert_eq!(
            parse(r#"{"name":"a","browser_download_url":"u","digest":"sha256:abc"}"#),
            Some("abc".to_string())
        );
        assert_eq!(
            parse(r#"{"name":"a","browser_download_url":"u","digest":"abc"}"#),
            Some("abc".to_string())
        );
        assert_eq!(
            parse(r#"{"name":"a","browser_download_url":"u","digest":null}"#),
            None
        );
        assert_eq!(parse(r#"{"name":"a","browser_download_url":"u"}"#), None);
    }

    #[test]
    fn github_api_requests_have_a_total_deadline() {
        let request =
            github_api_request("https://api.github.com/repos/zed-industries/zed/releases").unwrap();

        assert_eq!(
            request.extensions().get::<RequestTimeout>(),
            Some(&RequestTimeout(GITHUB_RELEASE_REQUEST_TIMEOUT))
        );
    }

    #[test]
    fn reads_the_tag_from_the_latest_release_redirect() {
        assert_eq!(
            super::tag_from_release_url("https://github.com/tamasfe/taplo/releases/tag/0.10.0"),
            Some("0.10.0".to_string())
        );
        assert_eq!(
            super::tag_from_release_url("https://github.com/a/b/releases/tag/release%2F2.3.5"),
            Some("release/2.3.5".to_string())
        );
        assert_eq!(
            super::tag_from_release_url("https://github.com/a/b/releases"),
            None
        );
    }

    #[test]
    fn reads_assets_from_the_release_page() {
        let html = r#"<a href="/tamasfe/taplo/releases/download/0.10.0/taplo-darwin-aarch64.gz" rel="nofollow">
            <a href="/tamasfe/taplo/releases/download/0.10.0/taplo-darwin-aarch64.gz">
            <a href="/tamasfe/taplo/archive/refs/tags/0.10.0.zip">
            <a href="/tamasfe/taplo/releases/download/0.10.0/taplo-full%2Bx.gz">"#;
        let assets = super::assets_from_html("tamasfe/taplo", html);
        let names: Vec<_> = assets.iter().map(|asset| asset.name.as_str()).collect();
        assert_eq!(names, ["taplo-darwin-aarch64.gz", "taplo-full+x.gz"]);
        assert_eq!(
            assets[0].browser_download_url,
            "https://github.com/tamasfe/taplo/releases/download/0.10.0/taplo-darwin-aarch64.gz"
        );
    }

    #[test]
    fn test_build_asset_url() {
        let tag = "release/2.3.5";
        let repo_name_with_owner = "microsoft/vscode-eslint";

        let tarball = build_asset_url(repo_name_with_owner, tag, AssetKind::TarGz).unwrap();
        assert_eq!(
            tarball,
            "https://github.com/microsoft/vscode-eslint/archive/refs/tags/release%2F2.3.5.tar.gz"
        );

        let zip = build_asset_url(repo_name_with_owner, tag, AssetKind::Zip).unwrap();
        assert_eq!(
            zip,
            "https://github.com/microsoft/vscode-eslint/archive/refs/tags/release%2F2.3.5.zip"
        );
    }
}
