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

    // Define the location, and radius
    let location = (-33.8670522, 151.1957362); // San Francisco coordinates
    let radius = 1500; // 5000 meters radius

    //Optional Args
    let keyword = "cruise";
    let language: Language = Language::En;
    let max_price = 4;
    let min_price = 1;
    let open_now = false;
    //page_token = None;
    let place_types: HashSet<PlaceTypes> = vec![
        PlaceTypes::Restaurant,
        PlaceTypes::Establishment,
        PlaceTypes::Food,

    ].into_iter().collect();

    // Perform the place search with rankby distance
    match place_search_service.nearby_search_rank_by_distance(location, Some(keyword), Some(language), Some(max_price), Some(min_price), Some(open_now), None, Some(place_types)).await {
        Ok(nearby_search) => {
            // Process and display the search result
            println!("{}", nearby_search.display());
        }
        Err(error) => {
            // Handle the error
            eprintln!("Error: {:?}", error);
        }
    }

    // Perform the place search with default args
    //match place_search_service.nearby_search(location, radius, Some(keyword), Some(language), Some(max_price), Some(min_price), Some(open_now), None, Some(place_types)).await {
    //    Ok(nearby_search) => {
    //        // Process and display the search result
    //        println!("{}", nearby_search.display());
    //    }
    //    Err(error) => {
    //        // Handle the error
    //        eprintln!("Error: {:?}", error);
    //    }
    //}
}
