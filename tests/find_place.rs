use dotenv::dotenv;
use std::env;
use std::collections::HashSet;
use google_places_api::client::GooglePlacesClient;
use google_places_api::models::constants::{PlaceDataField, Language, InputType, LocationBias};

#[tokio::test]
async fn test_find_place() {
    dotenv().ok();
    let api_key = env::var("GOOGLE_PLACES_API_KEY").expect("Please set the GOOGLE_PLACES_API_KEY environment variable");
    let client = GooglePlacesClient::new(&api_key);
    let input = "Mongolian Grill";
    let input_type: InputType = InputType::TextQuery;
    let fields: HashSet<PlaceDataField> = vec![
        PlaceDataField::Name,
        PlaceDataField::Rating,
        PlaceDataField::FormattedAddress,

    ].into_iter().collect();
    let language: Language = Language::En;
    let location_bias: LocationBias = LocationBias::Circular {
        radius: 10000,
        latitude: 33.85984846198168,
        longitude: 151.20907015422375,
    };

    // Perform the request
    match client.place_search_service.find_place(input, &input_type, Some(&fields), Some(&language), Some(&location_bias)).await {
        Ok(search_result) => {
            println!("{}", search_result.display());
        }
        Err(error) => {
            panic!("Error in find place test {}", error);
        }
    }
}
