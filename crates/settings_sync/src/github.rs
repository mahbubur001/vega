//! GitHub OAuth device flow and REST helpers for Settings Sync.
//!
//! VEGA signs in with its own GitHub OAuth App using the device flow, which
//! needs no client secret, so it is safe to ship in an open-source binary.

use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context as _, Result, anyhow, bail};
use futures::AsyncReadExt as _;
use gpui::BackgroundExecutor;
use http_client::{AsyncBody, Builder, HttpClient, Method, Request, Response, StatusCode};
use serde::Deserialize;

/// The public client ID of VEGA's GitHub OAuth App. Forks can override it with
/// the `VEGA_GITHUB_CLIENT_ID` environment variable, at build time or run time.
const DEFAULT_CLIENT_ID: &str = "";

/// `gist` is the narrowest scope that can store data. The user's login and
/// avatar come from `GET /user`, which works with any token.
const SCOPE: &str = "gist";

const DEVICE_CODE_URL: &str = "https://github.com/login/device/code";
const ACCESS_TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
const DEVICE_CODE_GRANT_TYPE: &str = "urn:ietf:params:oauth:grant-type:device_code";
pub(crate) const API_URL: &str = "https://api.github.com";
const API_VERSION: &str = "2022-11-28";

pub(crate) fn client_id() -> Option<String> {
    std::env::var("VEGA_GITHUB_CLIENT_ID")
        .ok()
        .or_else(|| option_env!("VEGA_GITHUB_CLIENT_ID").map(str::to_string))
        .or_else(|| Some(DEFAULT_CLIENT_ID.to_string()))
        .filter(|client_id| !client_id.is_empty())
}

/// An in-progress device-code authorization. The user must visit
/// [`Self::verification_uri`] and enter [`Self::user_code`] to complete it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeviceFlow {
    pub user_code: String,
    pub verification_uri: String,
    device_code: String,
    interval: u64,
}

#[derive(Deserialize)]
struct DeviceCodeResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    #[serde(default = "default_interval")]
    interval: u64,
}

fn default_interval() -> u64 {
    5
}

#[derive(Deserialize)]
struct AccessTokenResponse {
    access_token: Option<String>,
    error: Option<String>,
    interval: Option<u64>,
}

/// The signed-in GitHub user.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct GitHubUser {
    pub login: String,
    pub avatar_url: String,
    pub name: Option<String>,
}

/// Begins a device-code authorization by requesting a user code from GitHub.
pub(crate) async fn request_device_code(
    http_client: &Arc<dyn HttpClient>,
    client_id: &str,
) -> Result<DeviceFlow> {
    let body = form_encode(&[("client_id", client_id), ("scope", SCOPE)]);
    let request = Request::builder()
        .method(Method::POST)
        .uri(DEVICE_CODE_URL)
        .header("Accept", "application/json")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(AsyncBody::from(body))?;

    let mut response = http_client.send(request).await?;
    let body = read_body(&mut response).await?;
    anyhow::ensure!(
        response.status().is_success(),
        "GitHub device-code request failed: {}",
        response.status()
    );
    let parsed: DeviceCodeResponse =
        serde_json::from_slice(&body).context("parsing GitHub device-code response")?;

    Ok(DeviceFlow {
        user_code: parsed.user_code,
        verification_uri: parsed.verification_uri,
        device_code: parsed.device_code,
        interval: parsed.interval.max(1),
    })
}

/// Polls GitHub until the user completes the device-code authorization and
/// returns the resulting OAuth token.
pub(crate) async fn poll_for_access_token(
    http_client: &Arc<dyn HttpClient>,
    client_id: &str,
    device_flow: &DeviceFlow,
    executor: &BackgroundExecutor,
) -> Result<String> {
    let mut interval = device_flow.interval;
    let body = form_encode(&[
        ("client_id", client_id),
        ("device_code", device_flow.device_code.as_str()),
        ("grant_type", DEVICE_CODE_GRANT_TYPE),
    ]);

    loop {
        executor.timer(Duration::from_secs(interval)).await;

        let request = Request::builder()
            .method(Method::POST)
            .uri(ACCESS_TOKEN_URL)
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(AsyncBody::from(body.clone()))?;

        let mut response = http_client.send(request).await?;
        let response_body = read_body(&mut response).await?;
        let parsed: AccessTokenResponse = serde_json::from_slice(&response_body)
            .context("parsing GitHub access-token response")?;

        if let Some(token) = parsed.access_token {
            return Ok(token);
        }

        match parsed.error.as_deref() {
            Some("authorization_pending") => {}
            // GitHub asks us to back off and may say by how much.
            Some("slow_down") => interval = parsed.interval.unwrap_or(interval + 5),
            Some("expired_token") => bail!("The sign-in code expired. Please try again."),
            Some("access_denied") => bail!("Sign-in was cancelled on GitHub."),
            Some("device_flow_disabled") => {
                bail!("Device flow is disabled for VEGA's GitHub OAuth App.")
            }
            Some(other) => bail!("GitHub sign-in failed: {other}"),
            None => bail!("GitHub sign-in failed: unexpected response from GitHub"),
        }
    }
}

/// Fetches the user that owns `token`.
///
/// Returns `Ok(None)` when GitHub rejects the token, for example because it
/// was revoked or replaced after signing in on more than ten machines.
pub(crate) async fn fetch_user(
    http_client: &Arc<dyn HttpClient>,
    token: &str,
) -> Result<Option<GitHubUser>> {
    let request = api_request(Method::GET, "/user", token).body(AsyncBody::empty())?;
    let mut response = http_client.send(request).await?;
    let body = read_body(&mut response).await?;
    match response.status() {
        StatusCode::UNAUTHORIZED => Ok(None),
        status if status.is_success() => Ok(Some(
            serde_json::from_slice(&body).context("parsing GitHub user")?,
        )),
        status => Err(anyhow!("GitHub user request failed: {status}")),
    }
}

/// Starts a request to the GitHub REST API with VEGA's standard headers.
pub(crate) fn api_request(method: Method, path: &str, token: &str) -> Builder {
    Request::builder()
        .method(method)
        .uri(format!("{API_URL}{path}"))
        .header("Authorization", format!("Bearer {token}"))
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", API_VERSION)
}

pub(crate) async fn read_body(response: &mut Response<AsyncBody>) -> Result<Vec<u8>> {
    let mut body = Vec::new();
    response.body_mut().read_to_end(&mut body).await?;
    Ok(body)
}

fn form_encode(fields: &[(&str, &str)]) -> String {
    fields
        .iter()
        .map(|(key, value)| format!("{}={}", url_encode(key), url_encode(value)))
        .collect::<Vec<_>>()
        .join("&")
}

fn url_encode(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(byte as char)
            }
            _ => encoded.push_str(&format!("%{byte:02X}")),
        }
    }
    encoded
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use gpui::TestAppContext;
    use http_client::FakeHttpClient;

    use super::*;

    fn test_device_flow() -> DeviceFlow {
        DeviceFlow {
            user_code: "ABCD-1234".into(),
            verification_uri: "https://github.com/login/device".into(),
            device_code: "device-code".into(),
            interval: 5,
        }
    }

    fn json_response(status: u16, body: &'static str) -> anyhow::Result<Response<AsyncBody>> {
        Ok(Response::builder().status(status).body(body.into())?)
    }

    #[gpui::test]
    async fn test_poll_waits_for_authorization(cx: &mut TestAppContext) {
        let requests = Arc::new(AtomicUsize::new(0));
        let http_client: Arc<dyn HttpClient> = FakeHttpClient::create({
            let requests = requests.clone();
            move |request| {
                let request_index = requests.fetch_add(1, Ordering::SeqCst);
                async move {
                    assert_eq!(request.uri().to_string(), ACCESS_TOKEN_URL);
                    match request_index {
                        0 => json_response(200, r#"{"error":"authorization_pending"}"#),
                        1 => json_response(200, r#"{"error":"slow_down","interval":10}"#),
                        _ => json_response(200, r#"{"access_token":"gho_test"}"#),
                    }
                }
            }
        });

        let executor = cx.executor();
        let task = executor.spawn({
            let executor = executor.clone();
            async move {
                poll_for_access_token(&http_client, "client-id", &test_device_flow(), &executor)
                    .await
            }
        });

        executor.advance_clock(Duration::from_secs(5));
        executor.run_until_parked();
        assert_eq!(requests.load(Ordering::SeqCst), 1);

        executor.advance_clock(Duration::from_secs(5));
        executor.run_until_parked();
        assert_eq!(requests.load(Ordering::SeqCst), 2);

        // After `slow_down`, GitHub's new 10 second interval applies.
        executor.advance_clock(Duration::from_secs(5));
        executor.run_until_parked();
        assert_eq!(requests.load(Ordering::SeqCst), 2);

        executor.advance_clock(Duration::from_secs(5));
        executor.run_until_parked();
        assert_eq!(task.await.unwrap(), "gho_test");
    }

    #[gpui::test]
    async fn test_poll_stops_when_access_denied(cx: &mut TestAppContext) {
        let http_client: Arc<dyn HttpClient> = FakeHttpClient::create(|_| async {
            json_response(200, r#"{"error":"access_denied"}"#)
        });

        let executor = cx.executor();
        let task = executor.spawn({
            let executor = executor.clone();
            async move {
                poll_for_access_token(&http_client, "client-id", &test_device_flow(), &executor)
                    .await
            }
        });
        executor.advance_clock(Duration::from_secs(5));
        executor.run_until_parked();

        let error = task.await.unwrap_err();
        assert_eq!(error.to_string(), "Sign-in was cancelled on GitHub.");
    }

    #[gpui::test]
    async fn test_fetch_user(cx: &mut TestAppContext) {
        let http_client: Arc<dyn HttpClient> = FakeHttpClient::create(|request| async move {
            let authorization = request
                .headers()
                .get("Authorization")
                .and_then(|value| value.to_str().ok())
                .unwrap_or_default()
                .to_string();
            assert_eq!(request.uri().to_string(), format!("{API_URL}/user"));
            if authorization == "Bearer valid" {
                json_response(
                    200,
                    r#"{"login":"octocat","avatar_url":"https://avatars.example/octocat","name":"The Octocat"}"#,
                )
            } else {
                json_response(401, r#"{"message":"Bad credentials"}"#)
            }
        });

        let user = cx
            .executor()
            .spawn({
                let http_client = http_client.clone();
                async move { fetch_user(&http_client, "valid").await }
            })
            .await
            .unwrap();
        assert_eq!(
            user,
            Some(GitHubUser {
                login: "octocat".into(),
                avatar_url: "https://avatars.example/octocat".into(),
                name: Some("The Octocat".into()),
            })
        );

        let rejected = cx
            .executor()
            .spawn(async move { fetch_user(&http_client, "revoked").await })
            .await
            .unwrap();
        assert_eq!(rejected, None);
    }
}
