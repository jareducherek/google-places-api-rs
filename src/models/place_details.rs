use serde::{Deserialize, Serialize};
use crate::models::place::Place;

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaceDetails {
    pub html_attributions: Vec<String>,
    #[serde(rename = "result")]
    pub place: Place,
    pub status: String,
    pub info_messages: Option<Vec<String>>,
}

impl PlaceDetails {
    pub fn to_string(&self) -> String {
        let html_attributions = self.html_attributions.join(", ");
        let info_messages = self.info_messages.as_ref().map(|v| v.join(", ")).unwrap_or_default();
        format!("PlaceDetails {{ html_attributions: [{}], place: {}, status: {}, info_messages: [{}] }}", 
            html_attributions, self.place.to_string(), self.status, info_messages)
    }
}


