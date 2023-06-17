use dotenv::dotenv;
use std::env;
use relative_path::{RelativePath, RelativePathBuf};
use std::path::{Path, PathBuf};
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

    // Output path to view the corresponding json
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let output_path = RelativePath::new("examples/outputs/text_search.json").to_path(root_dir);

    // Define the request parameters
    let query = "restaurant";
    let radius = 5000; // 5000 meters radius

    // Perform the request
    match place_search_service.text_search(query, radius).await {
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

}
