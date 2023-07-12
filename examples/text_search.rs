use dotenv::dotenv;
use std::env;
use relative_path::RelativePath;
use std::path::Path;
use google_places_api::client::GooglePlacesClient;

#[tokio::main]
async fn main() {
    // Load environment variables from the .env file
    dotenv().ok();

    // Retrieve the API key from the environment variable
    let api_key = env::var("GOOGLE_PLACES_API_KEY")
        .expect("Please set the GOOGLE_PLACES_API_KEY environment variable");

    // Create a Google Places client
    let client = GooglePlacesClient::new(&api_key, Some(5), None);

    // Create a tracing subscriber for logging purposes
    let sub = tracing_subscriber::FmtSubscriber::builder().with_max_level(tracing::Level::INFO).finish();
    let sub_guard = tracing::subscriber::set_default(sub);

    // Output path to view the corresponding json
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let output_path = RelativePath::new("examples/outputs/text_search.json").to_path(root_dir);

    // Define the request parameters
    let query = "restaurant";
    let radius = 5000; // 5000 meters radius

    // Perform the request
    match client.place_search_service.text_search(query, &radius, None, None, None, None, None, None, None, None).await {
        Ok(search_result) => {
            tracing::info!("{}", search_result.display());
            std::fs::write(
                output_path,
                serde_json::to_string_pretty(&search_result).unwrap(),
            );
        }
        Err(error) => {
            tracing::error!("Error: {:?}", error);
        }
    }

}
