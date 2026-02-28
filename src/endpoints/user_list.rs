//! Discogs User List API endpoints.
//!
//! Reference: <https://www.discogs.com/developers/#page:user-lists>

use crate::auth::AuthLevel;
use crate::client::DiscogsClient;
use crate::error::Result;
use crate::models::{ApiResponse, DiscogsList};
use reqwest::Method;

pub struct ListApi<'a> {
    client: &'a DiscogsClient,
}

impl<'a> ListApi<'a> {
    pub(crate) fn new(client: &'a DiscogsClient) -> Self {
        Self { client }
    }

    pub async fn get_items(&self, list_id: u64) -> Result<ApiResponse<DiscogsList>> {
        self.client
            .request_json::<DiscogsList, (), ()>(
                Method::GET,
                &format!("/lists/{list_id}"),
                None,
                None,
                AuthLevel::None,
            )
            .await
    }
}
