use reqwest::Client;
use std::sync::Arc;

pub struct GooglePlacesClient {
    req_client: Arc<Client>,
    api_key: String,
}

impl GooglePlacesClient {
    pub fn new(api_key: &str) -> Self {
        let client = Arc::new(Client::new());
        GooglePlacesClient {
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
    

    // Implement methods for interacting with the Google Places API
    // For example, you could define methods for place search, place details retrieval, etc.
    // pub fn search_places(&self, query: &str, location: (f64, f64)) -> Result<PlaceSearchResult, GooglePlacesError> {
    //     // Implementation goes here
    // }

    // pub fn get_place_details(&self, place_id: &str) -> Result<PlaceDetails, GooglePlacesError> {
    //     // Implementation goes here
    // }
}
