use dotenv::dotenv;
use std::env;
use std::collections::HashSet;
use google_places_api::client::GooglePlacesClient;
use google_places_api::models::place_search::{PlaceSearchStatus};
use google_places_api::models::constants::{PlaceSearchPlaceFields, Language, InputType, LocationBias};

#[tokio::test]
async fn test_find_place() {
    dotenv().ok();
    let api_key = env::var("GOOGLE_PLACES_API_KEY").expect("Please set the GOOGLE_PLACES_API_KEY environment variable");
    let client = GooglePlacesClient::new(&api_key, None, None);
    let input = "Mongolian Grill";
    let input_type: InputType = InputType::TextQuery;
    let fields: HashSet<PlaceSearchPlaceFields> = vec![
        PlaceSearchPlaceFields::BusinessStatus,
        PlaceSearchPlaceFields::FormattedAddress,
        PlaceSearchPlaceFields::Viewport,
        PlaceSearchPlaceFields::Location,
        PlaceSearchPlaceFields::Icon,
        PlaceSearchPlaceFields::IconMaskBaseUri,
        PlaceSearchPlaceFields::IconBackgroundColor,
        PlaceSearchPlaceFields::Name,
        PlaceSearchPlaceFields::Photo,
        PlaceSearchPlaceFields::PlaceId,
        PlaceSearchPlaceFields::PlusCode,
        PlaceSearchPlaceFields::PriceLevel,
        PlaceSearchPlaceFields::Rating,
    ].into_iter().collect();
    let language: Language = Language::En;
    let location_bias: LocationBias = LocationBias::Circular {
        radius: 10000,
        latitude: 33.85984846198168,
        longitude: 151.20907015422375,
    };

    match client.place_search_service.find_place(input, &input_type, Some(&fields), Some(&language), Some(&location_bias)).await {
        Ok(find_place) => {
            print!("{}", find_place.display());
            assert!(find_place.places.len() == find_place.total_results as usize);
            assert!(matches!(find_place.status, PlaceSearchStatus::Ok));
            for place in find_place.places.iter() {
                assert!(place.id.len() > 0);
                assert!(place.name.is_some());
                assert!(place.business_status.is_some());
                assert!(place.formatted_address.is_some());
                assert!(place.geometry.is_some());
                assert!(place.icon.is_some());
                assert!(place.icon_mask_base_uri.is_some());
                assert!(place.icon_background_color.is_some());
                assert!(place.photos.is_some());
                assert!(place.plus_code.is_some());
                assert!(place.price_level.is_some());
                assert!(place.rating.is_some());
            }
        }
        Err(error) => {
            panic!("Error in nearby search test {}", error);
        }
    }
}
