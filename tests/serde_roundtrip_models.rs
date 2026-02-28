use discogs_rs::{
    CreateListingRequest, Currency, MediaCondition, NumberOrAuto, OrderStatus, OrdersQuery,
    PaginationParams, SaleStatus, SearchQuery, SearchType, SleeveCondition, SortOrder,
};
use serde_json::{Value, json};

fn assert_roundtrip<T>(typed: T, expected_json: Value)
where
    T: serde::Serialize + for<'de> serde::Deserialize<'de>,
{
    let serialized = serde_json::to_value(&typed).expect("serialize typed value");
    assert_eq!(serialized, expected_json);

    let deserialized: T =
        serde_json::from_value(expected_json.clone()).expect("deserialize json value");
    let reserialized = serde_json::to_value(&deserialized).expect("serialize deserialized value");
    assert_eq!(reserialized, expected_json);
}

#[test]
fn currency_serde_roundtrip() {
    let cases = [
        (Currency::Usd, "USD"),
        (Currency::Gbp, "GBP"),
        (Currency::Eur, "EUR"),
        (Currency::Cad, "CAD"),
        (Currency::Aud, "AUD"),
        (Currency::Jpy, "JPY"),
        (Currency::Chf, "CHF"),
        (Currency::Mxn, "MXN"),
        (Currency::Brl, "BRL"),
        (Currency::Nzd, "NZD"),
        (Currency::Sek, "SEK"),
        (Currency::Zar, "ZAR"),
    ];

    for (typed, wire) in cases {
        assert_roundtrip(typed, json!(wire));
    }
}

#[test]
fn search_type_serde_roundtrip() {
    let cases = [
        (SearchType::Release, "release"),
        (SearchType::Master, "master"),
        (SearchType::Artist, "artist"),
        (SearchType::Label, "label"),
    ];

    for (typed, wire) in cases {
        assert_roundtrip(typed, json!(wire));
    }
}

#[test]
fn sale_status_serde_roundtrip() {
    let cases = [
        (SaleStatus::Draft, "Draft"),
        (SaleStatus::ForSale, "For Sale"),
        (SaleStatus::Expired, "Expired"),
    ];

    for (typed, wire) in cases {
        assert_roundtrip(typed, json!(wire));
    }
}

#[test]
fn media_condition_serde_roundtrip() {
    let cases = [
        (MediaCondition::Mint, "Mint (M)"),
        (MediaCondition::NearMint, "Near Mint (NM or M-)"),
        (MediaCondition::VeryGoodPlus, "Very Good Plus (VG+)"),
        (MediaCondition::VeryGood, "Very Good (VG)"),
        (MediaCondition::GoodPlus, "Good Plus (G+)"),
        (MediaCondition::Good, "Good (G)"),
        (MediaCondition::Fair, "Fair (F)"),
        (MediaCondition::Poor, "Poor (P)"),
    ];

    for (typed, wire) in cases {
        assert_roundtrip(typed, json!(wire));
    }
}

#[test]
fn sleeve_condition_serde_roundtrip() {
    let cases = [
        (SleeveCondition::Mint, "Mint (M)"),
        (SleeveCondition::NearMint, "Near Mint (NM or M-)"),
        (SleeveCondition::VeryGoodPlus, "Very Good Plus (VG+)"),
        (SleeveCondition::VeryGood, "Very Good (VG)"),
        (SleeveCondition::GoodPlus, "Good Plus (G+)"),
        (SleeveCondition::Good, "Good (G)"),
        (SleeveCondition::Fair, "Fair (F)"),
        (SleeveCondition::Poor, "Poor (P)"),
        (SleeveCondition::Generic, "Generic"),
        (SleeveCondition::NotGraded, "Not Graded"),
        (SleeveCondition::NoCover, "No Cover"),
    ];

    for (typed, wire) in cases {
        assert_roundtrip(typed, json!(wire));
    }
}

#[test]
fn order_status_serde_roundtrip() {
    let cases = [
        (OrderStatus::NewOrder, "New Order"),
        (OrderStatus::BuyerContacted, "Buyer Contacted"),
        (OrderStatus::InvoiceSent, "Invoice Sent"),
        (OrderStatus::PaymentPending, "Payment Pending"),
        (OrderStatus::PaymentReceived, "Payment Received"),
        (OrderStatus::Shipped, "Shipped"),
        (OrderStatus::RefundSent, "Refund Sent"),
        (
            OrderStatus::CancelledNonPayingBuyer,
            "Cancelled (Non-Paying Buyer)",
        ),
        (
            OrderStatus::CancelledItemUnavailable,
            "Cancelled (Item Unavailable)",
        ),
        (
            OrderStatus::CancelledByBuyerRequest,
            "Cancelled (Per Buyer's Request)",
        ),
    ];

    for (typed, wire) in cases {
        assert_roundtrip(typed, json!(wire));
    }
}

#[test]
fn number_or_auto_serde_roundtrip() {
    assert_roundtrip(NumberOrAuto::Number(250), json!(250));
    assert_roundtrip(NumberOrAuto::Auto("Auto".to_owned()), json!("Auto"));
}

#[test]
fn search_query_serde_roundtrip() {
    let typed = SearchQuery {
        query: Some("nirvana nevermind".to_owned()),
        query_type: Some(SearchType::Release),
        title: Some("Nevermind".to_owned()),
        release_title: Some("Nevermind".to_owned()),
        credit: Some("Kurt Cobain".to_owned()),
        artist: Some("Nirvana".to_owned()),
        anv: Some("Nirvana".to_owned()),
        label: Some("DGC".to_owned()),
        genre: Some("Rock".to_owned()),
        style: Some("Grunge".to_owned()),
        country: Some("US".to_owned()),
        year: Some("1991".to_owned()),
        format: Some("Vinyl".to_owned()),
        catno: Some("DGCD-24425".to_owned()),
        barcode: Some("720642442510".to_owned()),
        track: Some("Smells Like Teen Spirit".to_owned()),
        submitter: Some("alice".to_owned()),
        contributor: Some("bob".to_owned()),
        pagination: PaginationParams {
            page: Some(2),
            per_page: Some(50),
        },
    };

    let expected = json!({
        "q": "nirvana nevermind",
        "type": "release",
        "title": "Nevermind",
        "release_title": "Nevermind",
        "credit": "Kurt Cobain",
        "artist": "Nirvana",
        "anv": "Nirvana",
        "label": "DGC",
        "genre": "Rock",
        "style": "Grunge",
        "country": "US",
        "year": "1991",
        "format": "Vinyl",
        "catno": "DGCD-24425",
        "barcode": "720642442510",
        "track": "Smells Like Teen Spirit",
        "submitter": "alice",
        "contributor": "bob",
        "page": 2,
        "per_page": 50
    });

    assert_roundtrip(typed, expected);
}

#[test]
fn create_listing_request_serde_roundtrip() {
    let typed = CreateListingRequest {
        release_id: 249504,
        condition: MediaCondition::NearMint,
        price: 19.99,
        status: SaleStatus::ForSale,
        sleeve_condition: Some(SleeveCondition::VeryGoodPlus),
        comments: Some("Play graded".to_owned()),
        allow_offers: Some(true),
        external_id: Some("EXT-42".to_owned()),
        location: Some("Shelf-A1".to_owned()),
        weight: Some(NumberOrAuto::Number(250)),
        format_quantity: Some(NumberOrAuto::Auto("Auto".to_owned())),
    };

    let expected = json!({
        "release_id": 249504,
        "condition": "Near Mint (NM or M-)",
        "price": 19.99,
        "status": "For Sale",
        "sleeve_condition": "Very Good Plus (VG+)",
        "comments": "Play graded",
        "allow_offers": true,
        "external_id": "EXT-42",
        "location": "Shelf-A1",
        "weight": 250,
        "format_quantity": "Auto"
    });

    assert_roundtrip(typed, expected);
}

#[test]
fn orders_query_serde_roundtrip() {
    let typed = OrdersQuery {
        status: Some(OrderStatus::PaymentReceived),
        created_after: Some("2024-01-01T00:00:00".to_owned()),
        created_before: Some("2024-12-31T23:59:59".to_owned()),
        archived: Some(false),
        pagination: PaginationParams {
            page: Some(3),
            per_page: Some(100),
        },
        sort: Some("id".to_owned()),
        sort_order: Some(SortOrder::Desc),
    };

    let expected = json!({
        "status": "Payment Received",
        "created_after": "2024-01-01T00:00:00",
        "created_before": "2024-12-31T23:59:59",
        "archived": false,
        "page": 3,
        "per_page": 100,
        "sort": "id",
        "sort_order": "desc"
    });

    assert_roundtrip(typed, expected);
}
