use serde::{Deserialize, Serialize};
use crate::models::place::Place;

//#[derive(Debug, Serialize, Deserialize)]
//pub struct PlaceDetails {
//    // Define the fields that represent place details
//    pub id: String,
//    pub name: String,
//    pub address: String,
//    pub phone_number: Option<String>,
//    // Add more fields as needed
//}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaceDetails {
    pub html_attributions: Vec<String>,
    #[serde(rename = "result")]
    pub place: Place,
    pub status: String,
}


