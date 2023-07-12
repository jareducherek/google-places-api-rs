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
    let output_path = RelativePath::new("examples/outputs/place_photos.png").to_path(root_dir);

    // Define the request parameters
    let photo_reference = "AZose0ninw6UOtG97IxGoKQN03Ar6JAKydIoTPfPT-KmMgqT6AAtAm90_RA65rV3imOU2Q6f34LcM8U_Trj5Y6hlo6l__0K7RpMvbdO77tvrAVLY0adHsBk80TUyTt2p7yH1_roKAdVCNCH1fFCJvSQVmIg0GoBEszWpTuUqr5wPaMnG_Ws-";
    
    // Perform the request
    match client.place_photos_service.get_photo_reference(photo_reference, None, None).await {
        Ok(search_result) => {
            let save_result = search_result.save(output_path);
            
        }
        Err(error) => {
            tracing::error!("Error: {:?}", error);
        }
    }
}
