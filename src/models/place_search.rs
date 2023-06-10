use serde::{Deserialize, Serialize};
use crate::models::place::Place;

#[derive(Debug, Serialize, Deserialize)]
pub struct NearbySearchResult {
    pub html_attributions: Vec<String>,
    #[serde(rename = "results")]
    pub places: Vec<Place>,
    pub status: PlaceSearchStatus,
    pub error_message: Option<String>,
    pub info_messages: Option<Vec<String>>,
    pub next_page_token: Option<String>,
    #[serde(skip)]
    pub total_results: u32,
}

impl NearbySearchResult {
    pub fn calculate_total_results(&mut self) {
        self.total_results = self.places.len() as u32;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FindPlaceSearchResult {
    #[serde(rename = "candidates")]
    pub results: Vec<Place>,
    pub status: PlaceSearchStatus,
    pub error_message: Option<String>,
    pub info_messages: Option<Vec<String>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TextSearchResult {
    pub html_attributions: Vec<String>,
    #[serde(rename = "results")]
    pub places: Vec<Place>,
    pub status: PlaceSearchStatus,
    pub error_message: Option<String>,
    pub info_messages: Option<Vec<String>>,
    pub next_page_token: Option<String>,
}

impl NearbySearchResult {
    pub fn to_string(&self) -> String {
        let places = self.places.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(", ");
        let html_attributions = self.html_attributions.join(", ");
        let info_messages = self.info_messages.as_ref().map(|v| v.join(", ")).unwrap_or_default();
        format!("NearbySearchResult {{ html_attributions: [{}], places: [{}], status: {}, error_message: {}, info_messages: [{}], next_page_token: {}, total_results: {} }}", 
            html_attributions, places, self.status.to_string(), 
            self.error_message.as_ref().unwrap_or(&"".to_string()), 
            info_messages, self.next_page_token.as_ref().unwrap_or(&"".to_string()), self.total_results)
    }
}

impl FindPlaceSearchResult {
    pub fn to_string(&self) -> String {
        let results = self.results.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(", ");
        let info_messages = self.info_messages.as_ref().map(|v| v.join(", ")).unwrap_or_default();
        format!("FindPlaceSearchResult {{ results: [{}], status: {}, error_message: {}, info_messages: [{}] }}", 
            results, self.status.to_string(), self.error_message.as_ref().unwrap_or(&"".to_string()), info_messages)
    }
}

impl TextSearchResult {
    pub fn to_string(&self) -> String {
        let places = self.places.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(", ");
        let html_attributions = self.html_attributions.join(", ");
        let info_messages = self.info_messages.as_ref().map(|v| v.join(", ")).unwrap_or_default();
        format!("TextSearchResult {{ html_attributions: [{}], places: [{}], status: {}, error_message: {}, info_messages: [{}], next_page_token: {} }}", 
            html_attributions, places, self.status.to_string(),
            self.error_message.as_ref().unwrap_or(&"".to_string()),
            info_messages, self.next_page_token.as_ref().unwrap_or(&"".to_string()))
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub enum PlaceSearchStatus {
    #[serde(rename = "OK")]
    Ok,
    #[serde(rename = "ZERO_RESULTS")]
    ZeroResults,
    #[serde(rename = "INVALID_REQUEST")]
    InvalidRequest,
    #[serde(rename = "OVER_QUERY_LIMIT")]
    OverQueryLimit,
    #[serde(rename = "REQUEST_DENIED")]
    RequestDenied,
    #[serde(rename = "UNKNOWN_ERROR")]
    UnknownError,
}

impl PlaceSearchStatus {
    pub fn to_string(&self) -> &str {
        match self {
            PlaceSearchStatus::Ok => "OK",
            PlaceSearchStatus::ZeroResults => "ZERO_RESULTS",
            PlaceSearchStatus::OverQueryLimit => "OVER_QUERY_LIMIT",
            PlaceSearchStatus::RequestDenied => "REQUEST_DENIED",
            PlaceSearchStatus::InvalidRequest => "INVALID_REQUEST",
            PlaceSearchStatus::UnknownError => "UNKNOWN_ERROR",
        }
    }
}