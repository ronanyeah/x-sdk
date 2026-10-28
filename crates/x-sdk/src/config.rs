//! A ready-made [`Configuration`] for a token.
//!
//! The generated struct has separate `bearer_access_token` and `oauth_access_token` fields,
//! and each endpoint reads only the one its spec entry names. Both are sent as
//! `Authorization: Bearer`, so [`bearer`] sets both and every endpoint gets the token.

use x_sdk_gen::apis::configuration::Configuration;

/// A `Configuration` that sends `token`, whether it is an app-only bearer token or a
/// user-context OAuth2 access token (e.g. `Tokens::access_token` from [`crate::auth`]).
pub fn bearer(token: impl Into<String>) -> Configuration {
    let token = token.into();
    let mut cfg = Configuration::new();
    cfg.bearer_access_token = Some(token.clone());
    cfg.oauth_access_token = Some(token);
    cfg
}
