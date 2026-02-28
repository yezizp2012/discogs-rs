//! Discogs Marketplace API endpoints.
//!
//! Reference: <https://www.discogs.com/developers/#page:marketplace>

use crate::auth::AuthLevel;
use crate::client::DiscogsClient;
use crate::error::Result;
use crate::models::{
    AddOrderMessageRequest, ApiResponse, CreateListingRequest, CreateListingResponse, Currency,
    EditOrderRequest, Listing, MarketplaceReleaseStats, Order, OrderMessage, OrderMessagesResponse,
    OrdersQuery, OrdersResponse, PaginationParams, Price,
};
use reqwest::Method;
use std::collections::BTreeMap;

pub struct MarketplaceApi<'a> {
    client: &'a DiscogsClient,
}

impl<'a> MarketplaceApi<'a> {
    pub(crate) fn new(client: &'a DiscogsClient) -> Self {
        Self { client }
    }

    pub async fn get_listing(
        &self,
        listing_id: u64,
        currency: Option<Currency>,
    ) -> Result<ApiResponse<Listing>> {
        #[derive(serde::Serialize)]
        struct CurrencyQuery {
            curr_abbr: Currency,
        }

        let query = currency.map(|curr_abbr| CurrencyQuery { curr_abbr });
        self.client
            .request_json::<Listing, CurrencyQuery, ()>(
                Method::GET,
                &format!("/marketplace/listings/{listing_id}"),
                query.as_ref(),
                None,
                AuthLevel::None,
            )
            .await
    }

    pub async fn add_listing(
        &self,
        request: &CreateListingRequest,
    ) -> Result<ApiResponse<CreateListingResponse>> {
        self.client
            .request_json::<CreateListingResponse, (), CreateListingRequest>(
                Method::POST,
                "/marketplace/listings",
                None,
                Some(request),
                AuthLevel::User,
            )
            .await
    }

    pub async fn edit_listing(
        &self,
        listing_id: u64,
        request: &CreateListingRequest,
    ) -> Result<ApiResponse<()>> {
        self.client
            .request_empty::<(), CreateListingRequest>(
                Method::POST,
                &format!("/marketplace/listings/{listing_id}"),
                None,
                Some(request),
                AuthLevel::User,
            )
            .await
    }

    pub async fn delete_listing(&self, listing_id: u64) -> Result<ApiResponse<()>> {
        self.client
            .request_empty::<(), ()>(
                Method::DELETE,
                &format!("/marketplace/listings/{listing_id}"),
                None,
                None,
                AuthLevel::User,
            )
            .await
    }

    pub async fn get_orders(
        &self,
        query: Option<&OrdersQuery>,
    ) -> Result<ApiResponse<OrdersResponse>> {
        self.client
            .request_json::<OrdersResponse, OrdersQuery, ()>(
                Method::GET,
                "/marketplace/orders",
                query,
                None,
                AuthLevel::User,
            )
            .await
    }

    pub async fn get_order(&self, order_id: &str) -> Result<ApiResponse<Order>> {
        self.client
            .request_json::<Order, (), ()>(
                Method::GET,
                &format!(
                    "/marketplace/orders/{}",
                    crate::endpoints::encode_path(order_id)
                ),
                None,
                None,
                AuthLevel::User,
            )
            .await
    }

    pub async fn edit_order(
        &self,
        order_id: &str,
        request: &EditOrderRequest,
    ) -> Result<ApiResponse<Order>> {
        self.client
            .request_json::<Order, (), EditOrderRequest>(
                Method::POST,
                &format!(
                    "/marketplace/orders/{}",
                    crate::endpoints::encode_path(order_id)
                ),
                None,
                Some(request),
                AuthLevel::User,
            )
            .await
    }

    pub async fn get_order_messages(
        &self,
        order_id: &str,
        query: Option<&PaginationParams>,
    ) -> Result<ApiResponse<OrderMessagesResponse>> {
        self.client
            .request_json::<OrderMessagesResponse, PaginationParams, ()>(
                Method::GET,
                &format!(
                    "/marketplace/orders/{}/messages",
                    crate::endpoints::encode_path(order_id)
                ),
                query,
                None,
                AuthLevel::User,
            )
            .await
    }

    pub async fn add_order_message(
        &self,
        order_id: &str,
        request: &AddOrderMessageRequest,
    ) -> Result<ApiResponse<OrderMessage>> {
        self.client
            .request_json::<OrderMessage, (), AddOrderMessageRequest>(
                Method::POST,
                &format!(
                    "/marketplace/orders/{}/messages",
                    crate::endpoints::encode_path(order_id)
                ),
                None,
                Some(request),
                AuthLevel::User,
            )
            .await
    }

    pub async fn get_fee(
        &self,
        amount: f64,
        currency: Option<Currency>,
    ) -> Result<ApiResponse<Price>> {
        let path = match currency {
            Some(currency) => format!("/marketplace/fee/{:.2}/{}", amount, currency.as_code()),
            None => format!("/marketplace/fee/{:.2}", amount),
        };

        self.client
            .request_json::<Price, (), ()>(Method::GET, &path, None, None, AuthLevel::None)
            .await
    }

    pub async fn get_price_suggestions(
        &self,
        release_id: u64,
    ) -> Result<ApiResponse<BTreeMap<String, Price>>> {
        self.client
            .request_json::<BTreeMap<String, Price>, (), ()>(
                Method::GET,
                &format!("/marketplace/price_suggestions/{release_id}"),
                None,
                None,
                AuthLevel::User,
            )
            .await
    }

    pub async fn get_release_stats(
        &self,
        release_id: u64,
        currency: Option<Currency>,
    ) -> Result<ApiResponse<MarketplaceReleaseStats>> {
        #[derive(serde::Serialize)]
        struct CurrencyQuery {
            curr_abbr: Currency,
        }

        let query = currency.map(|curr_abbr| CurrencyQuery { curr_abbr });
        self.client
            .request_json::<MarketplaceReleaseStats, CurrencyQuery, ()>(
                Method::GET,
                &format!("/marketplace/stats/{release_id}"),
                query.as_ref(),
                None,
                AuthLevel::None,
            )
            .await
    }
}
