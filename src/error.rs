use thiserror::Error;

#[derive(Debug, Error)]
pub enum GooglePlacesError {
    #[error("API request failed: {0}")]
    ApiError(String),
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("Invalid API key")]
    InvalidApiKey,
    // Add more custom error variants as needed
}