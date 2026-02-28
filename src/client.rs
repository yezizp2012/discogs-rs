use crate::auth::{Auth, AuthLevel, OutputFormat};
use crate::endpoints::{
    collection::CollectionApi, database::DatabaseApi, inventory::InventoryApi,
    marketplace::MarketplaceApi, user::UserApi, user_list::ListApi, wantlist::WantlistApi,
};
use crate::error::{DiscogsError, Result};
use crate::models::{AboutResponse, ApiResponse, Identity, RateLimit};
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use reqwest::{Method, Response, StatusCode};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub base_delay: Duration,
    pub backoff_factor: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 0,
            base_delay: Duration::from_millis(2_000),
            backoff_factor: 2.7,
        }
    }
}

#[derive(Debug, Clone)]
struct ClientConfig {
    base_url: String,
    user_agent: String,
    output_format: OutputFormat,
    auth: Auth,
    retry: RetryConfig,
}

#[derive(Clone)]
pub struct DiscogsClient {
    config: Arc<ClientConfig>,
    http: reqwest::Client,
}

pub struct DiscogsClientBuilder {
    config: ClientConfig,
    timeout: Duration,
}

impl DiscogsClientBuilder {
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.config.base_url = base_url.into();
        self
    }

    pub fn auth(mut self, auth: Auth) -> Self {
        self.config.auth = auth;
        self
    }

    pub fn user_token(mut self, token: impl Into<String>) -> Self {
        self.config.auth = Auth::UserToken {
            token: token.into(),
        };
        self
    }

    pub fn output_format(mut self, output_format: OutputFormat) -> Self {
        self.config.output_format = output_format;
        self
    }

    pub fn retry(mut self, retry: RetryConfig) -> Self {
        self.config.retry = retry;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn build(self) -> Result<DiscogsClient> {
        let http = reqwest::Client::builder().timeout(self.timeout).build()?;
        Ok(DiscogsClient {
            config: Arc::new(self.config),
            http,
        })
    }
}

impl DiscogsClient {
    pub fn builder(user_agent: impl Into<String>) -> DiscogsClientBuilder {
        DiscogsClientBuilder {
            config: ClientConfig {
                base_url: "https://api.discogs.com".to_string(),
                user_agent: user_agent.into(),
                output_format: OutputFormat::Discogs,
                auth: Auth::None,
                retry: RetryConfig::default(),
            },
            timeout: Duration::from_secs(30),
        }
    }

    pub fn with_default_user_agent() -> DiscogsClientBuilder {
        let ua = format!(
            "discogs-rs/{} +https://github.com/your-org/discogs-rs",
            env!("CARGO_PKG_VERSION")
        );
        Self::builder(ua)
    }

    pub fn with_user_token(
        user_agent: impl Into<String>,
        token: impl Into<String>,
    ) -> Result<DiscogsClient> {
        Self::builder(user_agent).user_token(token).build()
    }

    pub fn with_default_user_agent_and_user_token(
        token: impl Into<String>,
    ) -> Result<DiscogsClient> {
        Self::with_default_user_agent().user_token(token).build()
    }

    pub fn auth_level(&self) -> AuthLevel {
        self.config.auth.level()
    }

    pub async fn about(&self) -> Result<ApiResponse<AboutResponse>> {
        self.request_json::<AboutResponse, (), ()>(Method::GET, "/", None, None, AuthLevel::None)
            .await
    }

    pub async fn get_identity(&self) -> Result<ApiResponse<Identity>> {
        self.request_json::<Identity, (), ()>(
            Method::GET,
            "/oauth/identity",
            None,
            None,
            AuthLevel::User,
        )
        .await
    }

    pub fn database(&self) -> DatabaseApi<'_> {
        DatabaseApi::new(self)
    }

    pub fn marketplace(&self) -> MarketplaceApi<'_> {
        MarketplaceApi::new(self)
    }

    pub fn inventory(&self) -> InventoryApi<'_> {
        InventoryApi::new(self)
    }

    pub fn user(&self) -> UserApi<'_> {
        UserApi::new(self)
    }

    pub fn collection(&self) -> CollectionApi<'_> {
        CollectionApi::new(self)
    }

    pub fn wantlist(&self) -> WantlistApi<'_> {
        WantlistApi::new(self)
    }

    pub fn list(&self) -> ListApi<'_> {
        ListApi::new(self)
    }

    pub(crate) async fn request_json<T, Q, B>(
        &self,
        method: Method,
        path: &str,
        query: Option<&Q>,
        body: Option<&B>,
        required_auth: AuthLevel,
    ) -> Result<ApiResponse<T>>
    where
        T: DeserializeOwned,
        Q: Serialize + ?Sized,
        B: Serialize + ?Sized,
    {
        let response = self
            .send_with_retry(method, path, query, body, required_auth)
            .await?;

        let rate_limit = parse_rate_limit(&response);
        let data = response.json::<T>().await?;
        Ok(ApiResponse { data, rate_limit })
    }

    pub(crate) async fn request_empty<Q, B>(
        &self,
        method: Method,
        path: &str,
        query: Option<&Q>,
        body: Option<&B>,
        required_auth: AuthLevel,
    ) -> Result<ApiResponse<()>>
    where
        Q: Serialize + ?Sized,
        B: Serialize + ?Sized,
    {
        let response = self
            .send_with_retry(method, path, query, body, required_auth)
            .await?;

        let rate_limit = parse_rate_limit(&response);
        Ok(ApiResponse {
            data: (),
            rate_limit,
        })
    }

    pub(crate) async fn request_bytes<Q, B>(
        &self,
        method: Method,
        path: &str,
        query: Option<&Q>,
        body: Option<&B>,
        required_auth: AuthLevel,
    ) -> Result<ApiResponse<bytes::Bytes>>
    where
        Q: Serialize + ?Sized,
        B: Serialize + ?Sized,
    {
        let response = self
            .send_with_retry(method, path, query, body, required_auth)
            .await?;

        let rate_limit = parse_rate_limit(&response);
        let data = response.bytes().await?;
        Ok(ApiResponse { data, rate_limit })
    }

    async fn send_with_retry<Q, B>(
        &self,
        method: Method,
        path: &str,
        query: Option<&Q>,
        body: Option<&B>,
        required_auth: AuthLevel,
    ) -> Result<Response>
    where
        Q: Serialize + ?Sized,
        B: Serialize + ?Sized,
    {
        // Enforce auth level before sending any request to avoid unnecessary network round trips.
        self.ensure_auth(required_auth)?;

        let mut attempt: u32 = 0;
        loop {
            let mut request = self
                .http
                .request(method.clone(), self.absolute_url(path))
                .header(USER_AGENT, &self.config.user_agent)
                .header(ACCEPT, self.config.output_format.accept_header_value());

            if let Some(auth_header) = self.config.auth.authorization_header() {
                request = request.header(AUTHORIZATION, auth_header);
            }

            if let Some(query) = query {
                request = request.query(query);
            }
            if let Some(body) = body {
                request = request.json(body);
            }

            let response = request.send().await?;
            let status = response.status();

            // Discogs can return 429 under burst traffic; retry with configurable exponential backoff.
            if status == StatusCode::TOO_MANY_REQUESTS && attempt < self.config.retry.max_retries {
                let delay = retry_delay(&self.config.retry, attempt);
                tokio::time::sleep(delay).await;
                attempt += 1;
                continue;
            }

            if status.is_success() {
                return Ok(response);
            }

            return Err(http_error(response).await);
        }
    }

    fn ensure_auth(&self, required: AuthLevel) -> Result<()> {
        let current = self.config.auth.level();
        if current < required {
            return Err(DiscogsError::AuthRequired { required, current });
        }
        Ok(())
    }

    fn absolute_url(&self, path: &str) -> String {
        if path.starts_with("http://") || path.starts_with("https://") {
            return path.to_string();
        }

        let base = self.config.base_url.trim_end_matches('/');
        let path = if path.starts_with('/') {
            path.to_string()
        } else {
            format!("/{path}")
        };
        format!("{base}{path}")
    }
}

fn retry_delay(config: &RetryConfig, attempt: u32) -> Duration {
    let base_ms = config.base_delay.as_millis() as f64;
    let factor = config.backoff_factor.powi(attempt as i32);
    let delay_ms = (base_ms * factor).round() as u64;
    Duration::from_millis(delay_ms.max(1))
}

fn parse_rate_limit(response: &Response) -> Option<RateLimit> {
    let headers = response.headers();
    // Rate-limit headers may be absent on some responses, so parsing is intentionally optional.
    let limit = headers
        .get("x-discogs-ratelimit")?
        .to_str()
        .ok()?
        .parse()
        .ok()?;
    let used = headers
        .get("x-discogs-ratelimit-used")?
        .to_str()
        .ok()?
        .parse()
        .ok()?;
    let remaining = headers
        .get("x-discogs-ratelimit-remaining")?
        .to_str()
        .ok()?
        .parse()
        .ok()?;

    Some(RateLimit {
        limit,
        used,
        remaining,
    })
}

async fn http_error(response: Response) -> DiscogsError {
    let status = response.status();
    let message = match response.json::<serde_json::Value>().await {
        Ok(json) => json
            .get("message")
            .and_then(|value| value.as_str())
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| json.to_string()),
        Err(_) => "unknown error".to_string(),
    };

    DiscogsError::Http { status, message }
}

#[cfg(test)]
mod tests {
    use super::{DiscogsClient, RetryConfig, retry_delay};
    use crate::auth::AuthLevel;
    use std::time::Duration;

    #[test]
    fn retry_delay_grows_exponentially() {
        let config = RetryConfig {
            max_retries: 3,
            base_delay: Duration::from_millis(100),
            backoff_factor: 2.0,
        };

        assert_eq!(retry_delay(&config, 0), Duration::from_millis(100));
        assert_eq!(retry_delay(&config, 1), Duration::from_millis(200));
        assert_eq!(retry_delay(&config, 2), Duration::from_millis(400));
    }

    #[test]
    fn builder_user_token_sets_user_auth_level() {
        let client = DiscogsClient::builder("test-agent")
            .user_token("user-token")
            .build()
            .expect("build client");

        assert_eq!(client.auth_level(), AuthLevel::User);
    }

    #[test]
    fn convenience_user_token_constructor_sets_user_auth_level() {
        let client = DiscogsClient::with_user_token("test-agent", "user-token")
            .expect("build client with user token");

        assert_eq!(client.auth_level(), AuthLevel::User);
    }
}
