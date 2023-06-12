use dotenv::dotenv;
use std::env;
use google_places_api::client::GooglePlacesClient;
use google_places_api::services::PlaceDetailsService;

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
    let place_details_service = PlaceDetailsService::new(client);

    // Define place_id
    let place_id = "ChIJeSVts2QSkFQRyse0d8pWNa0";
    
    // Perform the place details request
    match place_details_service.get_place_details(place_id, None, None, None, None, None, None).await {
        Ok(place_details) => {
            // Display the place details
            println!("{}", place_details.display());
        }
        Err(error) => {
            // Handle the error
            eprintln!("Error: {:?}", error);
        }
    }
}
