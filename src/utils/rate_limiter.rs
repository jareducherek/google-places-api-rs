use crate::error::GooglePlacesError;
use tokio::time::{sleep, Duration, Instant};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU64, Ordering};

pub struct RateLimiter {
    max_requests: u64,
    per: Duration,
    last_request: Arc<Mutex<Instant>>,
}

impl RateLimiter {
    pub fn new(max_requests: u64, per: Duration) -> Self {
        RateLimiter {
            max_requests,
            per,
            last_request: Arc::new(Mutex::new(Instant::now())),
        }
    }

    pub async fn acquire(&self, current_requests: &AtomicU64) -> Result<(), GooglePlacesError> {
        let mut last_request = self.last_request.lock().unwrap();
        let elapsed = last_request.elapsed();

        if current_requests.load(Ordering::SeqCst) >= self.max_requests {
            return Err(GooglePlacesError::RateLimitError)
        }

        if elapsed < self.per && self.max_requests > 0 {
            // Sleep until the next permissible request
            let sleep_duration = self.per - elapsed;
            sleep(sleep_duration).await;
        }

        *last_request = Instant::now();
        Ok(())
    }
}