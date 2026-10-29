//! OAuth2 user-context tokens (authorization code flow with PKCE).
//!
//! Register the app as a Web App (confidential client) or Native App (public client) with the
//! permissions you need, and add your redirect URI as a callback URL.
//!
//! ```no_run
//! use x_sdk::auth::{OAuth2Client, Scope};
//!
//! # async fn run() -> Result<(), x_sdk::auth::Error> {
//! let client = OAuth2Client::new("client-id").with_secret("client-secret");
//! let redirect = "https://example.com/callback";
//!
//! // 1. Send the user to `request.url`; keep `request` until they come back.
//! let request = client.authorization_url(redirect, &[Scope::TweetRead, Scope::OfflineAccess])?;
//!
//! // 2. On the callback, check `state` and pull out the code.
//! # let callback = url::Url::parse("https://example.com/callback?code=x&state=y").unwrap();
//! let code = request.code_from_callback(&callback)?;
//!
//! // 3. Trade it for tokens.
//! let tokens = client.exchange_code(&code, redirect, &request.code_verifier).await?;
//!
//! let cfg = tokens.configuration();
//! # Ok(()) }
//! ```
//!
//! With the `local` feature, [`OAuth2Client::authorize_local`] runs steps 1-3 for a CLI.

use std::fmt;

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use url::Url;

pub use crate::scope::Scope;

const AUTHORIZE_URL: &str = "https://x.com/i/oauth2/authorize";
const TOKEN_URL: &str = "https://api.x.com/2/oauth2/token";

#[derive(Debug)]
pub enum Error {
    Http(reqwest::Error),
    /// The token endpoint rejected the request.
    Token {
        status: reqwest::StatusCode,
        body: String,
    },
    Random(getrandom::Error),
    Url(url::ParseError),
    Io(std::io::Error),
    /// The redirect URI has no host to listen on.
    InvalidRedirect,
    /// The user declined, or X reported an error on the callback.
    Denied(String),
    StateMismatch,
    MissingCode,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Http(e) => write!(f, "http error: {e}"),
            Error::Token { status, body } => write!(f, "token request failed ({status}): {body}"),
            Error::Random(e) => write!(f, "random source failed: {e}"),
            Error::Url(e) => write!(f, "invalid url: {e}"),
            Error::Io(e) => write!(f, "io error: {e}"),
            Error::InvalidRedirect => write!(f, "redirect uri has no host"),
            Error::Denied(e) => write!(f, "authorization failed: {e}"),
            Error::StateMismatch => write!(f, "state mismatch"),
            Error::MissingCode => write!(f, "no code in callback"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Http(e) => Some(e),
            Error::Random(e) => Some(e),
            Error::Url(e) => Some(e),
            Error::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Http(e)
    }
}

impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self {
        Error::Url(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

/// What came back from the token endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct Tokens {
    pub access_token: String,
    /// Present only if `Scope::OfflineAccess` was granted. Single use: a refresh returns a
    /// replacement, so store the new one.
    pub refresh_token: Option<String>,
    pub expires_in: Option<u64>,
    pub scope: Option<String>,
    pub token_type: Option<String>,
}

impl Tokens {
    /// A [`Config`](crate::Config) that sends the access token.
    pub fn configuration(&self) -> crate::Config {
        crate::Config::bearer(self.access_token.as_str())
    }
}

/// A pending authorization: send the user to `url`, then check the callback against it.
#[derive(Debug, Clone)]
pub struct AuthorizationRequest {
    pub url: Url,
    pub state: String,
    pub code_verifier: String,
}

impl AuthorizationRequest {
    /// Validate the redirect the user landed on and return its authorization code.
    pub fn code_from_callback(&self, callback: &Url) -> Result<String> {
        let param = |name: &str| {
            callback
                .query_pairs()
                .find(|(k, _)| k == name)
                .map(|(_, v)| v.into_owned())
        };
        if let Some(error) = param("error") {
            return Err(Error::Denied(error));
        }
        if param("state").as_deref() != Some(self.state.as_str()) {
            return Err(Error::StateMismatch);
        }
        param("code").ok_or(Error::MissingCode)
    }
}

#[derive(Debug, Clone)]
pub struct OAuth2Client {
    client_id: String,
    client_secret: Option<String>,
    http: reqwest::Client,
}

impl OAuth2Client {
    /// A public client. Add `with_secret` for a confidential one.
    pub fn new(client_id: impl Into<String>) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: None,
            http: reqwest::Client::new(),
        }
    }

    pub fn with_secret(mut self, client_secret: impl Into<String>) -> Self {
        self.client_secret = Some(client_secret.into());
        self
    }

    pub fn with_http_client(mut self, http: reqwest::Client) -> Self {
        self.http = http;
        self
    }

    /// Build the URL to send the user to, with a fresh PKCE verifier and state.
    pub fn authorization_url(
        &self,
        redirect_uri: &str,
        scopes: &[Scope],
    ) -> Result<AuthorizationRequest> {
        let code_verifier = random_token()?;
        let challenge = URL_SAFE_NO_PAD.encode(Sha256::digest(code_verifier.as_bytes()));
        let state = random_token()?;
        let scope = scopes
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join(" ");

        let url = Url::parse_with_params(
            AUTHORIZE_URL,
            &[
                ("response_type", "code"),
                ("client_id", self.client_id.as_str()),
                ("redirect_uri", redirect_uri),
                ("scope", scope.as_str()),
                ("state", state.as_str()),
                ("code_challenge", challenge.as_str()),
                ("code_challenge_method", "S256"),
            ],
        )?;
        Ok(AuthorizationRequest {
            url,
            state,
            code_verifier,
        })
    }

    /// Trade an authorization code for tokens. `redirect_uri` must match the one used for
    /// the authorization URL.
    pub async fn exchange_code(
        &self,
        code: &str,
        redirect_uri: &str,
        code_verifier: &str,
    ) -> Result<Tokens> {
        self.token_request(&[
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", redirect_uri),
            ("code_verifier", code_verifier),
        ])
        .await
    }

    /// Spend a refresh token. The result carries a replacement refresh token.
    pub async fn refresh(&self, refresh_token: &str) -> Result<Tokens> {
        self.token_request(&[
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
        ])
        .await
    }

    async fn token_request(&self, params: &[(&str, &str)]) -> Result<Tokens> {
        let mut form = params.to_vec();
        let mut request = self.http.post(TOKEN_URL);
        match &self.client_secret {
            Some(secret) => request = request.basic_auth(&self.client_id, Some(secret)),
            None => form.push(("client_id", self.client_id.as_str())),
        }
        let response = request.form(&form).send().await?;
        let status = response.status();
        if !status.is_success() {
            return Err(Error::Token {
                status,
                body: response.text().await.unwrap_or_default(),
            });
        }
        Ok(response.json().await?)
    }

    /// Run the whole flow for a CLI: listen on `redirect_uri` (which must be a loopback
    /// URL registered with the app), hand the authorization URL to `on_url` for display,
    /// wait for the browser to come back, and exchange the code.
    #[cfg(feature = "local")]
    pub async fn authorize_local(
        &self,
        redirect_uri: &str,
        scopes: &[Scope],
        on_url: impl FnOnce(&Url),
    ) -> Result<Tokens> {
        let redirect = Url::parse(redirect_uri)?;
        let host = redirect.host_str().ok_or(Error::InvalidRedirect)?;
        let port = redirect
            .port_or_known_default()
            .ok_or(Error::InvalidRedirect)?;

        // Bind first so a busy port fails before the user is sent anywhere.
        let listener = tokio::net::TcpListener::bind((host, port)).await?;
        let request = self.authorization_url(redirect_uri, scopes)?;
        on_url(&request.url);

        let callback = wait_for_callback(&listener, &redirect).await?;
        let code = request.code_from_callback(&callback)?;
        self.exchange_code(&code, redirect_uri, &request.code_verifier)
            .await
    }
}

fn random_token() -> Result<String> {
    let mut bytes = [0u8; 48];
    getrandom::fill(&mut bytes).map_err(Error::Random)?;
    Ok(URL_SAFE_NO_PAD.encode(bytes))
}

/// Wait for the browser to land on the callback and return the full URL it requested.
#[cfg(feature = "local")]
async fn wait_for_callback(listener: &tokio::net::TcpListener, redirect: &Url) -> Result<Url> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    loop {
        let (mut stream, _) = listener.accept().await?;
        let mut buf = vec![0u8; 8192];
        let n = stream.read(&mut buf).await?;
        let request = String::from_utf8_lossy(&buf[..n]);
        // "GET /callback?code=...&state=... HTTP/1.1"
        let target = request.split_whitespace().nth(1).unwrap_or("/");
        let url = redirect.join(target)?;

        // Favicon and other stray requests also reach this port.
        let ours = url.path() == redirect.path()
            && url.query_pairs().any(|(k, _)| k == "code" || k == "error");
        let (status, body) = if ours {
            ("200 OK", "Authorized. You can close this tab.")
        } else {
            ("404 Not Found", "Not found.")
        };
        let response = format!(
            "HTTP/1.1 {status}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );
        stream.write_all(response.as_bytes()).await?;
        if ours {
            return Ok(url);
        }
    }
}
