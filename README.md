# discogs-rs

[![CI](https://github.com/yezizp2012/discogs-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/yezizp2012/discogs-rs/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/discogs-rs.svg)](https://crates.io/crates/discogs-rs)
[![docs.rs](https://img.shields.io/docsrs/discogs-rs)](https://docs.rs/crate/discogs-rs/latest)
[![license](https://img.shields.io/github/license/yezizp2012/discogs-rs)](https://github.com/yezizp2012/discogs-rs/blob/master/LICENSE)

`discogs-rs` is an async Rust client library for Discogs API v2.

## Official Discogs API v2 Links

- API Home: <https://www.discogs.com/developers/>
- Authentication: <https://www.discogs.com/developers/#page:authentication>
- Database: <https://www.discogs.com/developers/#page:database>
- Marketplace: <https://www.discogs.com/developers/#page:marketplace>
- Inventory Export: <https://www.discogs.com/developers/#page:inventory-export>
- User Identity: <https://www.discogs.com/developers/#page:user-identity>
- User Collection: <https://www.discogs.com/developers/#page:user-collection>
- User Wantlist: <https://www.discogs.com/developers/#page:user-wantlist>
- User Lists: <https://www.discogs.com/developers/#page:user-lists>

## Features

- Domain-oriented API modules (`database`, `marketplace`, `inventory`, `user`, `collection`, `wantlist`, `list`)
- Strongly typed request/response models for common fields
- Forward compatibility via `flatten extra` for unknown fields
- OAuth helper for request/access token flow
- Parsed rate-limit metadata from Discogs response headers
- Configurable exponential backoff for `429 Too Many Requests`

## Installation

```toml
[dependencies]
discogs-rs = "0.1"
```

MSRV: `1.85`

## Feature Flags

- `rustls-tls` (default): use `reqwest` with Rustls TLS backend
- `native-tls`: use platform native TLS backend

Example:

```toml
[dependencies]
discogs-rs = { version = "0.1", default-features = false, features = ["native-tls"] }
```

## Quick Start

```rust
use discogs_rs::{Auth, DiscogsClient};

#[tokio::main]
async fn main() -> Result<(), discogs_rs::DiscogsError> {
    let client = DiscogsClient::with_default_user_agent()
        .auth(Auth::UserToken {
            token: std::env::var("DISCOGS_USER_TOKEN").unwrap(),
        })
        .build()?;

    let release = client.database().get_release(249504, None).await?;
    println!("{}", release.data.title);

    Ok(())
}
```

## Typed API Example

```rust
use discogs_rs::{DiscogsClient, PaginationParams, SearchQuery, SearchType};

let client = DiscogsClient::with_default_user_agent().build()?;

let query = SearchQuery {
    query: Some("nirvana".into()),
    query_type: Some(SearchType::Release),
    pagination: PaginationParams {
        page: Some(1),
        per_page: Some(25),
    },
    ..Default::default()
};

let result = client.database().search(Some(&query)).await?;
println!("results: {}", result.data.results.len());
```

## Authentication Modes

- `Auth::None`
- `Auth::UserToken { token }`
- `Auth::Discogs { consumer_key, consumer_secret }`
- `Auth::OAuth { consumer_key, consumer_secret, access_token, access_token_secret }`

For single-account usage, you can use the dedicated user-token constructors:

```rust
use discogs_rs::DiscogsClient;

let client = DiscogsClient::with_default_user_agent_and_user_token(
    std::env::var("DISCOGS_USER_TOKEN")?
)?;
```

## Auth Level Matrix

This client enforces auth level before dispatching HTTP requests.

- `AuthLevel::None`
  - Public endpoints (most database reads, public user/list data)
- `AuthLevel::Consumer`
  - Consumer-key gated endpoints (for example database search)
- `AuthLevel::User`
  - User-scoped endpoints (identity, collection writes, wantlist writes, marketplace order/listing writes, inventory export)

## API Coverage

### Core

- `DiscogsClient::about`
- `DiscogsClient::get_identity`

### Database API (`client.database()`)

- `get_artist`
- `get_artist_releases`
- `get_release`
- `get_release_rating`
- `set_release_rating`
- `delete_release_rating`
- `get_release_community_rating`
- `get_release_stats`
- `get_master`
- `get_master_versions`
- `get_label`
- `get_label_releases`
- `search`

### Marketplace API (`client.marketplace()`)

- `get_listing`
- `add_listing`
- `edit_listing`
- `delete_listing`
- `get_orders`
- `get_order`
- `edit_order`
- `get_order_messages`
- `add_order_message`
- `get_fee`
- `get_price_suggestions`
- `get_release_stats`

### Inventory Export API (`client.inventory()`)

- `export_inventory`
- `get_exports`
- `get_export`
- `download_export`

### User API (`client.user()`)

- `get_profile`
- `edit_profile`
- `get_inventory`
- `get_identity`
- `get_contributions`
- `get_submissions`
- `get_lists`
- `collection`
- `wantlist`
- `list`

### Collection API (`client.user().collection()` / `client.collection()`)

- `get_folders`
- `get_folder`
- `add_folder`
- `set_folder_name`
- `delete_folder`
- `get_releases`
- `get_release_instances`
- `add_release`
- `edit_release`
- `remove_release`
- `get_fields`
- `edit_instance_note`
- `get_value`

### Wantlist API (`client.user().wantlist()` / `client.wantlist()`)

- `get_releases`
- `add_release`
- `edit_notes`
- `remove_release`

### List API (`client.user().list()` / `client.list()`)

- `get_items`

### OAuth Helper (`DiscogsOAuthClient`)

- `request_token`
- `access_token`

## Examples

```bash
cargo run --example search
cargo run --example marketplace_listing_request
cargo run --example oauth_flow
```

## Rate Limit and Retry

Each call returns `ApiResponse<T>`, including optional `rate_limit` parsed from:

- `x-discogs-ratelimit`
- `x-discogs-ratelimit-used`
- `x-discogs-ratelimit-remaining`

Configure retry behavior with `RetryConfig`.

## Discogs API Constraints

- Discogs requires a descriptive `User-Agent`.
- Rate limits are exposed via response headers and can vary by auth mode.
- This crate retries only `429 Too Many Requests` using exponential backoff; all other non-2xx responses are mapped to `DiscogsError::Http`.
- Auth errors are prevented early when possible by local auth-level checks.

## Errors

`DiscogsError` variants:

- `AuthRequired`
- `Http`
- `Request`
- `Json`
- `InvalidOAuthResponse`

## API Compatibility Policy

- Common/high-value fields are strongly typed.
- Unknown fields are preserved through `#[serde(flatten)] extra` maps.
- This strategy minimizes breakage from additive API response changes while keeping ergonomic typed access.

## Development

```bash
cargo fmt
cargo check
cargo test
```
