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

    // pub async fn get_place_details(&self, place_id: &str) -> Result<PlaceDetails, GooglePlacesError> {
    //     return self.get_place_details(place_id, None, None, None, None, None, None);
    // }

    pub async fn get_place_details(
        &self,
        place_id: &str,
        #[allow(unused)]
        fields: Option<Vec<String>>, //todo
        #[allow(unused)]
        language: Option<String>, //todo
        #[allow(unused)]
        region: Option<String>, //todo
        #[allow(unused)]
        review_no_translation: Option<bool>, //todo
        #[allow(unused)]
        review_sort: Option<String>, //todo (wrong type, needs to be categorical variable? enum?)
        #[allow(unused)]
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
