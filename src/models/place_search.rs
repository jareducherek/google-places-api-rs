use serde::{Deserialize, Serialize};

use crate::models::Place;

#[derive(Debug, Serialize, Deserialize)]
pub struct NearbySearchResult {
    pub places: Vec<Place>,
    pub total_results: usize,
    // Add more fields as needed
}
