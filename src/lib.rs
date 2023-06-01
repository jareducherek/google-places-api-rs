// Declare modules
pub mod error;
pub mod client;
pub mod models;
pub mod services;
pub mod utils;

// Re-export public items
pub use client::GooglePlacesClient;
pub use models::{Place, PlaceDetails, PlaceSearchResult};
pub use services::GooglePlacesService;
