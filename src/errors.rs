#[derive(Debug)]
pub enum GoogleMapPlaceError {
    BadRequest(String),

    Unknown(String),
}