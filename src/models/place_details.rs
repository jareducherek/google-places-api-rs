use serde::{Deserialize, Serialize};
use crate::models::place::Place;
use crate::models::status::Status;

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
<<<<<<< Updated upstream
    pub status: String,
=======
    pub status: Status,
    pub info_messages: Option<Vec<String>>,
}

impl PlaceDetails {
    pub fn to_string(&self) -> String {
        let html_attributions = self.html_attributions.join(", ");
        let info_messages = self.info_messages.as_ref().map(|v| v.join(", ")).unwrap_or_default();
        format!("PlaceDetails {{ html_attributions: [{}], place: {}, status: {}, info_messages: [{}] }}", 
            html_attributions, self.place.to_string(), self.status.to_string(), info_messages)
    }
>>>>>>> Stashed changes
}


