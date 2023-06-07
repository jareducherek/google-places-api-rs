use dotenv::dotenv;
use std::env;
use google_places_api::client::GooglePlacesClient;
use google_places_api::services::PlaceSearchService;

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
    let input_type = "textquery";

    // Perform the place search
    match place_search_service.find_place(input, input_type).await {
        Ok(search_result) => {
            // Process and display the search result
            for result in search_result.results {
                println!("Place:\n{}", result.to_string());
            }
            println!("Status: {}", search_result.status);
        }
        Err(error) => {
            // Handle the error
            eprintln!("Error: {:?}", error);
        }
    }
}
