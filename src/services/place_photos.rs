use crate::client::GooglePlacesClient;
use crate::error::GooglePlacesError;
use crate::models::{Place, NearbySearchResult};
use image::io::Reader;
use std::io::Cursor;

pub struct PlacePhotosService {
    client: GooglePlacesClient,
}

impl PlacePhotosService {
    pub fn new(client: GooglePlacesClient) -> Self {
        PlacePhotosService { client }
    }

    pub async fn get_photo_reference(
        &self,
        photo_reference: &str,
        max_width: Option<u32>,
    ) -> Result<image::DynamicImage, GooglePlacesError> {
        // Construct the request URL
        let mut url = format!(
            "https://maps.googleapis.com/maps/api/place/photo?photoreference={}&key={}",
            photo_reference, self.client.get_api_key()
        );

        match max_width {
            Some(mw) => { url.push_str(&format!("&maxwidth={}", mw)) }
            None => {  }
        }
    
        // Send the HTTP GET request to download the image
        let response = self.client.get_req_client().get(&url).send().await.unwrap();
        let bytes = response.bytes().await.unwrap();
        let cur = Cursor::new(bytes);
        // Decode the image from the bytes using the image crate
        let reader = Reader::new(cur)
        .with_guessed_format()
        .expect("This will never fail using Cursor");

        let img: image::DynamicImage = reader.decode().expect("Failed to read image");
        Ok(img)

    }
}