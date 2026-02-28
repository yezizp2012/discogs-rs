# Changelog

All notable changes to `discogs-rs` are documented in this file.

## [0.1.0] - 2026-02-28

### Initial Release

- Added async Rust client foundations: `DiscogsClient`, `DiscogsClientBuilder`, and a unified request pipeline.
- Added authentication modes:
  - no auth
  - `UserToken`
  - Discogs application credentials
  - full OAuth access credentials
- Added API domain modules and entry points:
  - `database()`
  - `marketplace()`
  - `inventory()`
  - `user()`
  - `collection()`
  - `wantlist()`
  - `list()`
- Implemented Discogs API v2 core coverage:
  - Core: `about`, `get_identity`
  - Database: artist, release, master, label, search, and rating endpoints
  - Marketplace: listing, order, fee, pricing suggestion, and stats endpoints
  - Inventory Export: export creation, status query, and download
  - User: profile, inventory, identity, contributions, submissions, lists
  - Collection / Wantlist / List: query and mutation endpoints
- Added OAuth 1.0a helper client:
  - `DiscogsOAuthClient::request_token`
  - `DiscogsOAuthClient::access_token`
- Added unified response envelope: `ApiResponse<T>` with parsed `rate_limit` metadata.
- Added configurable exponential backoff for `429 Too Many Requests` via `RetryConfig`.
- Added unified error model: `DiscogsError`.
- Added crate metadata baseline:
  - crate name: `discogs-rs`
  - version: `0.1.0`
  - license: `MIT`
  - Rust edition: `2024`

### Compatibility Notes

- This version is the first usable public baseline.
- Model strategy uses strong typing for common fields and `flatten extra` for forward compatibility.
