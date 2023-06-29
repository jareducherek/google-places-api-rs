use thiserror::Error;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;

#[derive(Debug, Error)]
pub enum GooglePlacesError {
    #[error("API request failed: {0}")]
    ApiError(String),
    #[error("HTTP error: {0}")]
    HttpError(ReqwestError),    
    #[error("Invalid API key")]
    InvalidApiKey,
    #[error("Parse Error: {0}")]
    ParseError(SerdeError),
    // Add more custom error variants as needed
}
