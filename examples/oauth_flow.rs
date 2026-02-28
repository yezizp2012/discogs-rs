use discogs_rs::DiscogsOAuthClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Required env vars:
    // - DISCOGS_CONSUMER_KEY
    // - DISCOGS_CONSUMER_SECRET
    // Optional env vars:
    // - DISCOGS_CALLBACK_URL (default: "https://example.com/callback")
    // - DISCOGS_REQUEST_TOKEN, DISCOGS_REQUEST_TOKEN_SECRET, DISCOGS_OAUTH_VERIFIER
    let consumer_key = match env::var("DISCOGS_CONSUMER_KEY") {
        Ok(v) => v,
        Err(_) => {
            eprintln!("missing DISCOGS_CONSUMER_KEY, skipping oauth example");
            return Ok(());
        }
    };
    let consumer_secret = match env::var("DISCOGS_CONSUMER_SECRET") {
        Ok(v) => v,
        Err(_) => {
            eprintln!("missing DISCOGS_CONSUMER_SECRET, skipping oauth example");
            return Ok(());
        }
    };

    let oauth = DiscogsOAuthClient::new(
        consumer_key,
        consumer_secret,
        format!("discogs-rs-oauth-example/{}", env!("CARGO_PKG_VERSION")),
    )?;

    let request_token = env::var("DISCOGS_REQUEST_TOKEN").ok();
    let request_token_secret = env::var("DISCOGS_REQUEST_TOKEN_SECRET").ok();
    let verifier = env::var("DISCOGS_OAUTH_VERIFIER").ok();

    if let (Some(token), Some(token_secret), Some(verifier)) =
        (request_token, request_token_secret, verifier)
    {
        let access = oauth.access_token(&token, &token_secret, &verifier).await?;
        println!("access_token: {}", access.access_token);
        println!("access_token_secret: {}", access.access_token_secret);
        return Ok(());
    }

    let callback_url =
        env::var("DISCOGS_CALLBACK_URL").unwrap_or_else(|_| "https://example.com/callback".into());
    let request = oauth.request_token(&callback_url).await?;

    println!("authorize_url: {}", request.authorize_url);
    println!("request_token: {}", request.token);
    println!("request_token_secret: {}", request.token_secret);
    println!("callback_confirmed: {}", request.callback_confirmed);
    println!(
        "after authorization, rerun with DISCOGS_REQUEST_TOKEN, DISCOGS_REQUEST_TOKEN_SECRET, and DISCOGS_OAUTH_VERIFIER"
    );

    Ok(())
}
