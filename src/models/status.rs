use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
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

impl Status {
    pub fn to_string(&self) -> &str {
        match self {
            Status::Ok => "OK",
            Status::ZeroResults => "ZERO_RESULTS",
            Status::OverQueryLimit => "OVER_QUERY_LIMIT",
            Status::RequestDenied => "REQUEST_DENIED",
            Status::InvalidRequest => "INVALID_REQUEST",
            Status::UnknownError => "UNKNOWN_ERROR",
            Status::NotFound => "NOT_FOUND",
        }
    }
}
