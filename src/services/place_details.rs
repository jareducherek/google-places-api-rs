use crate::client::GooglePlacesClient;
use crate::error::GooglePlacesError;
use crate::models::PlaceDetails;
pub struct PlaceDetailsService {
    client: GooglePlacesClient,
}

impl PlaceDetailsService {
    pub fn new(client: GooglePlacesClient) -> Self {
        PlaceDetailsService { client }
    }

    pub async fn get_place_details(
        &self,
        place_id: &str,
    ) -> Result<PlaceDetails, GooglePlacesError> {
        // Implementation goes here
        unimplemented!()
    }
}
