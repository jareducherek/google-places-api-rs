use dotenv::dotenv;
use std::env;
use std::collections::HashSet;
use isocountry::CountryCode;
use relative_path::RelativePath;
use std::path::Path;
use google_places_api::client::GooglePlacesClient;
use google_places_api::models::constants::{PlaceDetailsPlaceFields, Language, ReviewSort};

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
    let output_path = RelativePath::new("examples/outputs/place_details.json").to_path(root_dir);
    println!("{}", output_path.display());

    // Define the request parameters
    let place_id = "ChIJN1t_tDeuEmsRUsoyG83frY4";
    let fields: HashSet<PlaceDetailsPlaceFields> = vec![
        PlaceDetailsPlaceFields::Name,
        PlaceDetailsPlaceFields::Rating,
        PlaceDetailsPlaceFields::FormattedPhoneNumber,
        PlaceDetailsPlaceFields::Vicinity,
    ].into_iter().collect();
    let language: Language = Language::Es;
    let region: CountryCode = CountryCode::USA;
    let review_no_translation: bool = false;
    let review_sort: ReviewSort = ReviewSort::Newest;

    // Perform the request
    match client.place_details_service.get_place_details(place_id, Some(&fields), Some(&language), Some(&region), Some(&review_no_translation), Some(&review_sort), None).await {
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
