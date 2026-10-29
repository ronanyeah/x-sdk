//! X API v2 client: the generated API surface from `x-sdk-gen`, plus hand-written helpers.

pub use config::Config;
pub use x_sdk_gen::{apis, models};

pub mod auth;
pub mod config;
mod paginated;
pub mod pagination;
#[cfg(feature = "stream")]
pub mod stream;
#[cfg(feature = "stream")]
mod stream_endpoints;
mod scope;
