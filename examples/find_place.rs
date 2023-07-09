use dotenv::dotenv;
use std::env;
use std::collections::HashSet;
use relative_path::RelativePath;
use std::path::Path;
use google_places_api::client::GooglePlacesClient;
use google_places_api::models::constants::{InputType, Language, LocationBias, PlaceSearchPlaceFields};

#[tokio::main]
async fn main() {
    // Load environment variables from the .env file
    dotenv().ok();

    // Retrieve the API key from the environment variable
    let api_key = env::var("GOOGLE_PLACES_API_KEY").expect("Please set the GOOGLE_PLACES_API_KEY environment variable");

    // Create a Google Places client
    let client = GooglePlacesClient::new(&api_key);

    // Create a tracing subscriber for logging purposes
    let sub = tracing_subscriber::FmtSubscriber::builder().with_max_level(tracing::Level::INFO).finish();
    let sub_guard = tracing::subscriber::set_default(sub);

    // Output path to view the corresponding json
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let output_path = RelativePath::new("examples/outputs/find_place.json").to_path(root_dir);
    
    // Define the request parameters
    let input = "Mongolian Grill";
    let input_type: InputType = InputType::TextQuery;
    let fields: HashSet<PlaceSearchPlaceFields> = vec![
        PlaceSearchPlaceFields::Name,
        PlaceSearchPlaceFields::Rating,
        PlaceSearchPlaceFields::FormattedAddress,

    ].into_iter().collect();
    let language: Language = Language::En;
    let location_bias: LocationBias = LocationBias::Circular {
        radius: 10000,
        latitude: 33.85984846198168,
        longitude: 151.20907015422375,
    };

    // Perform the request
    match client.place_search_service.find_place(input, &input_type, Some(&fields), Some(&language), Some(&location_bias)).await {
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
