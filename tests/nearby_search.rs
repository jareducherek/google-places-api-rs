use dotenv::dotenv;
use std::env;
use google_places_api::client::GooglePlacesClient;
use google_places_api::services::PlaceSearchService;

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
        Ok(search_result) => {
            println!("{}", search_result.display());
        }
        Err(error) => {
            panic!("Error in nearby search test {}", error);
        }
    }
}
