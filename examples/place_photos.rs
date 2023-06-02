use dotenv::dotenv;
use std::env;
use google_places_api::client::GooglePlacesClient;
use google_places_api::services::PlacePhotosService;

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
    let place_photos_service = PlacePhotosService::new(client);

    // Define photo_reference
    let photo_reference = "AZose0ninw6UOtG97IxGoKQN03Ar6JAKydIoTPfPT-KmMgqT6AAtAm90_RA65rV3imOU2Q6f34LcM8U_Trj5Y6hlo6l__0K7RpMvbdO77tvrAVLY0adHsBk80TUyTt2p7yH1_roKAdVCNCH1fFCJvSQVmIg0GoBEszWpTuUqr5wPaMnG_Ws-";
    
    // Perform the place details request
    match place_photos_service.get_photo_reference(photo_reference, None, None).await {
        Ok(img) => {
            let save_result = img.save("image.png");
        }
        Err(error) => {
            // Handle the error
            eprintln!("Error: {:?}", error);
        }
    }
}
