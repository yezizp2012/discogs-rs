//! Async Rust client for Discogs API v2.
//!
//! Official docs: <https://www.discogs.com/developers/>
//!
//! Main API entry points:
//! - [`DiscogsClient::database`]
//! - [`DiscogsClient::marketplace`]
//! - [`DiscogsClient::inventory`]
//! - [`DiscogsClient::user`]
//! - [`DiscogsClient::collection`]
//! - [`DiscogsClient::wantlist`]
//! - [`DiscogsClient::list`]
//!
//! Authentication reference:
//! - <https://www.discogs.com/developers/#page:authentication>

mod auth;
mod client;
mod endpoints;
mod error;
mod models;
mod oauth;

pub use auth::{Auth, AuthLevel, OutputFormat};
pub use client::{DiscogsClient, DiscogsClientBuilder, RetryConfig};
pub use error::{DiscogsError, Result};
pub use models::*;
pub use oauth::{AccessToken, DiscogsOAuthClient, RequestToken};
