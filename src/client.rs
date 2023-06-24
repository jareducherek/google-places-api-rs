use reqwest::Client;
use crate::services::{RequestService, PlaceSearchService, PlaceDetailsService, PlacePhotosService};

pub struct GooglePlacesClient<'a> {
    request_service: RequestService,
    place_search_service: PlaceSearchService<'a>,
    place_details_service: PlaceDetailsService<'a>,
    place_photos_service: PlacePhotosService<'a>,
}

impl<'a> GooglePlacesClient<'a> {
    pub fn new(api_key: &str) -> Self {
        let request_service = RequestService::new(api_key);

        GooglePlacesClient {
            request_service: request_service,
            place_search_service: PlaceSearchService::new(&request_service),
            place_details_service: PlaceDetailsService::new(&request_service),
            place_photos_service: PlacePhotosService::new(&request_service),
        }
    }
}
