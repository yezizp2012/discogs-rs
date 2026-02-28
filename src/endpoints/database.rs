//! Discogs Database API endpoints.
//!
//! Reference: <https://www.discogs.com/developers/#page:database>

use crate::auth::AuthLevel;
use crate::client::DiscogsClient;
use crate::error::Result;
use crate::models::{
    ApiResponse, Artist, ArtistReleases, ArtistReleasesQuery, Currency, Label, LabelReleases,
    Master, MasterVersions, MasterVersionsQuery, Release, ReleaseCommunityRating, ReleaseRating,
    ReleaseStats, SearchQuery, SearchResults,
};
use reqwest::Method;
use serde::Serialize;

pub struct DatabaseApi<'a> {
    client: &'a DiscogsClient,
}

impl<'a> DatabaseApi<'a> {
    pub(crate) fn new(client: &'a DiscogsClient) -> Self {
        Self { client }
    }

    pub async fn get_artist(&self, artist_id: u64) -> Result<ApiResponse<Artist>> {
        self.client
            .request_json::<Artist, (), ()>(
                Method::GET,
                &format!("/artists/{artist_id}"),
                None,
                None,
                AuthLevel::None,
            )
            .await
    }

    pub async fn get_artist_releases(
        &self,
        artist_id: u64,
        query: Option<&ArtistReleasesQuery>,
    ) -> Result<ApiResponse<ArtistReleases>> {
        self.client
            .request_json::<ArtistReleases, ArtistReleasesQuery, ()>(
                Method::GET,
                &format!("/artists/{artist_id}/releases"),
                query,
                None,
                AuthLevel::None,
            )
            .await
    }

    pub async fn get_release(
        &self,
        release_id: u64,
        currency: Option<Currency>,
    ) -> Result<ApiResponse<Release>> {
        #[derive(Serialize)]
        struct CurrencyQuery {
            curr_abbr: Currency,
        }

        let query = currency.map(|curr_abbr| CurrencyQuery { curr_abbr });
        self.client
            .request_json::<Release, CurrencyQuery, ()>(
                Method::GET,
                &format!("/releases/{release_id}"),
                query.as_ref(),
                None,
                AuthLevel::None,
            )
            .await
    }

    pub async fn get_release_rating(
        &self,
        release_id: u64,
        username: &str,
    ) -> Result<ApiResponse<ReleaseRating>> {
        let username = crate::endpoints::encode_path(username);
        self.client
            .request_json::<ReleaseRating, (), ()>(
                Method::GET,
                &format!("/releases/{release_id}/rating/{username}"),
                None,
                None,
                AuthLevel::None,
            )
            .await
    }

    pub async fn set_release_rating(
        &self,
        release_id: u64,
        username: &str,
        rating: u8,
    ) -> Result<ApiResponse<ReleaseRating>> {
        #[derive(Serialize)]
        struct RatingBody {
            rating: u8,
        }

        let rating = rating.clamp(1, 5);
        let username = crate::endpoints::encode_path(username);
        self.client
            .request_json::<ReleaseRating, (), RatingBody>(
                Method::PUT,
                &format!("/releases/{release_id}/rating/{username}"),
                None,
                Some(&RatingBody { rating }),
                AuthLevel::User,
            )
            .await
    }

    pub async fn delete_release_rating(
        &self,
        release_id: u64,
        username: &str,
    ) -> Result<ApiResponse<()>> {
        let username = crate::endpoints::encode_path(username);
        self.client
            .request_empty::<(), ()>(
                Method::DELETE,
                &format!("/releases/{release_id}/rating/{username}"),
                None,
                None,
                AuthLevel::User,
            )
            .await
    }

    pub async fn get_release_community_rating(
        &self,
        release_id: u64,
    ) -> Result<ApiResponse<ReleaseCommunityRating>> {
        self.client
            .request_json::<ReleaseCommunityRating, (), ()>(
                Method::GET,
                &format!("/releases/{release_id}/rating"),
                None,
                None,
                AuthLevel::None,
            )
            .await
    }

    pub async fn get_release_stats(&self, release_id: u64) -> Result<ApiResponse<ReleaseStats>> {
        self.client
            .request_json::<ReleaseStats, (), ()>(
                Method::GET,
                &format!("/releases/{release_id}/stats"),
                None,
                None,
                AuthLevel::None,
            )
            .await
    }

    pub async fn get_master(&self, master_id: u64) -> Result<ApiResponse<Master>> {
        self.client
            .request_json::<Master, (), ()>(
                Method::GET,
                &format!("/masters/{master_id}"),
                None,
                None,
                AuthLevel::None,
            )
            .await
    }

    pub async fn get_master_versions(
        &self,
        master_id: u64,
        query: Option<&MasterVersionsQuery>,
    ) -> Result<ApiResponse<MasterVersions>> {
        self.client
            .request_json::<MasterVersions, MasterVersionsQuery, ()>(
                Method::GET,
                &format!("/masters/{master_id}/versions"),
                query,
                None,
                AuthLevel::None,
            )
            .await
    }

    pub async fn get_label(&self, label_id: u64) -> Result<ApiResponse<Label>> {
        self.client
            .request_json::<Label, (), ()>(
                Method::GET,
                &format!("/labels/{label_id}"),
                None,
                None,
                AuthLevel::None,
            )
            .await
    }

    pub async fn get_label_releases(
        &self,
        label_id: u64,
        query: Option<&crate::models::PaginationParams>,
    ) -> Result<ApiResponse<LabelReleases>> {
        self.client
            .request_json::<LabelReleases, crate::models::PaginationParams, ()>(
                Method::GET,
                &format!("/labels/{label_id}/releases"),
                query,
                None,
                AuthLevel::None,
            )
            .await
    }

    pub async fn search(&self, query: Option<&SearchQuery>) -> Result<ApiResponse<SearchResults>> {
        self.client
            .request_json::<SearchResults, SearchQuery, ()>(
                Method::GET,
                "/database/search",
                query,
                None,
                AuthLevel::Consumer,
            )
            .await
    }
}
