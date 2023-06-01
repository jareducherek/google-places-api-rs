use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Place {
    // Define the fields that represent a place
    pub id: String,
    pub name: String,
    pub address: String,
    // Add more fields as needed
}




