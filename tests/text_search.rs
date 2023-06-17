use dotenv::dotenv;
use std::env;
use google_places_api::client::GooglePlacesClient;
use google_places_api::services::PlaceSearchService;

#[tokio::main]
async fn test_text_search() {
    dotenv().ok();
    let api_key = env::var("GOOGLE_PLACES_API_KEY")
        .expect("Please set the GOOGLE_PLACES_API_KEY environment variable");
    let client = GooglePlacesClient::new(&api_key);
    let place_search_service = PlaceSearchService::new(client);
    let query = "restaurant";
    let radius = 5000; // 5000 meters radius
    match place_search_service.text_search(query, radius).await {
        Ok(search_result) => {
            println!("{}", search_result.display());
        }
        Err(error) => {
            panic!("Error in text seatch test {}", error);
        }
    }

}
