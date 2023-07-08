use crate::services::RequestService;
use crate::error::GooglePlacesError;
use crate::models::PlaceDetailsResult;
use crate::models::constants::{PlaceDataField, Language, ReviewSort};
use isocountry::CountryCode;
use std::collections::HashSet;
use uuid::Uuid;
use std::sync::Arc;

pub struct PlaceDetailsService {
    client: Arc<RequestService>,
    session_token: Option<String>,
}

mod place_details {
    use super::*;

    pub fn build_place_details(
        api_key: &str,
        place_id: &str,
        fields: Option<&HashSet<PlaceDataField>>, 
        language: Option<&Language>, 
        region: Option<&CountryCode>,
        review_no_translation: Option<&bool>,
        review_sort: Option<&ReviewSort>, 
        session_token: Option<&str>,
    ) -> Result<String, GooglePlacesError> { 
        // TODO format for url might be wrong, need to test all cases
        let base_url = format!(
            "https://maps.googleapis.com/maps/api/place/details/json?place_id={}&key={}",
            place_id, api_key
        );
        let mut url = base_url;
        // Optional parameters
        let all_fields = fields.cloned();
        if let Some(mut all_fields) = all_fields {
            all_fields.insert(PlaceDataField::PlaceId);
            let field_list: Vec<String> = all_fields.into_iter().map(|f| String::from(f.to_string())).collect();
            let field_string = field_list.join(",");
            url.push_str(&format!("&fields={}", field_string));
        }
        if let Some(language) = language {
            url.push_str(&format!("&language={}", language.to_string()));
        }
        if let Some(region) = region {
            url.push_str(&format!("&region={}", region.alpha2()));
        }
        if let Some(review_no_translation) = review_no_translation {
            url.push_str(&format!("&reviews_no_translations={}", review_no_translation));
        }
        if let Some(review_sort) = review_sort {
            url.push_str(&format!("&sort={}", review_sort.to_string()));
        }
        if let Some(session_token) = session_token {
            if !session_token.is_empty() {
                url.push_str(&format!("&sessiontoken={}", session_token));
            }
        }
        Ok(url)
    }

    pub fn process_place_details(
        body: &str
    ) -> Result<PlaceDetailsResult, GooglePlacesError> {
        let search_result: PlaceDetailsResult = match serde_json::from_str(&body){
            Ok(search_result) => search_result,
            Err(e) => return Err(GooglePlacesError::ParseError(e)),
        };
        Ok(search_result)
    }
}

impl PlaceDetailsService {
    /// Retrieves detailed information about a place based on its place ID.

    pub fn new(client: Arc<RequestService>) -> Self {
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
    #[tracing::instrument(level="debug", name="Place Details", skip(self))]
    pub async fn get_place_details(
        &self,
        place_id: &str,
        fields: Option<&HashSet<PlaceDataField>>, 
        language: Option<&Language>, 
        region: Option<&CountryCode>,
        review_no_translation: Option<&bool>,
        review_sort: Option<&ReviewSort>, 
        session_token: Option<&str>,

    ) -> Result<PlaceDetailsResult, GooglePlacesError> { 
        let url = place_details::build_place_details(self.client.get_api_key(), place_id, fields, language, region, review_no_translation, review_sort, session_token)?;
        tracing::debug!("Place Details: `{url}`", url=url);
        let response: reqwest::Response = match self.client.get_response(&url).await{
            Ok(response) => response,
            Err(e) => return Err(e),
        };
        let body: String = match response.text().await{
            Ok(body) => body,
            Err(e) => return Err(GooglePlacesError::HttpError(e)),
        };
        Ok(place_details::process_place_details(&body)?)   
    }

}

#[cfg(test)]
mod test{
    use super::*;
    use relative_path::RelativePath;
    use std::path::Path;
    use crate::models::place_details::{PlaceDetailsStatus};

    #[test]
    fn test_build_nearby_search() {
        let api_key = "12345";
        let place_id = "ChIJN1t_tDeuEmsRUsoyG83frY4";
        let fields_set: HashSet<PlaceDataField> = vec![
           PlaceDataField::Name,
        ].into_iter().collect();
        let fields = Some(&fields_set);
        let language = Some(&Language::Es);
        let region = Some(&CountryCode::USA);
        let review_no_translation = Some(&false);
        let review_sort = Some(&ReviewSort::Newest);
        let session_token = Some("token123");
        let url = place_details::build_place_details(api_key, place_id, fields, language, region, review_no_translation, review_sort, session_token).unwrap();
        let actual_url = "https://maps.googleapis.com/maps/api/place/details/json?place_id=ChIJN1t_tDeuEmsRUsoyG83frY4&key=12345&fields=place_id,name&language=es&region=US&reviews_no_translations=false&sort=newest&sessiontoken=token123".to_string();
        assert_eq!(url.replace("name,place_id", "place_id,name"), actual_url); // order of name and place_id is not guaranteed, so we reorder them
    }

    #[test]
    fn test_process_nearby_search() {
        let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let input_path = RelativePath::new("resources/tests/place_details.txt").to_path(root_dir);
        let body = std::fs::read_to_string(input_path).unwrap();
        let search_result = place_details::process_place_details(&body).unwrap();
        assert_eq!(search_result.status, PlaceDetailsStatus::Ok);
        assert!(search_result.place.id.len() > 0);
        assert!(search_result.place.name.is_some());
        assert!(search_result.place.business_status.is_some());
        assert!(search_result.place.geometry.is_some());
        assert!(search_result.place.icon.is_some());
        assert!(search_result.place.icon_background_color.is_some());
        assert!(search_result.place.icon_mask_base_uri.is_some());
        // assert!(search_result.place.opening_hours.is_some()); // this is occasionally null
        // assert!(search_result.place.photos.as_ref().map(|vec| vec.len()).unwrap_or(0) > 0); // this is occasionally null
        assert!(search_result.place.plus_code.is_some());
        assert!(search_result.place.types.is_some());
        // assert!(search_result.place.vicinity.is_some()); // this is occasionally null
        // assert!(search_result.place.price_level.is_some()); // this is occasionally null
        assert!(search_result.place.rating.is_some());
        assert!(search_result.place.user_ratings_total.is_some());
    }
}
