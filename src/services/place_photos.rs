use crate::services::RequestService;
use crate::error::GooglePlacesError;
use image::io::Reader;
use std::io::Cursor;
use std::sync::Arc;

pub struct PlacePhotosService {
    client: Arc<RequestService>,
}

mod place_photos {
    use super::*;

    pub fn build_photo_references(
        api_key: &str,
        photo_reference: &str,
        max_width: Option<&u32>,
        max_height: Option<&u32>,
    ) -> Result<String, GooglePlacesError> {
        let url = format!(
            "https://maps.googleapis.com/maps/api/place/photo?maxwidth={}&maxheight={}&photoreference={}&key={}",
            max_width.unwrap_or(&1000), max_height.unwrap_or(&1000), photo_reference, api_key
        );
        Ok(url)
    }    

}

impl PlacePhotosService {
    pub fn new(client: Arc<RequestService>) -> Self {
        PlacePhotosService { client }
    }

    #[tracing::instrument(level="debug", name="Place Photos", skip(self))]
    pub async fn get_photo_reference(
        &self,
        photo_reference: &str,
        max_width: Option<&u32>,
        max_height: Option<&u32>,
    ) -> Result<image::DynamicImage, GooglePlacesError> {
        // Construct the request URL
        let url = place_photos::build_photo_references(self.client.get_api_key(), photo_reference, max_width, max_height)?;
        tracing::debug!("Place Photos: `{url}`", url=url);
        let response = match self.client.get_response(&url).await{
            Ok(response) => response,
            Err(e) => return Err(e),
        };
        let bytes = match response.bytes().await{
            Ok(body) => body,
            Err(e) => return Err(GooglePlacesError::HttpError(e)),
        };
        let cur = Cursor::new(bytes);
        // Decode the image from the bytes using the image crate
        let reader = match Reader::new(cur).with_guessed_format(){
            Ok(reader) => reader,
            Err(e) => return Err(GooglePlacesError::ReaderError(e)),
        };

        let img: image::DynamicImage = reader.decode().expect("Failed to read image");
        Ok(img)

    }
}

