use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Place {
    // Define the fields that represent a place
    #[serde(rename = "place_id")]
    pub id: String,
    pub name: String,
    #[serde(rename = "vicinity")]
    pub address: String,
    pub address_components: Option<Vec<AddressComponent>>,
    pub adr_address: Option<String>,
    pub business_status: Option<String>,
    pub formatted_address: Option<String>,
    pub formatted_phone_number: Option<String>,
    pub geometry: Option<Geometry>,
    pub icon: Option<String>,
    pub icon_background_color: Option<String>,
    pub icon_mask_base_uri: Option<String>,
    pub international_phone_number: Option<String>,
    pub opening_hours: Option<OpeningHours>,
    pub photos: Option<Vec<Photo>>,
    pub plus_code: Option<PlusCode>,
    pub rating: Option<f32>,
    pub reference: Option<String>,
    pub reviews: Option<Vec<Review>>,
    pub types: Option<Vec<String>>,
    pub url: Option<String>,
    pub utc_offset: Option<i32>,
    pub user_ratings_total: Option<i32>,
    pub website: Option<String>,
    // Add more fields as needed
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressComponent {
    pub long_name: Option<String>,
    pub short_name: Option<String>,
    pub types: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Geometry {
    pub location: Option<Location>,
    pub viewport: Option<Viewport>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub lat: Option<f64>,
    pub lng: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Viewport {
    pub northeast: Option<Location>,
    pub southwest: Option<Location>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpeningHours {
    pub open_now: Option<bool>,
    pub periods: Option<Vec<Period>>,
    pub weekday_text: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Period {
    pub open: Option<DayTime>,
    pub close: Option<DayTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DayTime {
    pub day: Option<i32>,
    pub time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Photo {
    pub html_attributions: Option<Vec<String>>,
    pub photo_reference: Option<String>,
    pub height: Option<u32>,
    pub width: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlusCode {
    pub compound_code: Option<String>,
    pub global_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Review {
    pub author_name: Option<String>,
    pub author_url: Option<String>,
    pub language: Option<String>,
    pub profile_photo_url: Option<String>,
    pub rating: Option<i32>,
    pub relative_time_description: Option<String>,
    pub text: Option<String>,
    pub time: Option<i64>,
}


