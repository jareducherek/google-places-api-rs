use dotenv::dotenv;
use isocountry::CountryCode;
use std::{env,thread};
use std::time::Duration;
use std::collections::HashSet;
use google_places_api::client::GooglePlacesClient;
use google_places_api::models::place_search::{PlaceSearchStatus};
use google_places_api::models::constants::{PlaceTypes, Language};

#[tokio::test]
async fn test_text_search() {
    dotenv().ok();
    let api_key = env::var("GOOGLE_PLACES_API_KEY")
        .expect("Please set the GOOGLE_PLACES_API_KEY environment variable");
    let client = GooglePlacesClient::new(&api_key, None, None);
    let query: &str = "restaurant";
    let radius:u32 = 20000; // 5000 meters radius
    let language = Language::En;
    let location: (f64, f64) = (37.7749, -122.4194); // San Francisco coordinates
    let max_price = 4;
    let min_price = 1;
    let open_now = false;
    let region: CountryCode = CountryCode::USA;
    let place_types: HashSet<PlaceTypes> = vec![
        PlaceTypes::Restaurant,
        PlaceTypes::Establishment,
        PlaceTypes::Food,

    ].into_iter().collect();
    let page_token;
    match client.place_search_service.text_search(query, &radius, Some(&language), Some(&location), Some(&max_price), Some(&min_price), Some(&open_now), None, Some(&region), Some(&place_types)).await {
        Ok(text_search) => {
            assert!(text_search.places.len() > 0);
            assert!(text_search.places.len() == text_search.total_results as usize);
            assert!(text_search.next_page_token.is_some());
            assert!(matches!(text_search.status, PlaceSearchStatus::Ok));
            page_token = text_search.next_page_token;
            for place in text_search.places.iter() {
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
            panic!("Error in text search test {}", error);
        }
    }
    thread::sleep(Duration::from_secs(2));
    match client.place_search_service.text_search_next_page(page_token.unwrap().as_str()).await {
        Ok(text_search) => {
            assert!(text_search.places.len() > 0);
            assert!(text_search.places.len() == text_search.total_results as usize);
            assert!(text_search.next_page_token.is_some());
            assert!(matches!(text_search.status, PlaceSearchStatus::Ok));
            for place in text_search.places.iter() {
                assert!(place.id.len() > 0);
                assert!(place.name.is_some());
                assert!(place.business_status.is_some());
                // assert!(place.formatted_address.is_some()); // this is sometimes null
                assert!(place.geometry.is_some());
                assert!(place.icon.is_some());
                assert!(place.icon_mask_base_uri.is_some());
                assert!(place.icon_background_color.is_some());
                // assert!(place.photos.is_some()); // this is sometimes null
                assert!(place.plus_code.is_some());
                // assert!(place.price_level.is_some()); // this is sometimes null
                assert!(place.rating.is_some());
            }
        }
        Err(error) => {
            panic!("Error in text search test {}", error);
        }
    }

}
