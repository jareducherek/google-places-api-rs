use crate::client::GooglePlacesClient;
use crate::error::GooglePlacesError;
use crate::models::{PlaceDetails, Language, ReviewSort};
use uuid::Uuid;
pub struct PlaceDetailsService {
    client: GooglePlacesClient,
    session_token: Option<String>,
}

impl PlaceDetailsService {
    pub fn new(client: GooglePlacesClient) -> Self {
        PlaceDetailsService { 
            client, 
            session_token: None,
        }
    }
    pub fn start_new_query(&mut self) {
        self.session_token = Some(Uuid::new_v4().to_string());
    }

    pub fn end_query(&mut self) {
        self.session_token = None;
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
        language: Option<Language>, //todo
        #[allow(unused)]
        region: Option<String>, //todo
        #[allow(unused)]
        review_no_translation: Option<bool>, //todo
        #[allow(unused)]
        review_sort: Option<ReviewSort>, //todo (wrong type, needs to be categorical variable? enum?)
        #[allow(unused)]
        session_token: Option<String>, //todo

    ) -> Result<PlaceDetails, GooglePlacesError> {//format for url might be wrong, need to test all cases
        let base_url = format!(
            "https://maps.googleapis.com/maps/api/place/details/json?place_id={}&key={}",
            place_id, self.client.get_api_key()
        );
        let mut url = base_url;
        // Fields
        if let Some(fields) = fields {
            let field_list: Vec<String> = fields.into_iter().map(|f| f.to_string()).collect();
            let field_string = field_list.join(",");
            url.push_str(&format!("&fields={}", field_string));
        }
        // Language
        if let Some(language) = language {
            url.push_str(&format!("&language={}", language.as_str()));
        }
        // Region
        if let Some(region) = region {
            url.push_str(&format!("&region={}", region));
        }
        // Review No Translation
        if let Some(review_no_translation) = review_no_translation {
            url.push_str(&format!("&reviews_no_translations={}", review_no_translation));
        }
        // Review Sort
        if let Some(review_sort) = review_sort {
            url.push_str(&format!("&sort={}", review_sort.to_string()));
        }
        // Session token
        if let Some(session_token) = session_token {
            if !session_token.is_empty() {
                url.push_str(&format!("&sessiontoken={}", session_token));
            }
        }

        let response: reqwest::Response = self.client.get_req_client().get(&url).send().await.unwrap();
        let body: String = response.text().await.unwrap();
        let search_result: PlaceDetails = serde_json::from_str(&body).unwrap();
        //self.end_query();
        
        Ok(search_result)
    }
}
