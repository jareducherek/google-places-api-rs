use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaceDetails {
    // Define the fields that represent place details
    pub id: String,
    pub name: String,
    pub address: String,
    pub phone_number: Option<String>,
    // Add more fields as needed
}