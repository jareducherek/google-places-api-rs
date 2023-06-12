// Declare modules
pub mod error;
pub mod client;
pub mod models;
pub mod services;
pub mod utils;

// Re-export public items
pub use client::GooglePlacesClient;
pub use models::{PlaceDetails, NearbySearchResult};
pub use services::{PlaceDetailsService, PlaceSearchService};
