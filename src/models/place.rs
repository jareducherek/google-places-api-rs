use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Place {
    // Define the fields that represent a place
    #[serde(rename = "place_id")]
    pub id: String,
    pub name: String,
    #[serde(rename = "vicinity")]
    pub address: String,
    // Add more fields as needed
}


//todo details needs to inherit from place
#[derive(Debug, Serialize, Deserialize)]
pub struct FullPlace { // TODO update this to be combined with the above struct
    address_components: Vec<AddressComponent>,
    adr_address: String,
    business_status: String,
    formatted_address: String,
    formatted_phone_number: String,
    geometry: Geometry,
    icon: String,
    icon_background_color: String,
    icon_mask_base_uri: String,
    international_phone_number: String,
    name: String,
    opening_hours: OpeningHours,
    photos: Vec<Photo>,
    place_id: String,
    plus_code: PlusCode,
    rating: f32,
    reference: String,
    reviews: Vec<Review>,
    types: Vec<String>,
    url: String,
    utc_offset: i32,
    user_ratings_total: i32,
    website: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AddressComponent {
    long_name: String,
    short_name: String,
    types: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Geometry {
    location: Location,
    viewport: Viewport,
}

#[derive(Debug, Serialize, Deserialize)]
struct Location {
    lat: f64,
    lng: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Viewport {
    northeast: Location,
    southwest: Location,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpeningHours {
    open_now: bool,
    periods: Vec<Period>,
    weekday_text: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Period {
    open: DayTime,
    close: DayTime,
}

#[derive(Debug, Serialize, Deserialize)]
struct DayTime {
    day: i32,
    time: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Photo {
    height: i32,
    html_attributions: Vec<String>,
    photo_reference: String,
    width: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct PlusCode {
    compound_code: String,
    global_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Review {
    author_name: String,
    author_url: String,
    language: String,
    profile_photo_url: String,
    rating: i32,
    relative_time_description: String,
    text: String,
    time: i64,
}



