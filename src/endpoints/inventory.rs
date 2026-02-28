//! Discogs Inventory Export API endpoints.
//!
//! Reference: <https://www.discogs.com/developers/#page:inventory-export>

use crate::auth::AuthLevel;
use crate::client::DiscogsClient;
use crate::error::Result;
use crate::models::{ApiResponse, InventoryExport, InventoryExports, PaginationParams};
use bytes::Bytes;
use reqwest::Method;

pub struct InventoryApi<'a> {
    client: &'a DiscogsClient,
}

impl<'a> InventoryApi<'a> {
    pub(crate) fn new(client: &'a DiscogsClient) -> Self {
        Self { client }
    }

    pub async fn export_inventory(&self) -> Result<ApiResponse<()>> {
        self.client
            .request_empty::<(), serde_json::Value>(
                Method::POST,
                "/inventory/export",
                None,
                Some(&serde_json::json!({})),
                AuthLevel::User,
            )
            .await
    }

    pub async fn get_exports(
        &self,
        query: Option<&PaginationParams>,
    ) -> Result<ApiResponse<InventoryExports>> {
        self.client
            .request_json::<InventoryExports, PaginationParams, ()>(
                Method::GET,
                "/inventory/export",
                query,
                None,
                AuthLevel::User,
            )
            .await
    }

    pub async fn get_export(&self, export_id: u64) -> Result<ApiResponse<InventoryExport>> {
        self.client
            .request_json::<InventoryExport, (), ()>(
                Method::GET,
                &format!("/inventory/export/{export_id}"),
                None,
                None,
                AuthLevel::User,
            )
            .await
    }

    pub async fn download_export(&self, export_id: u64) -> Result<ApiResponse<Bytes>> {
        self.client
            .request_bytes::<(), ()>(
                Method::GET,
                &format!("/inventory/export/{export_id}/download"),
                None,
                None,
                AuthLevel::User,
            )
            .await
    }
}
