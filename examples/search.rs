use discogs_rs::{Auth, DiscogsClient, PaginationParams, SearchQuery, SearchType};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Required env vars:
    // - DISCOGS_CONSUMER_KEY
    // - DISCOGS_CONSUMER_SECRET
    // Optional env vars:
    // - DISCOGS_SEARCH_QUERY (default: "nirvana")
    let consumer_key = match env::var("DISCOGS_CONSUMER_KEY") {
        Ok(v) => v,
        Err(_) => {
            eprintln!("missing DISCOGS_CONSUMER_KEY, skipping search example");
            return Ok(());
        }
    };
    let consumer_secret = match env::var("DISCOGS_CONSUMER_SECRET") {
        Ok(v) => v,
        Err(_) => {
            eprintln!("missing DISCOGS_CONSUMER_SECRET, skipping search example");
            return Ok(());
        }
    };

    let query_text = env::var("DISCOGS_SEARCH_QUERY").unwrap_or_else(|_| "nirvana".to_string());

    let client = DiscogsClient::with_default_user_agent()
        .auth(Auth::Discogs {
            consumer_key,
            consumer_secret,
        })
        .build()?;

    let query = SearchQuery {
        query: Some(query_text),
        query_type: Some(SearchType::Release),
        pagination: PaginationParams {
            page: Some(1),
            per_page: Some(5),
        },
        ..Default::default()
    };

    let response = client.database().search(Some(&query)).await?;
    println!("results: {}", response.data.results.len());
    for item in response.data.results.iter().take(5) {
        println!("{} | {}", item.id, item.title);
    }

    Ok(())
}
