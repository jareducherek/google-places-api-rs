use dotenv::dotenv;
use std::env;
use std::collections::HashSet;
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
    let input = "Museum of Contemporary Art Australia";
    let input_type = InputType::TextQuery;
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

    // Perform the place search
    match place_search_service.find_place(input, input_type, Some(fields), Some(language), Some(location_bias)).await {
        Ok(find_place) => {
            // Process and display the search result
            println!("{}", find_place.display());
        }
        Err(error) => {
            // Handle the error
            eprintln!("Error: {:?}", error);
        }
    }
}
