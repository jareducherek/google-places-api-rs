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




