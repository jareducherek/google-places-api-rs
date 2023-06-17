use dotenv::dotenv;
use std::env;
use google_places_api::client::GooglePlacesClient;
use google_places_api::services::PlaceSearchService;
use google_places_api::models::place_search::{PlaceSearchStatus};

#[tokio::test]
async fn test_nearby_search() {
    dotenv().ok();
    let api_key = env::var("GOOGLE_PLACES_API_KEY").expect("Please set the GOOGLE_PLACES_API_KEY environment variable");
    let client = GooglePlacesClient::new(&api_key);
    let place_search_service = PlaceSearchService::new(client);
    let query = "restaurant";
    let location = (37.7749, -122.4194); // San Francisco coordinates
    let radius = 5000; // 5000 meters radius

    match place_search_service.nearby_search(query, location, radius).await {
        Ok(nearby_search) => {
            assert!(nearby_search.places.len() > 0);
            assert!(nearby_search.next_page_token.is_some());
            assert!(matches!(nearby_search.status, PlaceSearchStatus::Ok));
            for place in nearby_search.places.iter() {
                assert!(place.id.len() > 0);
                assert!(place.name.is_some());
                assert!(place.business_status.is_some());
                assert!(place.geometry.is_some());
                assert!(place.icon.is_some());
                assert!(place.icon_background_color.is_some());
                assert!(place.icon_mask_base_uri.is_some());
                assert!(place.opening_hours.is_some());
                assert!(place.photos.as_ref().map(|vec| vec.len()).unwrap_or(0) > 0);
                assert!(place.plus_code.is_some());
                assert!(place.types.is_some());
                assert!(place.vicinity.is_some());
                // assert!(place.price_level.is_some()); // this is occasionally null
                assert!(place.rating.is_some());
                assert!(place.user_ratings_total.is_some());
            }

        }
        Err(error) => {
            panic!("Error in nearby search test {}", error);
        }
    }
}
