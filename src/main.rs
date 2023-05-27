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

    //let args: Vec<String> = std::env::args().collect();
    //if args.len() < 2 {
    //    println!("place_id arg required");
    //    return;
    //}

    let place_id = "ChIJATaCWGU3zDER32m__CAwDyY";

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