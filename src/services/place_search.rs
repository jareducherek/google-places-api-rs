use crate::services::RequestService;
use crate::error::GooglePlacesError;
use crate::models::place_search::{FindPlaceSearchResult, NearbySearchResult, TextSearchResult};
use crate::models::constants::{PlaceDataField, Language, InputType, LocationBias, PlaceTypes, RankBy};
use std::collections::HashSet;
use isocountry::CountryCode;
use urlencoding::encode;
use std::sync::Arc;
use serde::de::DeserializeOwned;

pub struct PlaceSearchService {
    client: Arc<RequestService>,
}
mod nearby_search {
    use super::*;
    pub fn build_nearby_search(
        api_key: &str,
        location: &(f64, f64),
        radius: Option<&u32>,
        keyword: Option<&str>,
        language: Option<&Language>,
        max_price: Option<&u32>,
        min_price: Option<&u32>,
        open_now: Option<&bool>,
        page_token: Option<&str>,
        rank_by: Option<&RankBy>,
        place_types: Option<&HashSet<PlaceTypes>>,
    ) -> Result<String, GooglePlacesError> {
        // Construct the request URL
        // "{}/maps/api/place/nearbysearch/json?keyword={}&location={}%2c{}&radius={}&key={}",
        let mut url = format!(
            "https://maps.googleapis.com/maps/api/place/nearbysearch/json?location={}%2C{}&key={}",
            location.0,location.1, api_key
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
            if *max_price > 4 {
                return Err(GooglePlacesError::ParamError("Max price has to be less than 4".to_string()))
            }
            url.push_str(&format!("&maxprice={}", max_price));
        }
        if let Some(min_price) = min_price {
            if *min_price > 4 {
                return Err(GooglePlacesError::ParamError("Min price has to be less than 4".to_string()))
            }
            url.push_str(&format!("&minprice={}", min_price));
        }
        if min_price > max_price {
            return Err(GooglePlacesError::ParamError("Min price has to be less than Max price".to_string()))
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
        if let Some(place_types) = place_types {
            let type_list: Vec<String> = place_types.into_iter().map(|p| String::from(p.as_str())).collect();
            let type_string = type_list.join(",");
            url.push_str(&format!("&type={}", type_string));
        }
        Ok(url)
    }
    pub fn process_nearby_search(body: &str) -> Result<NearbySearchResult, GooglePlacesError>{
        let mut search_result: NearbySearchResult = match serde_json::from_str(&body){
            Ok(search_result) => search_result,
            Err(e) => return Err(GooglePlacesError::ParseError(e)),
        };
        search_result.calculate_total_results();
        Ok(search_result)
    }
}
mod text_search {
    use super::*;
    pub fn process_text_search(body: &str) -> Result<TextSearchResult, GooglePlacesError> {
        let mut search_result: TextSearchResult = match serde_json::from_str(&body){
            Ok(search_result) => search_result,
            Err(e) => return Err(GooglePlacesError::ParseError(e)),
        };
        search_result.calculate_total_results();
        Ok(search_result)
    }
    pub fn build_text_search(
        api_key: &str,
        query: &str,
        radius: &u32,
        language: Option<&Language>,
        location: Option<&(f64, f64)>,
        max_price: Option<&u32>,
        min_price: Option<&u32>,
        open_now: Option<&bool>,
        page_token: Option<&str>,
        region: Option<&CountryCode>,
        place_types: Option<&HashSet<PlaceTypes>>
    ) -> Result<String, GooglePlacesError>{
        let mut url = format!(
            "https://maps.googleapis.com/maps/api/place/textsearch/json?query={}&radius={}&key={}",
            query, radius, api_key
        );
        // Optional parameters
        if let Some(language) = language {
            url.push_str(&format!("&language={}", language.as_str()));
        }
        if let Some(location) = location {
            url.push_str(&format!("&location={}%2C{}", location.0, location.1));
        }
        if let Some(max_price) = max_price {
            if *max_price > 4 {
                return Err(GooglePlacesError::ParamError("Max price has to be less than 4".to_string()))
            }
            url.push_str(&format!("&maxprice={}", max_price));
        }
        if let Some(min_price) = min_price {
            if *min_price > 4 {
                return Err(GooglePlacesError::ParamError("Min price has to be less than 4".to_string()))
            }
            url.push_str(&format!("&minprice={}", min_price));
        }
        if min_price > max_price {
            return Err(GooglePlacesError::ParamError("Min price has to be less than Max price".to_string()))
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
        if let Some(place_types) = place_types {
            let type_list: Vec<String> = place_types.into_iter().map(|p| String::from(p.as_str())).collect();
            let type_string = type_list.join(",");
            url.push_str(&format!("&type={}", type_string));
        }
        Ok(url)
    }
}
mod find_place {
    use super::*;
    pub fn process_find_place(body: &str) -> Result<FindPlaceSearchResult, GooglePlacesError> {
        let search_result: FindPlaceSearchResult = match serde_json::from_str(&body){
            Ok(search_result) => search_result,
            Err(e) => return Err(GooglePlacesError::ParseError(e)),
        };
        Ok(search_result)
    }
    pub fn build_find_place(
        api_key: &str,
        input: &str,
        input_type: &InputType,
        fields: Option<&HashSet<PlaceDataField>>,
        language: Option<&Language>,
        location_bias: Option<&LocationBias>,
    ) -> Result<String, GooglePlacesError>{
        
        let input_encoded = encode(input);
        let mut url = format!(
            "https://maps.googleapis.com/maps/api/place/findplacefromtext/json?input={}&inputtype={}&key={}",
            input_encoded, input_type.as_str(), api_key
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
        Ok(url)
    }
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
        place_types: Option<&HashSet<PlaceTypes>>,
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
                place_types,
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
        place_types: Option<&HashSet<PlaceTypes>>,
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
                place_types,
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
        place_types: Option<&HashSet<PlaceTypes>>,
    ) -> Result<NearbySearchResult, GooglePlacesError> {
    
        let url = nearby_search::build_nearby_search(self.client.get_api_key(), location, radius, keyword, 
        language, max_price, min_price, open_now, page_token, rank_by, place_types)?;
        let response: reqwest::Response = match self.client.get_req_client().get(url).send().await{
            Ok(response) => response,
            Err(e) => return Err(GooglePlacesError::HttpError(e)),
        };
        let body: String = match response.text().await{
            Ok(body) => body,
            Err(e) => return Err(GooglePlacesError::HttpError(e)),
        };
        Ok(nearby_search::process_nearby_search(&body)?)
    }

    pub async fn find_place(
        &self,
        input: &str,
        input_type: &InputType,
        fields: Option<&HashSet<PlaceDataField>>,
        language: Option<&Language>,
        location_bias: Option<&LocationBias>,
    ) -> Result<FindPlaceSearchResult, GooglePlacesError>{       
        let url = find_place::build_find_place(self.client.get_api_key(), input, input_type, fields, language, location_bias)?;
        let response: reqwest::Response = match self.client.get_req_client().get(url).send().await{
            Ok(response) => response,
            Err(e) => return Err(GooglePlacesError::HttpError(e)),
        };
        let body: String = match response.text().await{
            Ok(body) => body,
            Err(e) => return Err(GooglePlacesError::HttpError(e)),
        };
        Ok(find_place::process_find_place(&body)?)
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
        page_token: Option<&str>,
        region: Option<&CountryCode>,
        place_types: Option<&HashSet<PlaceTypes>>
    ) -> Result<TextSearchResult, GooglePlacesError>{
        let url = text_search::build_text_search(self.client.get_api_key(), query, radius, language, location, max_price, 
        min_price, open_now, page_token, region, place_types)?;
        let response: reqwest::Response = match self.client.get_req_client().get(url).send().await{
            Ok(response) => response,
            Err(e) => return Err(GooglePlacesError::HttpError(e)),
        };
        let body: String = match response.text().await{
            Ok(body) => body,
            Err(e) => return Err(GooglePlacesError::HttpError(e)),
        };
        Ok(text_search::process_text_search(&body)?)
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_build_nearby_search() {
        let api_key = "12345";
        let location = &(0.0, 0.0);
        let radius = Some(&1000);
        let keyword = Some("restaurant");
        let language = Some(&Language::En);
        let max_price = Some(&2);
        let min_price = Some(&1);
        let open_now = Some(&true);
        let page_token = Some("token123");
        let rank_by = Some(&RankBy::Distance);
        let place_set: HashSet<PlaceTypes> = vec![
        PlaceTypes::Restaurant,
        ].into_iter().collect();
        let place_types = Some(&place_set);
        let url = nearby_search::build_nearby_search(api_key, location, radius, keyword, language, max_price, min_price, open_now, page_token, rank_by, place_types).unwrap();
        let actual_url = "https://maps.googleapis.com/maps/api/place/nearbysearch/json?location=0%2C0&key=12345&radius=1000&keyword=restaurant&language=en&maxprice=2&minprice=1&opennow=true&pagetoken=token123&rankby=distance&type=restaurant".to_string();
        assert_eq!(url, actual_url);
    }

    #[test]
    fn test_build_text_search() {
        let api_key = "12345";
        let query = "query";
        let radius = &(1000 as u32);
        let language = Some(&Language::En);
        let location = Some(&(0.0, 0.0));
        let max_price = Some(&2);
        let min_price = Some(&1);
        let open_now = Some(&true);
        let page_token = Some("token123");
        let region = Some(&CountryCode::USA);
        let place_set: HashSet<PlaceTypes> = vec![
        PlaceTypes::Restaurant,
        ].into_iter().collect();
        let place_types = Some(&place_set);
        let url = text_search::build_text_search(api_key, query, radius, language, location, max_price, min_price, open_now, page_token, region, place_types).unwrap();
        let actual_url = "https://maps.googleapis.com/maps/api/place/textsearch/json?query=query&radius=1000&key=12345&language=en&location=0%2C0&maxprice=2&minprice=1&opennow=true&pagetoken=token123&region=US&type=restaurant".to_string();
        assert_eq!(url, actual_url);
    }

    #[test]
    fn test_build_find_place() {
        let api_key = "12345";
        let input = "Mongolian Grill";
        let input_type = &InputType::TextQuery;
        let fields_set: HashSet<PlaceDataField> = vec![
            PlaceDataField::Name,
        ].into_iter().collect();
        let fields = Some(&fields_set);
        let language: Option<&Language> = Some(&Language::En);
        let location_bias = Some(&LocationBias::Circular {
            radius: 10000,
            latitude: 33.85984846198168,
            longitude: 151.20907015422375,
        });
        let url = find_place::build_find_place(api_key, input, input_type, fields, language, location_bias).unwrap();
        println!("{}", url);
        let actual_url = "https://maps.googleapis.com/maps/api/place/findplacefromtext/json?input=Mongolian%20Grill&inputtype=textquery&key=12345&fields=place_id,name&language=en&locationbias=circle:10000@33.85984846198168,151.20907015422375".to_string();
        assert_eq!(url.replace("name,place_id", "place_id,name"), actual_url);
    }
}