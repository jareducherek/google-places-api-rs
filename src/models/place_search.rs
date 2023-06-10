use serde::{Deserialize, Serialize};
use crate::models::place::Place;
use crate::models::status::Status;

#[derive(Debug, Serialize, Deserialize)]
pub struct NearbySearchResult {
    #[serde(rename = "results")]
    pub places: Vec<Place>,
<<<<<<< Updated upstream
    // Add more fields as needed

=======
    pub status: Status,
    pub error_message: Option<String>,
    pub info_messages: Option<Vec<String>>,
    pub next_page_token: Option<String>,
>>>>>>> Stashed changes
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
<<<<<<< Updated upstream
    pub status: String,
}

// impl<'de> serde::Deserialize<'de> for NearbySearchResult {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         let mut search_result: NearbySearchResult = serde::Deserialize::deserialize(deserializer)?;
//         // Set the total_results field based on the length of the places vector
//         search_result.total_results = search_result.places.len() as u32;
//         Ok(search_result)
//     }
// }
=======
    pub status: Status,
    pub error_message: Option<String>,
    pub info_messages: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextSearchResult {
    pub html_attributions: Vec<String>,
    #[serde(rename = "results")]
    pub places: Vec<Place>,
    pub status: Status,
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
            html_attributions, places, self.status.to_string(), self.error_message.as_ref().unwrap_or(&"".to_string()), info_messages, self.next_page_token.as_ref().unwrap_or(&"".to_string()), self.total_results)
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
            html_attributions, places, self.status.to_string(), self.error_message.as_ref().unwrap_or(&"".to_string()), info_messages, self.next_page_token.as_ref().unwrap_or(&"".to_string()))
    }
}
>>>>>>> Stashed changes
