use serde::{Deserialize, Serialize};
use crate::models::place::Place;

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaceDetails {
    pub html_attributions: Vec<String>,
    #[serde(rename = "result")]
    pub place: Place,
    pub status: PlaceDetailsStatus,
    pub info_messages: Option<Vec<String>>,
}

impl PlaceDetails {
    pub fn to_string(&self) -> String {
        let html_attributions = self.html_attributions.join(", ");
        let info_messages = self.info_messages.as_ref().map(|v| v.join(", ")).unwrap_or_default();
        format!("PlaceDetails {{ html_attributions: [{}], place: {}, status: {}, info_messages: [{}] }}", 
            html_attributions, self.place.to_string(), self.status.to_string(), info_messages)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PlaceDetailsStatus {
    #[serde(rename = "OK")]
    Ok,
    #[serde(rename = "ZERO_RESULTS")]
    ZeroResults,
    #[serde(rename = "OVER_QUERY_LIMIT")]
    OverQueryLimit,
    #[serde(rename = "REQUEST_DENIED")]
    RequestDenied,
    #[serde(rename = "INVALID_REQUEST")]
    InvalidRequest,
    #[serde(rename = "UNKNOWN_ERROR")]
    UnknownError,
    #[serde(rename = "NOT_FOUND")]
    NotFound,
}

impl PlaceDetailsStatus {
    pub fn to_string(&self) -> &str {
        match self {
            PlaceDetailsStatus::Ok => "OK",
            PlaceDetailsStatus::ZeroResults => "ZERO_RESULTS",
            PlaceDetailsStatus::OverQueryLimit => "OVER_QUERY_LIMIT",
            PlaceDetailsStatus::RequestDenied => "REQUEST_DENIED",
            PlaceDetailsStatus::InvalidRequest => "INVALID_REQUEST",
            PlaceDetailsStatus::UnknownError => "UNKNOWN_ERROR",
            PlaceDetailsStatus::NotFound => "NOT_FOUND",
        }
    }
}

pub enum ReviewSort {
    MostRelevant,
    Newest,
}
impl ToString for ReviewSort {
    fn to_string(&self) -> String {
        match *self {
            ReviewSort::MostRelevant => "most_relevant".to_string(),
            ReviewSort::Newest => "newest".to_string(),
        }
    }
}
