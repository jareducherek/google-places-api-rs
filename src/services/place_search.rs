use crate::services::RequestService;
use crate::error::GooglePlacesError;
use crate::models::place_search::{FindPlaceSearchResult, NearbySearchResult, TextSearchResult};
use crate::models::constants::{PlaceDataField, Language, InputType, LocationBias,PlaceTypes, RankBy};
use std::collections::HashSet;
use isocountry::CountryCode;
use urlencoding::encode;
use std::sync::Arc;

pub struct PlaceSearchService {
    client: Arc<RequestService>,
}

impl PlaceSearchService {
    pub fn new(client: Arc<RequestService>) -> Self {
        PlaceSearchService { client }
    }
    pub async fn nearby_search(
        &self,
        location: &(f64, f64),
        radius: &u32,
        keyword: Option<&str>,
        language: Option<&Language>,
        max_price: Option<&u32>,
        min_price: Option<&u32>,
        open_now: Option<&bool>,
        page_token: Option<&str>,
        place_type: Option<&HashSet<PlaceTypes>>,
    ) -> Result<NearbySearchResult, GooglePlacesError> {
        return
            self.full_nearby_search(
                location,
                Some(radius),
                keyword,
                language, 
                max_price,
                min_price,
                open_now,
                page_token,
                None,
                place_type,
            ).await
        
    }
    pub async fn nearby_search_rank_by_distance(
        &self,
        location: &(f64, f64),
        keyword: Option<&str>,
        language: Option<&Language>,
        max_price: Option<&u32>,
        min_price: Option<&u32>,
        open_now: Option<&bool>,
        page_token: Option<&str>,
        place_type: Option<&HashSet<PlaceTypes>>,
    ) -> Result<NearbySearchResult, GooglePlacesError> {
        return
            self.full_nearby_search(
                location,
                None,
                keyword,
                language,
                max_price,
                min_price,
                open_now,
                page_token,
                Some(&RankBy::Distance),
                place_type,
            ).await
    }

    async fn full_nearby_search(
        &self,
        location: &(f64, f64),
        radius: Option<&u32>,
        keyword: Option<&str>,
        language: Option<&Language>,
        max_price: Option<&u32>,
        min_price: Option<&u32>,
        open_now: Option<&bool>,
        page_token: Option<&str>,
        rank_by: Option<&RankBy>,
        place_type: Option<&HashSet<PlaceTypes>>,
    ) -> Result<NearbySearchResult, GooglePlacesError> {
        // Construct the request URL
        // "{}/maps/api/place/nearbysearch/json?keyword={}&location={}%2c{}&radius={}&key={}",
        let mut url = format!(
            "https://maps.googleapis.com/maps/api/place/nearbysearch/json?location={}%2C{}&key={}",
            location.0,location.1, self.client.get_api_key()
        );
        // Optional parameters
        if let Some(radius) = radius {
            url.push_str(&format!("&radius={}", radius));
        }
        if let Some(keyword) = keyword {
            url.push_str(&format!("&keyword={}", encode(&keyword)));
        }
        if let Some(language) = language {
            url.push_str(&format!("&language={}", language.as_str()));
        }
        if let Some(max_price) = max_price {
            url.push_str(&format!("&maxprice={}", max_price));
        }
        if let Some(min_price) = min_price {
            url.push_str(&format!("&minprice={}", min_price));
        }
        if let Some(open_now) = open_now {
            url.push_str(&format!("&opennow={}", open_now));
        }
        if let Some(page_token) = page_token {
            url.push_str(&format!("&pagetoken={}", encode(&page_token)));
        }
        if let Some(rank_by) = rank_by {
            url.push_str(&format!("&rankby={}", rank_by.as_str()));
        }
        if let Some(place_type) = place_type {
            let type_list: Vec<String> = place_type.into_iter().map(|p| String::from(p.as_str())).collect();
            let type_string = type_list.join(",");
            url.push_str(&format!("&type={}", type_string));
        }

        let response: reqwest::Response = match self.client.get_req_client().get(&url).send().await{
            Ok(response) => response,
            Err(e) => return Err(GooglePlacesError::HttpError(e)),
        };
        let body: String = match response.text().await{
            Ok(body) => body,
            Err(e) => return Err(GooglePlacesError::HttpError(e)),
        };
        let mut search_result: NearbySearchResult = match serde_json::from_str(&body){
            Ok(search_result) => search_result,
            Err(e) => return Err(GooglePlacesError::ParseError(e)),
        };
        search_result.calculate_total_results();
        Ok(search_result)
    }

    pub async fn find_place(
        &self,
        input: &str,
        input_type: &InputType,
        fields: Option<&HashSet<PlaceDataField>>,
        language: Option<&Language>,
        location_bias: Option<&LocationBias>,
    ) -> Result<FindPlaceSearchResult, GooglePlacesError>{
        
        let input_encoded = encode(input);
        let mut url = format!(
            "https://maps.googleapis.com/maps/api/place/findplacefromtext/json?input={}&inputtype={}&key={}",
            input_encoded, input_type.as_str(), self.client.get_api_key()
        );
        // Fields
        let all_fields = fields.cloned();
        if let Some(mut all_fields) = all_fields {
            all_fields.insert(PlaceDataField::PlaceId);
            let field_list: Vec<String> = all_fields.into_iter().map(|f| String::from(f.as_str())).collect();
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
        let response: reqwest::Response = match self.client.get_req_client().get(&url).send().await{
            Ok(response) => response,
            Err(e) => return Err(GooglePlacesError::HttpError(e)),
        };
        let body: String = match response.text().await{
            Ok(body) => body,
            Err(e) => return Err(GooglePlacesError::HttpError(e)),
        };
        let search_result: FindPlaceSearchResult = match serde_json::from_str(&body){
            Ok(search_result) => search_result,
            Err(e) => return Err(GooglePlacesError::ParseError(e)),
        };
        Ok(search_result)
    }

    pub async fn text_search(
        &self,
        query: &str,
        radius: &u32,
        language: Option<&Language>,
        location: Option<&(f64, f64)>,
        max_price: Option<&u32>,
        min_price: Option<&u32>,
        open_now: Option<&bool>,
        page_token: Option<&String>,
        region: Option<&CountryCode>,
        place_type: Option<&HashSet<PlaceTypes>>
    ) -> Result<TextSearchResult, GooglePlacesError>{
        let mut url = format!(
            "https://maps.googleapis.com/maps/api/place/textsearch/json?query={}&radius={}&key={}",
            query, radius, self.client.get_api_key()
        );
        // Optional parameters
        if let Some(language) = language {
            url.push_str(&format!("&language={}", language.as_str()));
        }
        if let Some(location) = location {
            url.push_str(&format!("&location={}%2C{}", location.0, location.1));
        }
        if let Some(max_price) = max_price {
            url.push_str(&format!("&maxprice={}", max_price));
        }
        if let Some(min_price) = min_price {
            url.push_str(&format!("&minprice={}", min_price));
        }
        if let Some(open_now) = open_now {
            url.push_str(&format!("&opennow={}", open_now));
        }
        if let Some(page_token) = page_token {
            url.push_str(&format!("&pagetoken={}", encode(&page_token)));
        }
        if let Some(region) = region {
            url.push_str(&format!("&region={}", region.alpha2()));
        }
        if let Some(place_type) = place_type {
            let type_list: Vec<String> = place_type.into_iter().map(|p| String::from(p.as_str())).collect();
            let type_string = type_list.join(",");
            url.push_str(&format!("&type={}", type_string));
        }

        let response: reqwest::Response = match self.client.get_req_client().get(&url).send().await{
            Ok(response) => response,
            Err(e) => return Err(GooglePlacesError::HttpError(e)),
        };
        let body: String = match response.text().await{
            Ok(body) => body,
            Err(e) => return Err(GooglePlacesError::HttpError(e)),
        };
        let mut search_result: TextSearchResult = match serde_json::from_str(&body){
            Ok(search_result) => search_result,
            Err(e) => return Err(GooglePlacesError::ParseError(e)),
        };
        search_result.calculate_total_results();
        Ok(search_result)
    }
}
