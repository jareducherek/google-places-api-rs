use reqwest::Client;
use crate::error::GooglePlacesError;
use tokio::time::{Duration};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use crate::utils::RateLimiter;

pub struct RequestService {
    req_client: Arc<Client>,
    api_key: String,
    rate_limiter: RateLimiter,
    total_requests: AtomicU64,
}

impl RequestService {
    pub fn new(api_key: &str) -> Self {
        let client = Arc::new(Client::new());
        RequestService {
            req_client: client,
            api_key: api_key.to_string(),
            rate_limiter: RateLimiter::new(50, Duration::from_secs(1)),
            total_requests: AtomicU64::new(0),
        }
    }
    pub async fn get_response(&self, url: &str) -> Result<reqwest::Response, GooglePlacesError> {
        match self.rate_limiter.acquire(&self.total_requests).await {
            Ok(_) => {},
            Err(e) => return Err(e),
        };
        let response = match self.req_client.get(url).send().await {
            Ok(response) => Ok(response),
            Err(e) => Err(GooglePlacesError::HttpError(e)),
        };
        if response.is_ok() {
            self.total_requests.fetch_add(1, Ordering::SeqCst);
        }
        return response
    }

    pub fn get_api_key(&self) -> &str {
        &self.api_key
    }
    
}
