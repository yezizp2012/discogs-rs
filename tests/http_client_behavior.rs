use axum::extract::Request;
use axum::http::{HeaderValue, StatusCode, header::AUTHORIZATION};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use discogs_rs::{AuthLevel, DiscogsClient, DiscogsError, RetryConfig};
use serde_json::json;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use std::time::Duration;
use tokio::net::TcpListener;

async fn spawn_server(app: Router) -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind test server");
    let addr = listener.local_addr().expect("read test server addr");

    tokio::spawn(async move {
        axum::serve(listener, app).await.expect("run test server");
    });

    format!("http://{addr}")
}

#[tokio::test]
async fn retries_once_after_429_and_returns_success_payload() {
    let hits = Arc::new(AtomicUsize::new(0));
    let app = Router::new().route(
        "/",
        get({
            let hits = Arc::clone(&hits);
            move || {
                let hits = Arc::clone(&hits);
                async move {
                    let current = hits.fetch_add(1, Ordering::SeqCst);
                    if current == 0 {
                        return (
                            StatusCode::TOO_MANY_REQUESTS,
                            Json(json!({ "message": "rate limited" })),
                        )
                            .into_response();
                    }

                    let mut response = (
                        StatusCode::OK,
                        Json(json!({
                            "hello": "Welcome to the Discogs API.",
                            "api_version": "v2",
                            "documentation_url": "https://www.discogs.com/developers/",
                            "statistics": {
                                "releases": 1,
                                "artists": 2,
                                "labels": 3
                            }
                        })),
                    )
                        .into_response();

                    response
                        .headers_mut()
                        .insert("x-discogs-ratelimit", HeaderValue::from_static("60"));
                    response
                        .headers_mut()
                        .insert("x-discogs-ratelimit-used", HeaderValue::from_static("1"));
                    response.headers_mut().insert(
                        "x-discogs-ratelimit-remaining",
                        HeaderValue::from_static("59"),
                    );
                    response
                }
            }
        }),
    );

    let base_url = spawn_server(app).await;
    let client = DiscogsClient::builder("test-agent")
        .base_url(base_url)
        .retry(RetryConfig {
            max_retries: 1,
            base_delay: Duration::from_millis(1),
            backoff_factor: 1.0,
        })
        .build()
        .expect("build client");

    let response = client.about().await.expect("about response");
    assert_eq!(response.data.hello, "Welcome to the Discogs API.");
    assert_eq!(hits.load(Ordering::SeqCst), 2);

    let rate = response.rate_limit.expect("rate limit present");
    assert_eq!(rate.limit, 60);
    assert_eq!(rate.used, 1);
    assert_eq!(rate.remaining, 59);
}

#[tokio::test]
async fn user_token_is_forwarded_as_authorization_header() {
    async fn require_user_token(request: Request, next: Next) -> Response {
        let valid = request
            .headers()
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .map(|value| value == "Discogs token=test-token")
            .unwrap_or(false);

        if !valid {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "message": "missing or invalid token" })),
            )
                .into_response();
        }

        next.run(request).await
    }

    let app = Router::new()
        .route(
            "/oauth/identity",
            get(|| async {
                (
                    StatusCode::OK,
                    Json(json!({
                        "id": 1,
                        "username": "tester",
                        "resource_url": "https://api.discogs.com/users/tester",
                        "consumer_name": "discogs-rs"
                    })),
                )
            }),
        )
        .layer(middleware::from_fn(require_user_token));

    let base_url = spawn_server(app).await;
    let client = DiscogsClient::builder("test-agent")
        .base_url(base_url)
        .user_token("test-token")
        .build()
        .expect("build client");

    let response = client.get_identity().await.expect("identity response");
    assert_eq!(response.data.username, "tester");
}

#[tokio::test]
async fn get_identity_without_user_auth_fails_before_network_call() {
    let client = DiscogsClient::builder("test-agent")
        .base_url("http://127.0.0.1:1")
        .build()
        .expect("build client");

    let error = client.get_identity().await.expect_err("auth should fail");
    match error {
        DiscogsError::AuthRequired { required, current } => {
            assert_eq!(required, AuthLevel::User);
            assert_eq!(current, AuthLevel::None);
        }
        other => panic!("expected AuthRequired, got {other:?}"),
    }
}

#[tokio::test]
async fn non_success_response_maps_to_http_error_with_message() {
    let app = Router::new().route(
        "/",
        get(|| async {
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "message": "not found" })),
            )
        }),
    );

    let base_url = spawn_server(app).await;
    let client = DiscogsClient::builder("test-agent")
        .base_url(base_url)
        .build()
        .expect("build client");

    let error = client.about().await.expect_err("request should fail");
    match error {
        DiscogsError::Http { status, message } => {
            assert_eq!(status, StatusCode::NOT_FOUND);
            assert!(message.contains("not found"));
        }
        other => panic!("expected Http error, got {other:?}"),
    }
}
