use discogs_rs::{
    Auth, CreateListingRequest, DiscogsClient, MediaCondition, NumberOrAuto, SaleStatus,
    SleeveCondition,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Optional env vars:
    // - DISCOGS_RELEASE_ID (default: 249504)
    // - DISCOGS_CREATE_LISTING=1 (if set, submit request to API)
    // - DISCOGS_USER_TOKEN (required only when submitting)
    let release_id = env::var("DISCOGS_RELEASE_ID")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(249_504);

    let request = CreateListingRequest {
        release_id,
        condition: MediaCondition::VeryGoodPlus,
        sleeve_condition: Some(SleeveCondition::VeryGoodPlus),
        price: 19.99,
        comments: Some("Clean copy, play-tested.".to_string()),
        allow_offers: Some(true),
        status: SaleStatus::Draft,
        external_id: Some("example-inv-001".to_string()),
        location: Some("A1".to_string()),
        weight: Some(NumberOrAuto::Auto("auto".to_string())),
        format_quantity: Some(NumberOrAuto::Number(1)),
    };

    println!(
        "typed listing payload:\n{}",
        serde_json::to_string_pretty(&request)?
    );

    let should_submit = env::var("DISCOGS_CREATE_LISTING")
        .map(|v| v == "1")
        .unwrap_or(false);
    if !should_submit {
        println!("set DISCOGS_CREATE_LISTING=1 to call marketplace.add_listing");
        return Ok(());
    }

    let token = match env::var("DISCOGS_USER_TOKEN") {
        Ok(v) => v,
        Err(_) => {
            eprintln!("missing DISCOGS_USER_TOKEN, cannot submit listing");
            return Ok(());
        }
    };

    let client = DiscogsClient::with_default_user_agent()
        .auth(Auth::UserToken { token })
        .build()?;

    let created = client.marketplace().add_listing(&request).await?;
    println!("created listing id: {}", created.data.listing_id);

    Ok(())
}
