use crate::auth::AuthLevel;
use crate::client::DiscogsClient;
use crate::endpoints::{collection::CollectionApi, user_list::ListApi, wantlist::WantlistApi};
use crate::error::Result;
use crate::models::{
    ApiResponse, EditProfileRequest, Identity, PaginationParams, UserContributions,
    UserContributionsQuery, UserInventory, UserInventoryQuery, UserLists, UserProfile,
    UserSubmissions,
};
use reqwest::Method;

pub struct UserApi<'a> {
    client: &'a DiscogsClient,
}

impl<'a> UserApi<'a> {
    pub(crate) fn new(client: &'a DiscogsClient) -> Self {
        Self { client }
    }

    pub async fn get_profile(&self, username: &str) -> Result<ApiResponse<UserProfile>> {
        self.client
            .request_json::<UserProfile, (), ()>(
                Method::GET,
                &format!("/users/{}", crate::endpoints::encode_path(username)),
                None,
                None,
                AuthLevel::None,
            )
            .await
    }

    pub async fn edit_profile(
        &self,
        username: &str,
        request: &EditProfileRequest,
    ) -> Result<ApiResponse<UserProfile>> {
        self.client
            .request_json::<UserProfile, (), EditProfileRequest>(
                Method::POST,
                &format!("/users/{}", crate::endpoints::encode_path(username)),
                None,
                Some(request),
                AuthLevel::User,
            )
            .await
    }

    pub async fn get_inventory(
        &self,
        username: &str,
        query: Option<&UserInventoryQuery>,
    ) -> Result<ApiResponse<UserInventory>> {
        self.client
            .request_json::<UserInventory, UserInventoryQuery, ()>(
                Method::GET,
                &format!(
                    "/users/{}/inventory",
                    crate::endpoints::encode_path(username)
                ),
                query,
                None,
                AuthLevel::None,
            )
            .await
    }

    pub async fn get_identity(&self) -> Result<ApiResponse<Identity>> {
        self.client.get_identity().await
    }

    pub fn collection(&self) -> CollectionApi<'a> {
        CollectionApi::new(self.client)
    }

    pub fn wantlist(&self) -> WantlistApi<'a> {
        WantlistApi::new(self.client)
    }

    pub fn list(&self) -> ListApi<'a> {
        ListApi::new(self.client)
    }

    pub async fn get_contributions(
        &self,
        username: &str,
        query: Option<&UserContributionsQuery>,
    ) -> Result<ApiResponse<UserContributions>> {
        self.client
            .request_json::<UserContributions, UserContributionsQuery, ()>(
                Method::GET,
                &format!(
                    "/users/{}/contributions",
                    crate::endpoints::encode_path(username)
                ),
                query,
                None,
                AuthLevel::None,
            )
            .await
    }

    pub async fn get_submissions(
        &self,
        username: &str,
        query: Option<&PaginationParams>,
    ) -> Result<ApiResponse<UserSubmissions>> {
        self.client
            .request_json::<UserSubmissions, PaginationParams, ()>(
                Method::GET,
                &format!(
                    "/users/{}/submissions",
                    crate::endpoints::encode_path(username)
                ),
                query,
                None,
                AuthLevel::None,
            )
            .await
    }

    pub async fn get_lists(
        &self,
        username: &str,
        query: Option<&PaginationParams>,
    ) -> Result<ApiResponse<UserLists>> {
        self.client
            .request_json::<UserLists, PaginationParams, ()>(
                Method::GET,
                &format!("/users/{}/lists", crate::endpoints::encode_path(username)),
                query,
                None,
                AuthLevel::None,
            )
            .await
    }
}
