use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type ExtraFields = BTreeMap<String, serde_json::Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    pub limit: u32,
    pub used: u32,
    pub remaining: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub rate_limit: Option<RateLimit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub per_page: u32,
    pub pages: u32,
    pub page: u32,
    pub items: u32,
    #[serde(default)]
    pub urls: Option<PaginationUrls>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationUrls {
    #[serde(default)]
    pub first: Option<String>,
    #[serde(default)]
    pub prev: Option<String>,
    #[serde(default)]
    pub next: Option<String>,
    #[serde(default)]
    pub last: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AboutStatistics {
    pub releases: u64,
    pub artists: u64,
    pub labels: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AboutResponse {
    pub hello: String,
    pub api_version: String,
    pub documentation_url: String,
    pub statistics: AboutStatistics,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub id: u64,
    pub username: String,
    pub resource_url: String,
    #[serde(default)]
    pub consumer_name: Option<String>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub resource_url: String,
    #[serde(rename = "type")]
    pub image_type: String,
    pub uri: String,
    #[serde(default)]
    pub uri150: Option<String>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserReference {
    #[serde(default)]
    pub id: Option<u64>,
    pub username: String,
    pub resource_url: String,
    #[serde(default)]
    pub avatar_url: Option<String>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistCredit {
    #[serde(default)]
    pub anv: Option<String>,
    pub id: u64,
    #[serde(default)]
    pub join: Option<String>,
    pub name: String,
    pub resource_url: String,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub tracks: Option<String>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Format {
    #[serde(default)]
    pub qty: Option<String>,
    pub name: String,
    #[serde(default)]
    pub descriptions: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelReference {
    pub resource_url: String,
    #[serde(default)]
    pub entity_type: Option<String>,
    #[serde(default)]
    pub catno: Option<String>,
    pub id: u64,
    pub name: String,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    #[serde(default)]
    pub duration: Option<String>,
    #[serde(default)]
    pub position: Option<String>,
    pub title: String,
    #[serde(rename = "type_", default)]
    pub track_type: Option<String>,
    #[serde(default)]
    pub extraartists: Option<Vec<ArtistCredit>>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub duration: Option<u64>,
    #[serde(default)]
    pub embed: Option<bool>,
    #[serde(default)]
    pub title: Option<String>,
    pub uri: String,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistMember {
    pub active: bool,
    pub id: u64,
    pub name: String,
    pub resource_url: String,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artist {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub namevariations: Option<Vec<String>>,
    #[serde(default)]
    pub profile: Option<String>,
    pub resource_url: String,
    #[serde(default)]
    pub releases_url: Option<String>,
    #[serde(default)]
    pub uri: Option<String>,
    #[serde(default)]
    pub urls: Option<Vec<String>>,
    #[serde(default)]
    pub data_quality: Option<String>,
    #[serde(default)]
    pub images: Option<Vec<Image>>,
    #[serde(default)]
    pub members: Option<Vec<ArtistMember>>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistRelease {
    #[serde(default)]
    pub artist: Option<String>,
    pub id: u64,
    #[serde(default)]
    pub main_release: Option<u64>,
    pub resource_url: String,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub thumb: Option<String>,
    pub title: String,
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub year: Option<i32>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistReleases {
    pub releases: Vec<ArtistRelease>,
    pub pagination: Pagination,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseCommunityRatingSummary {
    pub average: f64,
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseCommunity {
    #[serde(default)]
    pub contributors: Option<Vec<UserReference>>,
    #[serde(default)]
    pub data_quality: Option<String>,
    #[serde(default)]
    pub have: Option<u64>,
    #[serde(default)]
    pub rating: Option<ReleaseCommunityRatingSummary>,
    #[serde(default)]
    pub status: Option<DatabaseStatus>,
    #[serde(default)]
    pub submitter: Option<UserReference>,
    #[serde(default)]
    pub want: Option<u64>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseCompany {
    #[serde(default)]
    pub catno: Option<String>,
    #[serde(default)]
    pub entity_type: Option<String>,
    #[serde(default)]
    pub entity_type_name: Option<String>,
    pub id: u64,
    pub name: String,
    pub resource_url: String,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseIdentifier {
    #[serde(rename = "type")]
    pub id_type: String,
    pub value: String,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseSeries {
    pub name: String,
    #[serde(default)]
    pub catno: Option<String>,
    #[serde(default)]
    pub entity_type: Option<String>,
    #[serde(default)]
    pub entity_type_name: Option<String>,
    pub id: u64,
    pub resource_url: String,
    #[serde(default)]
    pub thumbnail_url: Option<String>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Release {
    pub id: u64,
    pub title: String,
    pub resource_url: String,
    #[serde(default)]
    pub uri: Option<String>,
    #[serde(default)]
    pub year: Option<i32>,
    #[serde(default)]
    pub artists: Option<Vec<ArtistCredit>>,
    #[serde(default)]
    pub data_quality: Option<String>,
    #[serde(default)]
    pub thumb: Option<String>,
    #[serde(default)]
    pub community: Option<ReleaseCommunity>,
    #[serde(default)]
    pub companies: Option<Vec<ReleaseCompany>>,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub date_added: Option<String>,
    #[serde(default)]
    pub date_changed: Option<String>,
    #[serde(default)]
    pub estimated_weight: Option<u64>,
    #[serde(default)]
    pub extraartists: Option<Vec<ArtistCredit>>,
    #[serde(default)]
    pub format_quantity: Option<u32>,
    #[serde(default)]
    pub formats: Option<Vec<Format>>,
    #[serde(default)]
    pub genres: Option<Vec<String>>,
    #[serde(default)]
    pub identifiers: Option<Vec<ReleaseIdentifier>>,
    #[serde(default)]
    pub images: Option<Vec<Image>>,
    #[serde(default)]
    pub labels: Option<Vec<LabelReference>>,
    #[serde(default)]
    pub lowest_price: Option<f64>,
    #[serde(default)]
    pub master_id: Option<u64>,
    #[serde(default)]
    pub master_url: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub num_for_sale: Option<u64>,
    #[serde(default)]
    pub released: Option<String>,
    #[serde(default)]
    pub released_formatted: Option<String>,
    #[serde(default)]
    pub series: Option<Vec<ReleaseSeries>>,
    #[serde(default)]
    pub status: Option<DatabaseStatus>,
    #[serde(default)]
    pub styles: Option<Vec<String>>,
    #[serde(default)]
    pub tracklist: Option<Vec<Track>>,
    #[serde(default)]
    pub videos: Option<Vec<Video>>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseRating {
    pub username: String,
    pub release_id: u64,
    pub rating: u8,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatingSummary {
    pub average: f64,
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseCommunityRating {
    pub release_id: u64,
    pub rating: RatingSummary,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseStats {
    pub num_have: u64,
    pub num_want: u64,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Master {
    pub id: u64,
    pub title: String,
    pub resource_url: String,
    #[serde(default)]
    pub main_release: Option<u64>,
    #[serde(default)]
    pub main_release_url: Option<String>,
    #[serde(default)]
    pub uri: Option<String>,
    #[serde(default)]
    pub artists: Option<Vec<ArtistCredit>>,
    #[serde(default)]
    pub versions_url: Option<String>,
    #[serde(default)]
    pub year: Option<i32>,
    #[serde(default)]
    pub images: Option<Vec<Image>>,
    #[serde(default)]
    pub tracklist: Option<Vec<Track>>,
    #[serde(default)]
    pub videos: Option<Vec<Video>>,
    #[serde(default)]
    pub genres: Option<Vec<String>>,
    #[serde(default)]
    pub styles: Option<Vec<String>>,
    #[serde(default)]
    pub num_for_sale: Option<u64>,
    #[serde(default)]
    pub lowest_price: Option<f64>,
    #[serde(default)]
    pub data_quality: Option<String>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionStats {
    pub in_collection: u64,
    pub in_wantlist: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterVersionStats {
    #[serde(default)]
    pub user: Option<CollectionStats>,
    #[serde(default)]
    pub community: Option<CollectionStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterVersion {
    #[serde(default)]
    pub status: Option<DatabaseStatus>,
    #[serde(default)]
    pub stats: Option<MasterVersionStats>,
    #[serde(default)]
    pub thumb: Option<String>,
    #[serde(default)]
    pub format: Option<String>,
    #[serde(default)]
    pub country: Option<String>,
    pub title: String,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub released: Option<String>,
    #[serde(default)]
    pub major_formats: Option<Vec<String>>,
    #[serde(default)]
    pub catno: Option<String>,
    pub resource_url: String,
    pub id: u64,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterVersions {
    pub versions: Vec<MasterVersion>,
    pub pagination: Pagination,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelShort {
    pub resource_url: String,
    pub id: u64,
    pub name: String,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub id: u64,
    pub name: String,
    pub resource_url: String,
    #[serde(default)]
    pub uri: Option<String>,
    #[serde(default)]
    pub releases_url: Option<String>,
    #[serde(default)]
    pub profile: Option<String>,
    #[serde(default)]
    pub contact_info: Option<String>,
    #[serde(default)]
    pub urls: Option<Vec<String>>,
    #[serde(default)]
    pub images: Option<Vec<Image>>,
    #[serde(default)]
    pub sublabels: Option<Vec<LabelShort>>,
    #[serde(default)]
    pub data_quality: Option<String>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelRelease {
    #[serde(default)]
    pub artist: Option<String>,
    #[serde(default)]
    pub catno: Option<String>,
    #[serde(default)]
    pub format: Option<String>,
    pub id: u64,
    pub resource_url: String,
    #[serde(default)]
    pub status: Option<DatabaseStatus>,
    #[serde(default)]
    pub thumb: Option<String>,
    pub title: String,
    #[serde(default)]
    pub year: Option<i32>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelReleases {
    pub releases: Vec<LabelRelease>,
    pub pagination: Pagination,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultUserData {
    #[serde(default)]
    pub in_wantlist: Option<bool>,
    #[serde(default)]
    pub in_collection: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultCommunity {
    #[serde(default)]
    pub want: Option<u64>,
    #[serde(default)]
    pub have: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: u64,
    #[serde(rename = "type")]
    pub result_type: String,
    #[serde(default)]
    pub user_data: Option<SearchResultUserData>,
    #[serde(default)]
    pub master_id: Option<u64>,
    #[serde(default)]
    pub master_url: Option<String>,
    pub uri: String,
    pub title: String,
    #[serde(default)]
    pub thumb: Option<String>,
    #[serde(default)]
    pub cover_image: Option<String>,
    pub resource_url: String,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub year: Option<String>,
    #[serde(default)]
    pub format: Option<Vec<String>>,
    #[serde(default)]
    pub label: Option<Vec<String>>,
    #[serde(default)]
    pub genre: Option<Vec<String>>,
    #[serde(default)]
    pub style: Option<Vec<String>>,
    #[serde(default)]
    pub barcode: Option<Vec<String>>,
    #[serde(default)]
    pub catno: Option<String>,
    #[serde(default)]
    pub community: Option<SearchResultCommunity>,
    #[serde(default)]
    pub format_quantity: Option<u32>,
    #[serde(default)]
    pub formats: Option<Vec<Format>>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults {
    pub results: Vec<SearchResult>,
    pub pagination: Pagination,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Currency {
    Usd,
    Gbp,
    Eur,
    Cad,
    Aud,
    Jpy,
    Chf,
    Mxn,
    Brl,
    Nzd,
    Sek,
    Zar,
}

impl Currency {
    pub fn as_code(self) -> &'static str {
        match self {
            Currency::Usd => "USD",
            Currency::Gbp => "GBP",
            Currency::Eur => "EUR",
            Currency::Cad => "CAD",
            Currency::Aud => "AUD",
            Currency::Jpy => "JPY",
            Currency::Chf => "CHF",
            Currency::Mxn => "MXN",
            Currency::Brl => "BRL",
            Currency::Nzd => "NZD",
            Currency::Sek => "SEK",
            Currency::Zar => "ZAR",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Price {
    #[serde(default)]
    pub currency: Option<Currency>,
    #[serde(default)]
    pub value: Option<f64>,
    #[serde(default)]
    pub curr_abbr: Option<Currency>,
    #[serde(default)]
    pub curr_id: Option<u64>,
    #[serde(default)]
    pub formatted: Option<String>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListingReleaseSummary {
    #[serde(default)]
    pub catalog_number: Option<String>,
    pub resource_url: String,
    #[serde(default)]
    pub year: Option<i32>,
    pub id: u64,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub images: Option<Vec<Image>>,
    #[serde(default)]
    pub artist: Option<String>,
    pub title: String,
    #[serde(default)]
    pub format: Option<String>,
    #[serde(default)]
    pub thumbnail: Option<String>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listing {
    pub id: u64,
    #[serde(default)]
    pub status: Option<SaleStatus>,
    #[serde(default)]
    pub resource_url: Option<String>,
    #[serde(default)]
    pub uri: Option<String>,
    #[serde(default)]
    pub price: Option<Price>,
    #[serde(default)]
    pub sleeve_condition: Option<SleeveCondition>,
    #[serde(default)]
    pub condition: Option<MediaCondition>,
    #[serde(default)]
    pub posted: Option<String>,
    #[serde(default)]
    pub comments: Option<String>,
    #[serde(default)]
    pub allow_offers: Option<bool>,
    #[serde(default)]
    pub ships_from: Option<String>,
    #[serde(default)]
    pub audio: Option<bool>,
    #[serde(default)]
    pub release: Option<ListingReleaseSummary>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateListingResponse {
    pub listing_id: u64,
    pub resource_url: String,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    #[serde(default)]
    pub release: Option<ListingReleaseSummary>,
    #[serde(default)]
    pub price: Option<Price>,
    #[serde(default)]
    pub media_condition: Option<MediaCondition>,
    #[serde(default)]
    pub sleeve_condition: Option<SleeveCondition>,
    pub id: u64,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub resource_url: String,
    #[serde(default)]
    pub status: Option<OrderStatus>,
    #[serde(default)]
    pub messages_url: Option<String>,
    #[serde(default)]
    pub uri: Option<String>,
    #[serde(default)]
    pub fee: Option<Price>,
    #[serde(default)]
    pub created: Option<String>,
    #[serde(default)]
    pub items: Option<Vec<OrderItem>>,
    #[serde(default)]
    pub shipping_address: Option<String>,
    #[serde(default)]
    pub additional_instructions: Option<String>,
    #[serde(default)]
    pub archived: Option<bool>,
    #[serde(default)]
    pub seller: Option<UserReference>,
    #[serde(default)]
    pub last_activity: Option<String>,
    #[serde(default)]
    pub buyer: Option<UserReference>,
    #[serde(default)]
    pub total: Option<Price>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrdersResponse {
    pub orders: Vec<Order>,
    pub pagination: Pagination,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderMessage {
    pub message: String,
    #[serde(default)]
    pub timestamp: Option<String>,
    #[serde(default)]
    pub subject: Option<String>,
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub status_id: Option<u64>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderMessagesResponse {
    pub messages: Vec<OrderMessage>,
    pub pagination: Pagination,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceReleaseStats {
    #[serde(default)]
    pub lowest_price: Option<Price>,
    #[serde(default)]
    pub num_for_sale: Option<u64>,
    #[serde(default)]
    pub blocked_from_sale: Option<bool>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryExport {
    pub id: u64,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub created_ts: Option<String>,
    #[serde(default)]
    pub finished_ts: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub download_url: Option<String>,
    #[serde(default)]
    pub filename: Option<String>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryExports {
    pub items: Vec<InventoryExport>,
    pub pagination: Pagination,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserListSummary {
    pub id: u64,
    pub name: String,
    pub resource_url: String,
    #[serde(default)]
    pub uri: Option<String>,
    #[serde(default)]
    pub public: Option<bool>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub date_added: Option<String>,
    #[serde(default)]
    pub date_changed: Option<String>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: u64,
    pub username: String,
    pub resource_url: String,
    #[serde(default)]
    pub curr_abbr: Option<Currency>,
    #[serde(default)]
    pub avatar_url: Option<String>,
    #[serde(default)]
    pub profile: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub home_page: Option<String>,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub num_collection: Option<u64>,
    #[serde(default)]
    pub num_wantlist: Option<u64>,
    #[serde(default)]
    pub num_for_sale: Option<u64>,
    #[serde(default)]
    pub num_lists: Option<u64>,
    #[serde(default)]
    pub registered: Option<String>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInventory {
    pub listings: Vec<Listing>,
    pub pagination: Pagination,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserContributions {
    pub contributions: Vec<Release>,
    pub pagination: Pagination,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmissionArtist {
    pub id: u64,
    pub name: String,
    pub resource_url: String,
    #[serde(default)]
    pub releases_url: Option<String>,
    #[serde(default)]
    pub uri: Option<String>,
    #[serde(default)]
    pub namevariations: Option<Vec<String>>,
    #[serde(default)]
    pub data_quality: Option<String>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSubmissionsPayload {
    pub artists: Vec<SubmissionArtist>,
    pub labels: Vec<Label>,
    pub releases: Vec<Release>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSubmissions {
    pub submissions: UserSubmissionsPayload,
    pub pagination: Pagination,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLists {
    pub lists: Vec<UserListSummary>,
    pub pagination: Pagination,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionFolder {
    pub id: u64,
    pub count: u64,
    pub name: String,
    pub resource_url: String,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionFolders {
    pub folders: Vec<CollectionFolder>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicReleaseInfo {
    pub id: u64,
    pub title: String,
    #[serde(default)]
    pub year: Option<i32>,
    pub resource_url: String,
    #[serde(default)]
    pub thumb: Option<String>,
    #[serde(default)]
    pub cover_image: Option<String>,
    #[serde(default)]
    pub formats: Option<Vec<Format>>,
    #[serde(default)]
    pub labels: Option<Vec<LabelReference>>,
    #[serde(default)]
    pub artists: Option<Vec<ArtistCredit>>,
    #[serde(default)]
    pub genres: Option<Vec<String>>,
    #[serde(default)]
    pub styles: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionNote {
    pub field_id: u64,
    pub value: String,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionRelease {
    pub id: u64,
    pub instance_id: u64,
    #[serde(default)]
    pub folder_id: Option<u64>,
    #[serde(default)]
    pub rating: Option<u8>,
    #[serde(default)]
    pub basic_information: Option<BasicReleaseInfo>,
    #[serde(default)]
    pub notes: Option<Vec<CollectionNote>>,
    #[serde(default)]
    pub date_added: Option<String>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionReleases {
    pub releases: Vec<CollectionRelease>,
    pub pagination: Pagination,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddCollectionReleaseResponse {
    pub instance_id: u64,
    pub resource_url: String,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionField {
    pub id: u64,
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
    pub public: bool,
    pub position: u32,
    #[serde(default)]
    pub options: Option<Vec<String>>,
    #[serde(default)]
    pub lines: Option<u32>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionFields {
    pub fields: Vec<CollectionField>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionValue {
    pub minimum: String,
    pub median: String,
    pub maximum: String,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WantlistBasicInformation {
    pub resource_url: String,
    pub id: u64,
    #[serde(default)]
    pub formats: Option<Vec<Format>>,
    #[serde(default)]
    pub thumb: Option<String>,
    #[serde(default)]
    pub cover_image: Option<String>,
    pub title: String,
    #[serde(default)]
    pub labels: Option<Vec<LabelReference>>,
    #[serde(default)]
    pub year: Option<i32>,
    #[serde(default)]
    pub artists: Option<Vec<ArtistCredit>>,
    #[serde(default)]
    pub genres: Option<Vec<String>>,
    #[serde(default)]
    pub styles: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WantlistEntry {
    pub id: u64,
    pub resource_url: String,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub rating: Option<u8>,
    #[serde(default)]
    pub basic_information: Option<WantlistBasicInformation>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WantlistEntries {
    pub wants: Vec<WantlistEntry>,
    pub pagination: Pagination,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscogsList {
    pub list_id: u64,
    pub name: String,
    pub resource_url: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub created_ts: Option<String>,
    #[serde(default)]
    pub modified_ts: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub public: Option<bool>,
    pub items: Vec<ListItem>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItem {
    pub id: u64,
    pub display_title: String,
    #[serde(default)]
    pub comment: Option<String>,
    #[serde(default)]
    pub uri: Option<String>,
    #[serde(default)]
    pub image_url: Option<String>,
    #[serde(default)]
    pub resource_url: Option<String>,
    #[serde(rename = "type", default)]
    pub item_type: Option<String>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaginationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SearchType {
    Release,
    Master,
    Artist,
    Label,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DatabaseStatus {
    Accepted,
    Draft,
    Deleted,
    Rejected,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SaleStatus {
    Draft,
    #[serde(rename = "For Sale")]
    ForSale,
    Expired,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MediaCondition {
    #[serde(rename = "Mint (M)")]
    Mint,
    #[serde(rename = "Near Mint (NM or M-)")]
    NearMint,
    #[serde(rename = "Very Good Plus (VG+)")]
    VeryGoodPlus,
    #[serde(rename = "Very Good (VG)")]
    VeryGood,
    #[serde(rename = "Good Plus (G+)")]
    GoodPlus,
    #[serde(rename = "Good (G)")]
    Good,
    #[serde(rename = "Fair (F)")]
    Fair,
    #[serde(rename = "Poor (P)")]
    Poor,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SleeveCondition {
    #[serde(rename = "Mint (M)")]
    Mint,
    #[serde(rename = "Near Mint (NM or M-)")]
    NearMint,
    #[serde(rename = "Very Good Plus (VG+)")]
    VeryGoodPlus,
    #[serde(rename = "Very Good (VG)")]
    VeryGood,
    #[serde(rename = "Good Plus (G+)")]
    GoodPlus,
    #[serde(rename = "Good (G)")]
    Good,
    #[serde(rename = "Fair (F)")]
    Fair,
    #[serde(rename = "Poor (P)")]
    Poor,
    Generic,
    #[serde(rename = "Not Graded")]
    NotGraded,
    #[serde(rename = "No Cover")]
    NoCover,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OrderStatus {
    #[serde(rename = "New Order")]
    NewOrder,
    #[serde(rename = "Buyer Contacted")]
    BuyerContacted,
    #[serde(rename = "Invoice Sent")]
    InvoiceSent,
    #[serde(rename = "Payment Pending")]
    PaymentPending,
    #[serde(rename = "Payment Received")]
    PaymentReceived,
    Shipped,
    #[serde(rename = "Refund Sent")]
    RefundSent,
    #[serde(rename = "Cancelled (Non-Paying Buyer)")]
    CancelledNonPayingBuyer,
    #[serde(rename = "Cancelled (Item Unavailable)")]
    CancelledItemUnavailable,
    #[serde(rename = "Cancelled (Per Buyer's Request)")]
    CancelledByBuyerRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NumberOrAuto {
    Number(u32),
    Auto(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArtistReleasesQuery {
    #[serde(flatten)]
    pub pagination: PaginationParams,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SortOrder>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MasterVersionsQuery {
    #[serde(flatten)]
    pub pagination: PaginationParams,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub released: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SortOrder>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchQuery {
    #[serde(rename = "q", skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub query_type: Option<SearchType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catno: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub barcode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributor: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateListingRequest {
    pub release_id: u64,
    pub condition: MediaCondition,
    pub price: f64,
    pub status: SaleStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sleeve_condition: Option<SleeveCondition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_offers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<NumberOrAuto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_quantity: Option<NumberOrAuto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrdersQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OrderStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SortOrder>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditOrderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OrderStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddOrderMessageRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OrderStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditProfileRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_page: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curr_abbr: Option<Currency>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserInventoryQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SortOrder>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserContributionsQuery {
    #[serde(flatten)]
    pub pagination: PaginationParams,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SortOrder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddFolderRequest {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CollectionReleasesQuery {
    #[serde(flatten)]
    pub pagination: PaginationParams,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SortOrder>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditCollectionReleaseRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditInstanceNoteRequest {
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WantlistUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<u8>,
}
