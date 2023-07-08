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
    pub fn new(api_key: &str, max_requests: Option<u64>, per: Option<Duration> ) -> Self {
        let client = Arc::new(Client::new());
        let max_requests = match max_requests {
            Some(max_requests) => max_requests,
            None => 1000,
        };
        let new_per = match per {
            Some(per) => per,
            None => Duration::from_millis(100),
        };
        RequestService {
            req_client: client,
            api_key: api_key.to_string(),
            rate_limiter: RateLimiter::new(max_requests, new_per),
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

#[cfg(test)]
mod test {
    use super::*;
    use std::time::{Duration, Instant};

    #[tokio::test]
    async fn test_service_api_key() {
        let service = RequestService::new("test", Some(1), Some(Duration::from_millis(100)));
        assert_eq!(service.get_api_key(), "test");
    }

    #[tokio::test]
    async fn test_service_max_requests() {
        let service = RequestService::new("test", Some(1), Some(Duration::from_millis(100)));
        service.get_response("https://www.google.com").await.unwrap();
        let response = service.get_response("https://www.google.com").await;
        assert!(response.is_err());
    }

    #[tokio::test]
    async fn test_service_period_limit() {
        let service = RequestService::new("test", Some(2), Some(Duration::from_millis(250)));
        let now = Instant::now();
        service.get_response("https://www.google.com").await.unwrap();
        service.get_response("https://www.google.com").await.unwrap();
        assert!(now.elapsed().as_millis() >= 500);
    }
}
