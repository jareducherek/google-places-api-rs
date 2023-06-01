use serde::{Deserialize, Serialize};
use crate::models::Place;

#[derive(Debug, Serialize, Deserialize)]
pub struct NearbySearchResult {
    #[serde(rename = "results")]
    pub places: Vec<Place>,
    // Add more fields as needed

    #[serde(skip)]
    pub total_results: u32,
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