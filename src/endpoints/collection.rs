use crate::auth::AuthLevel;
use crate::client::DiscogsClient;
use crate::error::Result;
use crate::models::{
    AddCollectionReleaseResponse, AddFolderRequest, ApiResponse, CollectionFields,
    CollectionFolder, CollectionFolders, CollectionReleases, CollectionReleasesQuery,
    CollectionValue, EditCollectionReleaseRequest, EditInstanceNoteRequest,
};
use reqwest::Method;

pub struct CollectionApi<'a> {
    client: &'a DiscogsClient,
}

impl<'a> CollectionApi<'a> {
    pub(crate) fn new(client: &'a DiscogsClient) -> Self {
        Self { client }
    }

    pub async fn get_folders(&self, username: &str) -> Result<ApiResponse<CollectionFolders>> {
        self.client
            .request_json::<CollectionFolders, (), ()>(
                Method::GET,
                &format!(
                    "/users/{}/collection/folders",
                    crate::endpoints::encode_path(username)
                ),
                None,
                None,
                AuthLevel::None,
            )
            .await
    }

    pub async fn get_folder(
        &self,
        username: &str,
        folder_id: u64,
    ) -> Result<ApiResponse<CollectionFolder>> {
        let required = if folder_id == 0 {
            AuthLevel::None
        } else {
            AuthLevel::User
        };

        self.client
            .request_json::<CollectionFolder, (), ()>(
                Method::GET,
                &format!(
                    "/users/{}/collection/folders/{folder_id}",
                    crate::endpoints::encode_path(username)
                ),
                None,
                None,
                required,
            )
            .await
    }

    pub async fn add_folder(
        &self,
        username: &str,
        request: &AddFolderRequest,
    ) -> Result<ApiResponse<CollectionFolder>> {
        self.client
            .request_json::<CollectionFolder, (), AddFolderRequest>(
                Method::POST,
                &format!(
                    "/users/{}/collection/folders",
                    crate::endpoints::encode_path(username)
                ),
                None,
                Some(request),
                AuthLevel::User,
            )
            .await
    }

    pub async fn set_folder_name(
        &self,
        username: &str,
        folder_id: u64,
        request: &AddFolderRequest,
    ) -> Result<ApiResponse<CollectionFolder>> {
        self.client
            .request_json::<CollectionFolder, (), AddFolderRequest>(
                Method::POST,
                &format!(
                    "/users/{}/collection/folders/{folder_id}",
                    crate::endpoints::encode_path(username)
                ),
                None,
                Some(request),
                AuthLevel::User,
            )
            .await
    }

    pub async fn delete_folder(&self, username: &str, folder_id: u64) -> Result<ApiResponse<()>> {
        self.client
            .request_empty::<(), ()>(
                Method::DELETE,
                &format!(
                    "/users/{}/collection/folders/{folder_id}",
                    crate::endpoints::encode_path(username)
                ),
                None,
                None,
                AuthLevel::User,
            )
            .await
    }

    pub async fn get_releases(
        &self,
        username: &str,
        folder_id: u64,
        query: Option<&CollectionReleasesQuery>,
    ) -> Result<ApiResponse<CollectionReleases>> {
        let required = if folder_id == 0 {
            AuthLevel::None
        } else {
            AuthLevel::User
        };

        self.client
            .request_json::<CollectionReleases, CollectionReleasesQuery, ()>(
                Method::GET,
                &format!(
                    "/users/{}/collection/folders/{folder_id}/releases",
                    crate::endpoints::encode_path(username)
                ),
                query,
                None,
                required,
            )
            .await
    }

    pub async fn get_release_instances(
        &self,
        username: &str,
        release_id: u64,
    ) -> Result<ApiResponse<CollectionReleases>> {
        self.client
            .request_json::<CollectionReleases, (), ()>(
                Method::GET,
                &format!(
                    "/users/{}/collection/releases/{release_id}",
                    crate::endpoints::encode_path(username)
                ),
                None,
                None,
                AuthLevel::None,
            )
            .await
    }

    pub async fn add_release(
        &self,
        username: &str,
        release_id: u64,
        folder_id: Option<u64>,
    ) -> Result<ApiResponse<AddCollectionReleaseResponse>> {
        let folder_id = folder_id.unwrap_or(1);
        self.client
            .request_json::<AddCollectionReleaseResponse, (), serde_json::Value>(
                Method::POST,
                &format!(
                    "/users/{}/collection/folders/{folder_id}/releases/{release_id}",
                    crate::endpoints::encode_path(username)
                ),
                None,
                Some(&serde_json::json!({})),
                AuthLevel::User,
            )
            .await
    }

    pub async fn edit_release(
        &self,
        username: &str,
        folder_id: u64,
        release_id: u64,
        instance_id: u64,
        request: &EditCollectionReleaseRequest,
    ) -> Result<ApiResponse<()>> {
        self.client
            .request_empty::<(), EditCollectionReleaseRequest>(
                Method::POST,
                &format!(
                    "/users/{}/collection/folders/{folder_id}/releases/{release_id}/instances/{instance_id}",
                    crate::endpoints::encode_path(username)
                ),
                None,
                Some(request),
                AuthLevel::User,
            )
            .await
    }

    pub async fn remove_release(
        &self,
        username: &str,
        folder_id: u64,
        release_id: u64,
        instance_id: u64,
    ) -> Result<ApiResponse<()>> {
        self.client
            .request_empty::<(), ()>(
                Method::DELETE,
                &format!(
                    "/users/{}/collection/folders/{folder_id}/releases/{release_id}/instances/{instance_id}",
                    crate::endpoints::encode_path(username)
                ),
                None,
                None,
                AuthLevel::User,
            )
            .await
    }

    pub async fn get_fields(&self, username: &str) -> Result<ApiResponse<CollectionFields>> {
        self.client
            .request_json::<CollectionFields, (), ()>(
                Method::GET,
                &format!(
                    "/users/{}/collection/fields",
                    crate::endpoints::encode_path(username)
                ),
                None,
                None,
                AuthLevel::None,
            )
            .await
    }

    pub async fn edit_instance_note(
        &self,
        username: &str,
        folder_id: u64,
        release_id: u64,
        instance_id: u64,
        field_id: u64,
        value: &str,
    ) -> Result<ApiResponse<()>> {
        let body = EditInstanceNoteRequest {
            value: value.to_string(),
        };

        self.client
            .request_empty::<(), EditInstanceNoteRequest>(
                Method::POST,
                &format!(
                    "/users/{}/collection/folders/{folder_id}/releases/{release_id}/instances/{instance_id}/fields/{field_id}",
                    crate::endpoints::encode_path(username)
                ),
                None,
                Some(&body),
                AuthLevel::User,
            )
            .await
    }

    pub async fn get_value(&self, username: &str) -> Result<ApiResponse<CollectionValue>> {
        self.client
            .request_json::<CollectionValue, (), ()>(
                Method::GET,
                &format!(
                    "/users/{}/collection/value",
                    crate::endpoints::encode_path(username)
                ),
                None,
                None,
                AuthLevel::User,
            )
            .await
    }
}
