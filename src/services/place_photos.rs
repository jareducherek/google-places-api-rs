use crate::services::RequestService;
use crate::error::GooglePlacesError;
use image::io::Reader;
use std::io::Cursor;
use std::sync::Arc;

pub struct PlacePhotosService {
    client: Arc<RequestService>,
}

impl PlacePhotosService {
    pub fn new(client: Arc<RequestService>) -> Self {
        PlacePhotosService { client }
    }

    pub async fn get_photo_reference(
        &self,
        photo_reference: &str,
        max_width: Option<&u32>,
        max_height: Option<&u32>,
    ) -> Result<image::DynamicImage, GooglePlacesError> {
        // Construct the request URL
        let url = format!(
            "https://maps.googleapis.com/maps/api/place/photo?maxwidth={}&maxheight={}&photoreference={}&key={}",
            max_width.unwrap_or(&1000), max_height.unwrap_or(&1000), photo_reference, self.client.get_api_key()
        );
    
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

