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
        fields: Option<Vec<String>>, //todo
        language: Option<String>, //todo
        region: Option<String>, //todo
        review_no_translation: Option<bool>, //todo
        review_sort: Option<String>, //todo (wrong type, needs to be categorical variable? enum?)
        session_token: Option<String>, //todo

    ) -> Result<PlaceDetails, GooglePlacesError> {
        // Implementation goes here
        //unimplemented!()

        let url = format!(
            "https://maps.googleapis.com/maps/api/place/details/json?place_id={}&key={}",
            place_id, self.client.get_api_key()
        );

        let response: reqwest::Response = self.client.get_req_client().get(&url).send().await.unwrap();
        let body: String = response.text().await.unwrap();
        let search_result: PlaceDetails = serde_json::from_str(&body).unwrap();
        Ok(search_result)
    }
}
