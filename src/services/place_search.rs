use crate::client::GooglePlacesClient;
use crate::error::GooglePlacesError;
use crate::models::place_search::{FindPlaceSearchResult, NearbySearchResult, TextSearchResult};
use crate::models::constants::{PlaceDataField, Language, InputType, LocationBias,PlaceTypes, RankBy};
use std::collections::HashSet;
use urlencoding::encode;

pub struct PlaceSearchService {
    client: GooglePlacesClient,
}

impl PlaceSearchService {
    pub fn new(client: GooglePlacesClient) -> Self {
        PlaceSearchService { client }
    }

    pub async fn nearby_search(
        &self,
        location: (f64, f64),
        radius: u32,
        keyword: Option<String>,
        language: Option<Language>,
        max_price: Option<u32>,
        min_price: Option<u32>,
        open_now: Option<bool>,
        page_token: Option<String>,
        rank_by: Option<RankBy>,
        place_type: Option<HashSet<PlaceTypes>>,
    ) -> Result<NearbySearchResult, GooglePlacesError> {
        // Construct the request URL
        // "{}/maps/api/place/nearbysearch/json?keyword={}&location={}%2c{}&radius={}&key={}",
        let mut url = format!(
            "https://maps.googleapis.com/maps/api/place/nearbysearch/json?location={},{}&radius={}&key={}",
            location.0, location.1, radius, self.client.get_api_key()
        );

        // Keyword
        if let Some(keyword) = keyword {
            url.push_str(&format!("&keyword={}", encode(&keyword)));
        }

        // Language
        if let Some(language) = language {
            url.push_str(&format!("&language={}", language.as_str()));
        }

        // Max price
        if let Some(max_price) = max_price {
            url.push_str(&format!("&maxprice={}", max_price));
        }

        // Min price
        if let Some(min_price) = min_price {
            url.push_str(&format!("&minprice={}", min_price));
        }

        // Open now
        if let Some(open_now) = open_now {
            url.push_str(&format!("&opennow={}", open_now));
        }

        // Page token
        if let Some(page_token) = page_token {
            url.push_str(&format!("&pagetoken={}", encode(&page_token)));
        }

        // Rank by
        if let Some(rank_by) = rank_by {
            url.push_str(&format!("&rankby={}", rank_by.as_str()));
        }

        // Place type
        if let Some(place_type) = place_type {
            let type_list: Vec<String> = place_type.into_iter().map(|p| String::from(p.as_str())).collect();
            let type_string = type_list.join(",");
            url.push_str(&format!("&type={}", type_string));
        }

        let response: reqwest::Response = self.client.get_req_client().get(&url).send().await.unwrap();
        let body: String = response.text().await.unwrap();
        let mut search_result: NearbySearchResult = serde_json::from_str(&body).unwrap();
        search_result.calculate_total_results();
        Ok(search_result)
    }

    pub async fn find_place(
        &self,
        input: &str,
        input_type: InputType,
        fields: Option<HashSet<PlaceDataField>>,
        language: Option<Language>,
        location_bias: Option<LocationBias>,
    ) -> Result<FindPlaceSearchResult, GooglePlacesError>{
        
        let input_encoded = encode(input);
        let mut url = format!(
            "https://maps.googleapis.com/maps/api/place/findplacefromtext/json?input={}&inputtype={}&key={}",
            input_encoded, input_type.as_str(), self.client.get_api_key()
        );
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
        
        // Location Bias
        if let Some(location_bias) = location_bias {
            url.push_str(&format!("&locationbias={}", location_bias.as_str()));
        }

        let response: reqwest::Response = self.client.get_req_client().get(&url).send().await.unwrap();
        let body: String = response.text().await.unwrap();
        let search_result: FindPlaceSearchResult = serde_json::from_str(&body).unwrap();
        Ok(search_result)
    }

    pub async fn text_search(
        &self,
        query: &str,
        radius: u32,
    ) -> Result<TextSearchResult, GooglePlacesError>{
        let url = format!(
            "https://maps.googleapis.com/maps/api/place/textsearch/json?query={}&radius={}&key={}",
            query, radius, self.client.get_api_key()
        );
        let response: reqwest::Response = self.client.get_req_client().get(&url).send().await.unwrap();
        let body: String = response.text().await.unwrap();
        let search_result: TextSearchResult = serde_json::from_str(&body).unwrap();
        Ok(search_result)
    }
}
