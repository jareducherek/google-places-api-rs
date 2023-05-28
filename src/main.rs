use waldo_google_api::{Places, Response};
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let api_key = match std::env::var("GOOGLE_API_KEY") {
        Ok(val) => val,
        Err(_) => {
            println!("GOOGLE_API env required");
            return;
        }
    };

    let place_id = "ChIJaccr2d4WsocRCJWGxAi8hWs";

    let places = &Places { api_key: &api_key };

    let res = match places.get_map_place(place_id) {
        Ok(b) => b,
        Err(e) => {
            println!("Error {:?}", e);
            return;
        }
    };

    match res {
        Response::Ok { result } => {
            println!("id                : {:?}", result.place_id);
            println!("name              : {:?}", result.name);
            println!("formatted_address : {:?}", result.formatted_address);
            println!("");

            println!("street_number : {}", result.street_number().unwrap_or(""),);
            println!("route         : {}", result.route().unwrap_or(""));
            println!("sublocality   : {}", result.sublocality().unwrap_or(""));
            println!("postal_code   : {}", result.postal_code().unwrap_or(""));
            println!("city          : {}", result.city().unwrap_or(""));
            println!("state         : {}", result.state().unwrap_or(""));
            println!("country       : {}", result.country().unwrap_or(""));

            // Retrieve a photo reference if available
            

            if let Some(photo) = result.photos.as_ref().and_then(|photos| photos.get(0)) {
                let photo_reference = photo.photo_reference.as_ref().map(|s| s.as_str()).unwrap_or("");
                let photo_url = places.get_place_image(photo_reference, Some(800)).unwrap_or_else(|e| {
                    println!("Failed to retrieve image URL: {:?}", e);
                    String::new()
                });

                println!("photo_reference: {}", photo_reference); // Added print statement
                println!("photo_url      : {}", photo_url);

                // Download the image data
                let mut file = std::fs::File::create("image.png").unwrap();
                reqwest::blocking::get(photo_url.as_str())
        .unwrap()
        .copy_to(&mut file)
        .unwrap();
                
            }
        }
        Response::ZeroResults => {
            println!("Zero results");
        }
        Response::InvalidRequest => {
            println!("Invalid Request");
        }
        Response::OverQueryLimit => {
            println!("Over Query Limit");
        }
        Response::RequestDenied { error_message } => {
            println!("Request Denied: {}", error_message);
        }
        Response::UnknownError => {
            println!("Unknown Error")
        }
    };
}