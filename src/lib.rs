// Declare modules
pub mod error;
pub mod client;
pub mod models;
pub mod utils;
mod services;

// Re-export public items
pub use client::GooglePlacesClient;
