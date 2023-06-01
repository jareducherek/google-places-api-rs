use crate::client::GooglePlacesClient;
use crate::error::GooglePlacesError;
use crate::models::PlaceSearchResult;

pub struct PlaceSearchService {
    client: GooglePlacesClient,
}

impl PlaceSearchService {
    pub fn new(client: GooglePlacesClient) -> Self {
        PlaceSearchService { client }
    }

    pub async fn nearby_search(
        &self,
        query: &str,
        location: (f64, f64),
        radius: u32,
    ) -> Result<PlaceSearchResult, GooglePlacesError> {
        // Implementation goes here
        // TODO this should return a vector of 
        unimplemented!()
    }
}
