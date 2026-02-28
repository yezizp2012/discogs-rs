use reqwest::StatusCode;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, DiscogsError>;

#[derive(Debug, Error)]
pub enum DiscogsError {
    #[error("authentication level {required:?} required, current level is {current:?}")]
    AuthRequired {
        required: crate::auth::AuthLevel,
        current: crate::auth::AuthLevel,
    },

    #[error("http error {status}: {message}")]
    Http { status: StatusCode, message: String },

    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("json parse failed: {0}")]
    Json(#[from] serde_json::Error),

    #[error("invalid OAuth response: {0}")]
    InvalidOAuthResponse(String),
}
