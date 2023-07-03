use dotenv::dotenv;
use std::env;
use std::collections::HashSet;
use relative_path::RelativePath;
use std::path::Path;
use google_places_api::client::GooglePlacesClient;
use google_places_api::models::constants::{Language, PlaceTypes};

#[tokio::main]
async fn main() {
    // Load environment variables from the .env file
    dotenv().ok();

    // Retrieve the API key from the environment variable
    let api_key = env::var("GOOGLE_PLACES_API_KEY")
        .expect("Please set the GOOGLE_PLACES_API_KEY environment variable");

    // Create a Google Places client
    let client = GooglePlacesClient::new(&api_key);

    // Output path to view the corresponding json
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let output_path = RelativePath::new("examples/outputs/nearby_search.json").to_path(root_dir);

    // Define the request parameters
    let keyword = "restaurant";
    let location = (37.7749, -122.4194); // San Francisco coordinates
    // Optional request parameters
    let radius = 5000; // 5000 meters radius
    let language: Language = Language::En;
    let max_price = 4;
    let min_price = 1;
    let open_now = false;
    let place_set: HashSet<PlaceTypes> = vec![
        PlaceTypes::Restaurant,
        PlaceTypes::Establishment,
        PlaceTypes::Food,
        ].into_iter().collect();
    let place_types = Some(&place_set);;

    // Perform the request
    match client.place_search_service.nearby_search_rank_by_distance(&location, Some(keyword), Some(&language), Some(&max_price), Some(&min_price), Some(&open_now), None, place_types).await {
        Ok(search_result) => {
            println!("{}", search_result.display());
            std::fs::write(
                output_path,
                serde_json::to_string_pretty(&search_result).unwrap(),
            );
        }
        Err(error) => {
            eprintln!("Error: {:?}", error);
        }
    }

    // Perform the place search with default args
    match client.place_search_service.nearby_search(&location, &radius, Some(&keyword), Some(&language), Some(&max_price), Some(&min_price), Some(&open_now), None, Some(&place_types)).await {
       Ok(nearby_search) => {
           // Process and display the search result
           println!("{}", nearby_search.display());
       }
       Err(error) => {
           // Handle the error
           eprintln!("Error: {:?}", error);
       }
    }
}
