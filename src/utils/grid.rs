use geoutils::Location;
use itertools::iproduct;

pub fn coordinates_to_grid(tl_lat: f64, tl_lng: f64, br_lat: f64, br_lng: f64, m_btwn_pts: f64) -> Result<Vec<(f64, f64)>, GridError> {
    let tl = Location::new(tl_lat, tl_lng);
    let br = Location::new(br_lat, br_lng);
    let distance: f64 = tl.haversine_distance_to(&br).meters();
    let diag_count: f64 = (distance / m_btwn_pts).ceil();
    let dlat: f64 = (br_lat - tl_lat) / diag_count;
    let dlong: f64 = (br_lng - tl_lng) / diag_count;
    let all_points: Vec<(f64, f64)> = iproduct!(0..=(diag_count as usize), 0..=(diag_count as usize))
        .map(|(i, j)| {
            let cur_lat = tl_lat + i as f64 * dlat;
            let cur_long = tl_lng + j as f64 * dlong;
            (cur_lat, cur_long)
        })
        .collect();
    Ok(all_points)
}

#[derive(Debug)]
pub struct GridError {
    message: String,
}

impl GridError {
    fn new(message: &str) -> GridError {
        GridError {
            message: message.to_owned(),
        }
    }
}

impl std::fmt::Display for GridError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for GridError {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_coordinates_to_grid() {
        let tl_lat = 37.7749;
        let tl_lng = -122.4194;
        let br_lat = 37.3352;
        let br_lng = -121.8811;
        let m_btwn_pts = 800.0;
        let result = coordinates_to_grid(tl_lat, tl_lng, br_lat, br_lng, m_btwn_pts);
        match result {
            Ok(grid_points) => {
                println!("Grid points: {:?}", grid_points);
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }
}

