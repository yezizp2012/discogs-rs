use crate::auth::AuthLevel;
use crate::client::DiscogsClient;
use crate::error::Result;
use crate::models::{
    ApiResponse, PaginationParams, WantlistEntries, WantlistEntry, WantlistUpdateRequest,
};
use reqwest::Method;

pub struct WantlistApi<'a> {
    client: &'a DiscogsClient,
}

impl<'a> WantlistApi<'a> {
    pub(crate) fn new(client: &'a DiscogsClient) -> Self {
        Self { client }
    }

    pub async fn get_releases(
        &self,
        username: &str,
        query: Option<&PaginationParams>,
    ) -> Result<ApiResponse<WantlistEntries>> {
        self.client
            .request_json::<WantlistEntries, PaginationParams, ()>(
                Method::GET,
                &format!("/users/{}/wants", crate::endpoints::encode_path(username)),
                query,
                None,
                AuthLevel::None,
            )
            .await
    }

    pub async fn add_release(
        &self,
        username: &str,
        release_id: u64,
        request: &WantlistUpdateRequest,
    ) -> Result<ApiResponse<WantlistEntry>> {
        self.client
            .request_json::<WantlistEntry, (), WantlistUpdateRequest>(
                Method::PUT,
                &format!(
                    "/users/{}/wants/{release_id}",
                    crate::endpoints::encode_path(username)
                ),
                None,
                Some(request),
                AuthLevel::User,
            )
            .await
    }

    pub async fn edit_notes(
        &self,
        username: &str,
        release_id: u64,
        request: &WantlistUpdateRequest,
    ) -> Result<ApiResponse<WantlistEntry>> {
        self.client
            .request_json::<WantlistEntry, (), WantlistUpdateRequest>(
                Method::POST,
                &format!(
                    "/users/{}/wants/{release_id}",
                    crate::endpoints::encode_path(username)
                ),
                None,
                Some(request),
                AuthLevel::User,
            )
            .await
    }

    pub async fn remove_release(&self, username: &str, release_id: u64) -> Result<ApiResponse<()>> {
        self.client
            .request_empty::<(), ()>(
                Method::DELETE,
                &format!(
                    "/users/{}/wants/{release_id}",
                    crate::endpoints::encode_path(username)
                ),
                None,
                None,
                AuthLevel::User,
            )
            .await
    }
}
