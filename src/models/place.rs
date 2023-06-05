use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct Place {
    // Define the fields that represent a place
    #[serde(rename = "place_id")]
    pub id: String,
    pub name: Option<String>,
    #[serde(rename = "vicinity")]
    pub address: Option<String>,
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
    pub curbside_pickup: Option<bool>,
    pub current_opening_hours: Option<PlaceOpeningHours>,
    pub delivery: Option<bool>,
    pub dine_in: Option<bool>,
    pub editorial_summary: Option<PlaceEditorialSummary>,
    pub permanently_closed: Option<bool>,
    pub serves_beer: Option<bool>,
    pub serves_breakfast: Option<bool>,
    pub serves_brunch: Option<bool>,
    pub serves_dinner: Option<bool>,
    pub serves_lunch: Option<bool>,
    pub serves_vegetarian_food: Option<bool>,
    pub serves_wine: Option<bool>,
    pub takeout: Option<bool>,
    pub secondary_opening_hours: Option<Vec<PlaceOpeningHours>>,
    pub vicinity: Option<String>,
    pub wheelchair_accessible_entrance: Option<bool>,
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
    pub height: Option<i32>,
    pub html_attributions: Option<Vec<String>>,
    pub photo_reference: Option<String>,
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

// Additional structs

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaceOpeningHours {
    pub open_now: Option<bool>,
    pub periods: Option<Vec<OpeningHoursPeriod>>,
    pub weekday_text: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpeningHoursPeriod {
    pub open: Option<OpeningHoursTime>,
    pub close: Option<OpeningHoursTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpeningHoursTime {
    pub day: Option<i32>,
    pub time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaceEditorialSummary {
    pub body: Option<String>,
    pub attribution: Option<String>,
}


impl Place {
    pub fn to_string(&self) -> String {
        let json_value: Value = json!(self);
        let cleaned_value = remove_empty_fields(&json_value);
        cleaned_value.to_string()
    }
}

fn remove_empty_fields(value: &Value) -> Value {
    match value {
        Value::Object(obj) => {
            let cleaned_fields: serde_json::Map<String, Value> = obj
                .iter()
                .filter(|(_, v)| !v.is_null())
                .map(|(k, v)| (k.clone(), remove_empty_fields(v)))
                .collect();

            Value::Object(cleaned_fields)
        }
        Value::Array(arr) => {
            let cleaned_array: Vec<Value> = arr
                .iter()
                .map(|v| remove_empty_fields(v))
                .collect();

            Value::Array(cleaned_array)
        }
        _ => value.clone(),
    }
}