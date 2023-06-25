use std::sync::Arc;
use crate::services::{RequestService, PlaceSearchService, PlaceDetailsService, PlacePhotosService};

pub struct GooglePlacesClient {
    request_service: Arc<RequestService>,
    pub place_search_service: PlaceSearchService,
    pub place_details_service: PlaceDetailsService,
    pub place_photos_service: PlacePhotosService,
}

impl GooglePlacesClient {
    pub fn new(api_key: &str) -> Self {
        let request_service = Arc::new(RequestService::new(api_key));

        GooglePlacesClient {
            place_search_service: PlaceSearchService::new(request_service.clone()),
            place_details_service: PlaceDetailsService::new(request_service.clone()),
            place_photos_service: PlacePhotosService::new(request_service.clone()),
            request_service: request_service,
        }
    }
}
