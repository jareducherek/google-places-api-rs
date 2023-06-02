use serde::{Deserialize, Serialize};
use crate::models::place::Place;

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaceDetails {
    pub html_attributions: Vec<String>,
    #[serde(rename = "result")]
    pub place: Place,
    pub status: String,
}


