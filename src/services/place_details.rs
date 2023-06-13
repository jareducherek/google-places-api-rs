use crate::client::GooglePlacesClient;
use crate::error::GooglePlacesError;
use crate::models::PlaceDetails;
use crate::models::constants::{PlaceDataField, Language, ReviewSort};
use isocountry::CountryCode;
use std::collections::HashSet;
use uuid::Uuid;
pub struct PlaceDetailsService {
    client: GooglePlacesClient,
    session_token: Option<String>,
}

impl PlaceDetailsService {
    /// Retrieves detailed information about a place based on its place ID.

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

    ///
    /// This function makes a request to the Google Places API to fetch the details of a place
    /// identified by the provided place ID. The retrieved information includes the name, address,
    /// phone number, opening hours, photos, reviews, and other details of the place.
    ///
    /// # Arguments
    ///
    /// * `place_id` - The unique identifier of the place.
    /// * `fields` - Optional. A list of specific fields to retrieve for the place. If not provided,
    ///              all available fields will be returned.
    /// * `language` - Optional. The preferred language for the returned place details. If provided,
    ///                the API will attempt to provide the information in the specified language.
    /// * `region` - Optional. The region code (ccTLD) to bias the results towards a specific country
    ///              or region. This helps to narrow down the search to a specific geographical area.
    /// * `review_no_translation` - Optional. Specifies whether to disable translation of reviews.
    ///                             If set to `true`, reviews will be returned in their original language.
    ///                             If omitted or set to `false`, reviews will be translated to the
    ///                             preferred language specified or determined by the Accept-Language header.
    /// * `review_sort` - Optional. The sorting method to use when returning reviews. Can be set to
    ///                   "most_relevant" (default) or "newest". For "most_relevant", reviews are sorted
    ///                   by relevance and biased towards the preferred language. For "newest", reviews
    ///                   are sorted in chronological order.
    /// * `session_token` - Optional. A session token that identifies an autocomplete session for billing
    ///                     purposes. Each session can have multiple queries and one place selection.
    ///                     If not provided, each request is billed separately.
    ///
    /// # Returns
    ///
    /// The retrieved place details as a `PlaceDetails` struct if successful, or an error of type
    /// `GooglePlacesError` if the API request fails or the place details cannot be retrieved.
    ///
    pub async fn get_place_details(
        &self,
        place_id: &str,
        fields: Option<HashSet<PlaceDataField>>, 
        language: Option<Language>, 
        region: Option<CountryCode>,
        review_no_translation: Option<bool>,
        review_sort: Option<ReviewSort>, 
        session_token: Option<String>,

    ) -> Result<PlaceDetails, GooglePlacesError> {//format for url might be wrong, need to test all cases
        let base_url = format!(
            "https://maps.googleapis.com/maps/api/place/details/json?place_id={}&key={}",
            place_id, self.client.get_api_key()
        );
        let mut url = base_url;
        // Fields
        if let Some(mut fields) = fields {
            fields.insert(PlaceDataField::PlaceId);
            let field_list: Vec<String> = fields.into_iter().map(|f| String::from(f.as_str())).collect();
            let field_string = field_list.join(",");
            url.push_str(&format!("&fields={}", field_string));
        }
        // Language
        if let Some(language) = language {
            url.push_str(&format!("&language={}", language.as_str()));
        }
        // Region
        if let Some(region) = region {
            url.push_str(&format!("&region={}", region.alpha2()));
        }
        // Review No Translation
        if let Some(review_no_translation) = review_no_translation {
            url.push_str(&format!("&reviews_no_translations={}", review_no_translation));
        }
        // Review Sort
        if let Some(review_sort) = review_sort {
            url.push_str(&format!("&sort={}", review_sort.as_str()));
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
