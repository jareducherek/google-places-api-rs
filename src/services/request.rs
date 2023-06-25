use reqwest::Client;
use std::sync::Arc;

pub struct RequestService {
    pub req_client: Arc<Client>,
    api_key: String,
}

impl RequestService {
    pub fn new(api_key: &str) -> Self {
        let client = Arc::new(Client::new());
        RequestService {
            req_client: client,
            api_key: api_key.to_string(),
        }
    }

    pub fn get_req_client(&self) -> &Client {
        &self.req_client
    }

    pub fn get_api_key(&self) -> &str {
        &self.api_key
    }
    
}
