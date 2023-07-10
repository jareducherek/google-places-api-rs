use dotenv::dotenv;
use std::env;
use std::collections::HashSet;
use isocountry::CountryCode;
use google_places_api::client::GooglePlacesClient;
use google_places_api::models::constants::{Language, ReviewSort, PlaceDetailsPlaceFields};
use google_places_api::models::place_details::{PlaceDetailsStatus};

#[tokio::test]
async fn test_place_details() {
    dotenv().ok();
    let api_key = env::var("GOOGLE_PLACES_API_KEY").expect("Please set the GOOGLE_PLACES_API_KEY environment variable");
    let client = GooglePlacesClient::new(&api_key);
    let place_id = "ChIJN1t_tDeuEmsRUsoyG83frY4";
    let language: Language = Language::Es;
    let region: CountryCode = CountryCode::USA;
    let review_no_translation: bool = false;
    let review_sort: ReviewSort = ReviewSort::Newest;

    match client.place_details_service.get_place_details(place_id, None, Some(&language), Some(&region), Some(&review_no_translation), Some(&review_sort), None).await {
        Ok(place_details) => {
            // Display the place details
            assert!(place_details.place.address_components.map(|vec| vec.len()).unwrap_or(0) > 0);
            assert!(place_details.place.adr_address.is_some());
            assert!(place_details.place.business_status.is_some());
            let opening_hours = place_details.place.current_opening_hours;
            if let Some(opening_hours) = opening_hours {
                assert!(opening_hours.periods.is_some())
            } else {
                panic!("Opening hours not available");
            }
            assert!(place_details.place.formatted_address.is_some());
            assert!(place_details.place.geometry.is_some());
            assert!(place_details.place.icon.is_some());
            assert!(place_details.place.icon_background_color.is_some());
            assert!(place_details.place.icon_mask_base_uri.is_some());
            assert!(place_details.place.international_phone_number.is_some());
            assert!(place_details.place.name.unwrap_or("".to_string()).contains("Sydney"));
            assert!(place_details.place.opening_hours.is_some());
            assert!(place_details.place.photos.map(|vec| vec.len()).unwrap_or(0) > 0);
            assert_eq!(place_details.place.id, "ChIJN1t_tDeuEmsRUsoyG83frY4");
            assert!(place_details.place.plus_code.is_some());
            // assert!(place_details.place.price_level.is_some()); // this is occasionally null
            assert!(place_details.place.rating.is_some());
            assert!(place_details.place.reviews.map(|vec| vec.len()).unwrap_or(0) > 0);
            assert!(place_details.place.types.is_some());
            assert!(place_details.place.url.is_some());
            assert!(place_details.place.user_ratings_total.is_some());
            assert!(place_details.place.utc_offset.is_some());
            assert!(place_details.place.vicinity.map(|s| s).unwrap_or("".to_string()).contains("Pirrama"));
            assert!(place_details.place.website.is_some());
            assert!(place_details.place.wheelchair_accessible_entrance.is_some());
            assert!(place_details.place.website.is_some());
            assert!(place_details.place.formatted_phone_number.is_some());
            assert!(matches!(place_details.status, PlaceDetailsStatus::Ok));

        }
        Err(error) => {
            panic!("Error in place details test {}", error);
        }
    }

    let fields: HashSet<PlaceDetailsPlaceFields> = vec![
        PlaceDetailsPlaceFields::Name,
        PlaceDetailsPlaceFields::Rating,
        PlaceDetailsPlaceFields::FormattedPhoneNumber,
    ].into_iter().collect();

    match client.place_details_service.get_place_details(place_id, Some(&fields), Some(&language), Some(&region), Some(&review_no_translation), Some(&review_sort), None).await {
        Ok(place_details) => {
            assert!(place_details.place.name.unwrap_or("".to_string()).contains("Sydney"));
            assert!(place_details.place.rating.is_some());
            assert!(place_details.place.formatted_phone_number.is_some());
            assert!(place_details.place.website.is_none());
        }
        Err(error) => {
            panic!("Error in place details test {}", error);
        }
    }

}
