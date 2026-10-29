//! Client configuration.
//!
//! The generated `Configuration` has separate `bearer_access_token` and `oauth_access_token`
//! fields, and each endpoint reads only the one its spec entry names. Both are sent as
//! `Authorization: Bearer`, so [`Config::bearer`] sets both. `Config` derefs to
//! `Configuration` without allowing mutation, so the fields cannot get out of step, and `&config`
//! can be passed straight to any generated function.

use std::ops::Deref;

use x_sdk_gen::apis::configuration::Configuration;

#[derive(Debug, Clone)]
pub struct Config(Configuration);

impl Config {
    /// Send `token` on every request, whether it is an app-only bearer token or a user-context
    /// OAuth2 access token (e.g. `Tokens::access_token` from [`crate::auth`]).
    pub fn bearer(token: impl Into<String>) -> Self {
        let token = token.into();
        let mut cfg = Configuration::new();
        cfg.bearer_access_token = Some(token.clone());
        cfg.oauth_access_token = Some(token);
        cfg.user_agent = Some(concat!("x-sdk/", env!("CARGO_PKG_VERSION")).to_owned());
        Self(cfg)
    }

    /// Use a different API host, e.g. a local mock server.
    pub fn base_path(mut self, base_path: impl Into<String>) -> Self {
        self.0.base_path = base_path.into();
        self
    }

    /// Replace the default `x-sdk/<version>` user agent.
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.0.user_agent = Some(user_agent.into());
        self
    }

    /// Use your own `reqwest::Client`, for proxies, timeouts and so on.
    pub fn client(mut self, client: reqwest::Client) -> Self {
        self.0.client = client;
        self
    }
}

impl Deref for Config {
    type Target = Configuration;

    fn deref(&self) -> &Configuration {
        &self.0
    }
}

impl From<Config> for Configuration {
    fn from(config: Config) -> Self {
        config.0
    }
}
