use dotenv::dotenv;
use std::env;
use google_places_api::client::GooglePlacesClient;
use google_places_api::services::PlaceSearchService;

#[tokio::test]
async fn test_find_place() {
    dotenv().ok();
    let api_key = env::var("GOOGLE_PLACES_API_KEY").expect("Please set the GOOGLE_PLACES_API_KEY environment variable");
    let client = GooglePlacesClient::new(&api_key);
    let place_search_service = PlaceSearchService::new(client);
    let input = "Mongolian Grill";
    let input_type = "textquery";

    match place_search_service.find_place(input, input_type).await {
        Ok(search_result) => {
            println!("{}", search_result.display());
        }
        Err(error) => {
            panic!("Error in find place test {}", error);
        }
    }
}
