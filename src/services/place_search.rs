use crate::client::GooglePlacesClient;
use crate::error::GooglePlacesError;
use crate::models::{Place, NearbySearchResult, FindPlaceSearchResult};

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
    ) -> Result<NearbySearchResult, GooglePlacesError> {
        // Construct the request URL
        // "{}/maps/api/place/nearbysearch/json?keyword={}&location={}%2c{}&radius={}&key={}",
        let url = format!(
            "https://maps.googleapis.com/maps/api/place/nearbysearch/json?location={},{}&radius={}&keyword={}&key={}",
            location.0, location.1, radius, query, self.client.get_api_key()
        );

        let response: reqwest::Response = self.client.get_req_client().get(&url).send().await.unwrap();
        let body: String = response.text().await.unwrap();
        let mut search_result: NearbySearchResult = serde_json::from_str(&body).unwrap();
        search_result.calculate_total_results();
        Ok(search_result)
    }

    pub async fn find_place(//todo panicked at 'called `Result::unwrap()` on an `Err` value: waldo\src\services\place_search.rs:47:80
        &self,
        input: &str,
        input_type: &str,
    ) -> Result<FindPlaceSearchResult, GooglePlacesError>{

        let url = format!(
            "https://maps.googleapis.com/maps/api/place/findplacefromtext/json?input={}&inputtype={}&key={}",
            input, input_type, self.client.get_api_key()
        );

        let response: reqwest::Response = self.client.get_req_client().get(&url).send().await.unwrap();
        let body: String = response.text().await.unwrap();
        let search_result: FindPlaceSearchResult = serde_json::from_str(&body).unwrap();
        Ok(search_result)
    }
    
}
