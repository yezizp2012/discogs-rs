//! Endpoint modules grouped by Discogs API domain.

pub mod collection;
pub mod database;
pub mod inventory;
pub mod marketplace;
pub mod user;
pub mod user_list;
pub mod wantlist;

pub(crate) fn encode_path(value: impl AsRef<str>) -> String {
    url::form_urlencoded::byte_serialize(value.as_ref().as_bytes()).collect()
}
