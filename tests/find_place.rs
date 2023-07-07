use dotenv::dotenv;
use std::env;
use std::collections::HashSet;
use google_places_api::client::GooglePlacesClient;
use google_places_api::models::place_search::{PlaceSearchStatus};
use google_places_api::models::constants::{PlaceDataField, Language, InputType, LocationBias};

#[tokio::test]
async fn test_find_place() {
    dotenv().ok();
    let api_key = env::var("GOOGLE_PLACES_API_KEY").expect("Please set the GOOGLE_PLACES_API_KEY environment variable");
    let client = GooglePlacesClient::new(&api_key);
    let input = "Mongolian Grill";
    let input_type: InputType = InputType::TextQuery;
    let fields: HashSet<PlaceDataField> = vec![
        PlaceDataField::Name,
        PlaceDataField::AddressComponents,
        PlaceDataField::BusinessStatus,
        PlaceDataField::FormattedAddress,
        PlaceDataField::Location,
        PlaceDataField::Icon,
        PlaceDataField::IconMaskBaseUri,
        PlaceDataField::IconBackgroundColor,
        PlaceDataField::PermanentlyClosed,
        PlaceDataField::Photo,
        PlaceDataField::PlaceId,
        PlaceDataField::PlusCode,
        PlaceDataField::Type,
        PlaceDataField::Url,
        PlaceDataField::UtcOffset,
        PlaceDataField::Vicinity,
        PlaceDataField::WheelchairAccessibleEntrance,

        PlaceDataField::FormattedPhoneNumber,
        PlaceDataField::InternationalPhoneNumber,
        PlaceDataField::OpeningHours,
        PlaceDataField::CurrentOpeningHours,
        PlaceDataField::SecondaryOpeningHours,
        PlaceDataField::Website,

        PlaceDataField::CurbsidePickup,
        PlaceDataField::Delivery,
        PlaceDataField::DineIn,
        PlaceDataField::EditorialSummary,
        PlaceDataField::PriceLevel,
        PlaceDataField::Rating,
        PlaceDataField::Reservable,
        PlaceDataField::Reviews,
        PlaceDataField::ServesBeer,
        PlaceDataField::ServesBreakfast,
        PlaceDataField::ServesBrunch,
        PlaceDataField::ServesDinner,
        PlaceDataField::ServesLunch,
        PlaceDataField::ServesVegetarianFood,
        PlaceDataField::ServesWine,
        PlaceDataField::Takeout,
        PlaceDataField::UserRatingsTotal,
    ].into_iter().collect();
    let language: Language = Language::En;
    let location_bias: LocationBias = LocationBias::Circular {
        radius: 10000,
        latitude: 33.85984846198168,
        longitude: 151.20907015422375,
    };
    // Perform the request
    //match client.place_search_service.find_place(input, &input_type, Some(&fields), Some(&language), Some(&location_bias)).await {
    //    Ok(search_result) => {
    //        println!("{}", search_result.display());
    //    }
    //    Err(error) => {
    //        panic!("Error in find place test {}", error);
    //    }
    //}

    match client.place_search_service.find_place(input, &input_type, Some(&fields), Some(&language), Some(&location_bias)).await {
        Ok(find_place) => {
            // assert!(find_place.places.len() > 0);
            assert!(find_place.places.len() == find_place.total_results as usize);
            assert!(matches!(find_place.status, PlaceSearchStatus::Ok));
            for place in find_place.places.iter() {
                assert!(place.id.len() > 0);
                assert!(place.name.is_some());
                assert!(place.address_components.is_some());
                assert!(place.business_status.is_some());
                assert!(place.formatted_address.is_some());
                assert!(place.vicinity.is_some());
                assert!(place.icon.is_some());
                assert!(place.icon_mask_base_uri.is_some());
                assert!(place.icon_background_color.is_some());
                assert!(place.permanently_closed.is_some());
                assert!(place.photos.is_some());
                // assert!(place.id.is_some());
                assert!(place.plus_code.is_some());
                assert!(place.types.is_some());
                assert!(place.url.is_some());
                assert!(place.utc_offset.is_some());
                assert!(place.vicinity.is_some());
                assert!(place.wheelchair_accessible_entrance.is_some());

                assert!(place.formatted_phone_number.is_some());
                assert!(place.international_phone_number.is_some());
                assert!(place.opening_hours.is_some());
                assert!(place.current_opening_hours.is_some());
                assert!(place.secondary_opening_hours.is_some());
                assert!(place.website.is_some());

                assert!(place.curbside_pickup.is_some());
                assert!(place.delivery.is_some());
                assert!(place.dine_in.is_some());
                assert!(place.editorial_summary.is_some());
                assert!(place.price_level.is_some());
                assert!(place.rating.is_some());
                assert!(place.reservable.is_some());
                assert!(place.reviews.is_some());
                assert!(place.serves_beer.is_some());
                assert!(place.serves_breakfast.is_some());
                assert!(place.serves_brunch.is_some());
                assert!(place.serves_dinner.is_some());
                assert!(place.serves_lunch.is_some());
                assert!(place.serves_vegetarian_food.is_some());
                assert!(place.serves_wine.is_some());
                assert!(place.takeout.is_some());
                assert!(place.user_ratings_total.is_some());

            }
        }
        Err(error) => {
            panic!("Error in nearby search test {}", error);
        }
    }
}
