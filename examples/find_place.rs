use dotenv::dotenv;
use std::env;
use google_places_api::client::GooglePlacesClient;
use google_places_api::services::PlaceSearchService;
use google_places_api::models::constants::*;


#[tokio::main]
async fn main() {
    // Load environment variables from the .env file
    dotenv().ok();

    // Retrieve the API key from the environment variable
    let api_key = env::var("GOOGLE_PLACES_API_KEY")
        .expect("Please set the GOOGLE_PLACES_API_KEY environment variable");

    // Create a Google Places client
    let client = GooglePlacesClient::new(&api_key);

    // Create a PlaceSearchService instance
    let place_search_service = PlaceSearchService::new(client);

    // Define the search query, location, and radius
    let input = "Mongolian Grill";
    let input_type = InputType::TextQuery;

    // Perform the place search
    match place_search_service.find_place(input, input_type, None, None, None).await {
        Ok(find_place) => {
            // Process and display the search result
            for result in find_place.results {
                println!("Place:\n{}", result.display());
            }
            println!("Status: {}", find_place.status.as_str());
        }
        Err(error) => {
            // Handle the error
            eprintln!("Error: {:?}", error);
        }
    }
}
