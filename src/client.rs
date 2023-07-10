use std::sync::Arc;
use crate::services::{RequestService, PlaceSearchService, PlaceDetailsService, PlacePhotosService};
use crate::error::GooglePlacesError;


pub struct GooglePlacesClient {
    pub place_search_service: PlaceSearchService,
    pub place_details_service: PlaceDetailsService,
    pub place_photos_service: PlacePhotosService,
}

impl GooglePlacesClient {
    pub fn new(api_key: &str, max_requests: Option<u64>, per: Option<std::time::Duration>) -> Self {
        let request_service = Arc::new(RequestService::new(api_key, max_requests, per));

        GooglePlacesClient {
            place_search_service: PlaceSearchService::new(request_service.clone()),
            place_details_service: PlaceDetailsService::new(request_service.clone()),
            place_photos_service: PlacePhotosService::new(request_service.clone()),
        }
    }
}
