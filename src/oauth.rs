use crate::error::{DiscogsError, Result};
use rand::{Rng, distr::Alphanumeric};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};
use url::form_urlencoded;

const API_BASE: &str = "https://api.discogs.com";

#[derive(Debug, Clone)]
pub struct DiscogsOAuthClient {
    consumer_key: String,
    consumer_secret: String,
    user_agent: String,
    http: reqwest::Client,
}

#[derive(Debug, Clone)]
pub struct RequestToken {
    pub token: String,
    pub token_secret: String,
    pub callback_confirmed: bool,
    pub authorize_url: String,
}

#[derive(Debug, Clone)]
pub struct AccessToken {
    pub access_token: String,
    pub access_token_secret: String,
}

impl DiscogsOAuthClient {
    pub fn new(
        consumer_key: impl Into<String>,
        consumer_secret: impl Into<String>,
        user_agent: impl Into<String>,
    ) -> Result<Self> {
        Ok(Self {
            consumer_key: consumer_key.into(),
            consumer_secret: consumer_secret.into(),
            user_agent: user_agent.into(),
            http: reqwest::Client::builder().build()?,
        })
    }

    pub async fn request_token(&self, callback_url: &str) -> Result<RequestToken> {
        let nonce = oauth_nonce();
        let timestamp = oauth_timestamp_seconds();
        let callback_encoded: String =
            form_urlencoded::byte_serialize(callback_url.as_bytes()).collect();

        let header_value = format!(
            "OAuth oauth_consumer_key=\"{}\", oauth_nonce=\"{}\", oauth_signature=\"{}&\", oauth_signature_method=\"PLAINTEXT\", oauth_timestamp=\"{}\", oauth_callback=\"{}\"",
            self.consumer_key,
            self.nonce_safe(&nonce),
            self.consumer_secret,
            timestamp,
            callback_encoded
        );

        let response = self
            .http
            .get(format!("{API_BASE}/oauth/request_token"))
            .header(USER_AGENT, &self.user_agent)
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .header(AUTHORIZATION, header_value)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let message = response
                .text()
                .await
                .unwrap_or_else(|_| "unknown error".to_string());
            return Err(DiscogsError::Http { status, message });
        }

        let text = response.text().await?;
        // Discogs OAuth endpoints return URL-encoded key/value pairs instead of JSON.
        let values = parse_oauth_form(&text);

        let token = values
            .get("oauth_token")
            .cloned()
            .ok_or_else(|| DiscogsError::InvalidOAuthResponse(text.clone()))?;
        let token_secret = values
            .get("oauth_token_secret")
            .cloned()
            .ok_or_else(|| DiscogsError::InvalidOAuthResponse(text.clone()))?;
        let callback_confirmed = values
            .get("oauth_callback_confirmed")
            .map(|v| v == "true")
            .unwrap_or(false);

        Ok(RequestToken {
            authorize_url: format!("https://discogs.com/oauth/authorize?oauth_token={token}"),
            token,
            token_secret,
            callback_confirmed,
        })
    }

    pub async fn access_token(
        &self,
        request_token: &str,
        request_token_secret: &str,
        verifier: &str,
    ) -> Result<AccessToken> {
        let nonce = oauth_nonce();
        let timestamp = oauth_timestamp_seconds();

        let header_value = format!(
            "OAuth oauth_consumer_key=\"{}\", oauth_nonce=\"{}\", oauth_token=\"{}\", oauth_signature=\"{}&{}\", oauth_signature_method=\"PLAINTEXT\", oauth_timestamp=\"{}\", oauth_verifier=\"{}\"",
            self.consumer_key,
            self.nonce_safe(&nonce),
            request_token,
            self.consumer_secret,
            request_token_secret,
            timestamp,
            verifier
        );

        let response = self
            .http
            .post(format!("{API_BASE}/oauth/access_token"))
            .header(USER_AGENT, &self.user_agent)
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .header(AUTHORIZATION, header_value)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let message = response
                .text()
                .await
                .unwrap_or_else(|_| "unknown error".to_string());
            return Err(DiscogsError::Http { status, message });
        }

        let text = response.text().await?;
        // Access token exchange uses the same URL-encoded payload shape as request token.
        let values = parse_oauth_form(&text);
        let access_token = values
            .get("oauth_token")
            .cloned()
            .ok_or_else(|| DiscogsError::InvalidOAuthResponse(text.clone()))?;
        let access_token_secret = values
            .get("oauth_token_secret")
            .cloned()
            .ok_or_else(|| DiscogsError::InvalidOAuthResponse(text.clone()))?;

        Ok(AccessToken {
            access_token,
            access_token_secret,
        })
    }

    fn nonce_safe(&self, nonce: &str) -> String {
        nonce
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .collect()
    }
}

pub fn build_oauth_header(
    consumer_key: &str,
    consumer_secret: &str,
    access_token: &str,
    access_token_secret: &str,
) -> String {
    let nonce = oauth_nonce();
    let timestamp = oauth_timestamp_seconds();

    format!(
        "OAuth oauth_consumer_key=\"{consumer_key}\", oauth_token=\"{access_token}\", oauth_signature_method=\"PLAINTEXT\", oauth_signature=\"{consumer_secret}&{access_token_secret}\", oauth_timestamp=\"{timestamp}\", oauth_nonce=\"{nonce}\", oauth_token_secret=\"{access_token_secret}\", oauth_version=\"1.0\""
    )
}

fn oauth_nonce() -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect()
}

fn oauth_timestamp_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn parse_oauth_form(raw: &str) -> BTreeMap<String, String> {
    form_urlencoded::parse(raw.as_bytes())
        .into_owned()
        .collect()
}
