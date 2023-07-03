use thiserror::Error;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;
use std::io::Error as IOError;

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
    #[error("IO Error")]
    ReaderError(IOError),
    #[error("Invalid Input Parameter: {0}")]
    ParamError(String),
    // Add more custom error variants as needed
}
