use dotenv::dotenv;
use std::env;
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

    // Define the search query, location, and radius
    let input = "restaurant";
    let input_type = "textquery"; 

    // Perform the place search
    match place_search_service.find_place(input, input_type).await {
        Ok(search_result) => {
            // Process and display the search result
            println!("Candidates:");
            for mut candidate in search_result.candidates {
                println!("Name: {}", candidate.name);
                println!("Address: {}", candidate.formatted_address.unwrap_or_else(|| String::from("N/A")));
                if let Some(geometry) = candidate.geometry.take() {
                    if let Some(location) = geometry.location {
                        println!("Location: {}, {}", location.lat.unwrap_or(0.0), location.lng.unwrap_or(0.0));
                    }
                    if let Some(viewport) = geometry.viewport {
                        if let Some(northeast) = viewport.northeast {
                            println!("Viewport NE: {}, {}", northeast.lat.unwrap_or(0.0), northeast.lng.unwrap_or(0.0));
                        }
                        if let Some(southwest) = viewport.southwest {
                            println!("Viewport SW: {}, {}", southwest.lat.unwrap_or(0.0), southwest.lng.unwrap_or(0.0));
                        }
                    }
                }
                if let Some(opening_hours) = candidate.opening_hours {
                    println!("Opening Hours: Open now - {}", opening_hours.open_now.unwrap_or(false));
                }
                println!("Rating: {}", candidate.rating.unwrap_or(0.0));
                println!("");
            }
            println!("Status: {}", search_result.status);
        }
        Err(error) => {
            // Handle the error
            eprintln!("Error: {:?}", error);
        }
    }
    
}
