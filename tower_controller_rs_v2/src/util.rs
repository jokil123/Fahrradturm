use std::time::Duration;

use prost_types::Timestamp;

use crate::controller_error::ControllerError;

pub fn timestamp_to_duration(timestamp: Timestamp) -> Duration {
    Duration::from_secs(timestamp.seconds as u64) + Duration::from_nanos(timestamp.nanos as u64)
}

pub fn box_id_to_coords(id: &str) -> Result<Vec<u32>, ControllerError> {
    id.split(',')
        .map(|s| s.parse::<u32>().map_err(|_| ControllerError::ParseError))
        .collect()
}

pub fn coords_to_box_id(coords: &Vec<u32>) -> String {
    coords
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

pub fn generate_n_dimensional_coords(dim: &Vec<u32>) -> Vec<Vec<u32>> {
    match dim.len() {
        0 => vec![vec![]],
        1 => {
            let mut coords = Vec::new();
            for i in 0..dim[0] {
                coords.push(vec![i]);
            }
            coords
        }
        _ => {
            let mut coords = Vec::new();
            for i in 0..dim[0] {
                let sub_coords = generate_n_dimensional_coords(&dim[1..].to_vec());
                for mut c in sub_coords {
                    c.insert(0, i);
                    coords.push(c);
                }
            }
            coords
        }
    }
}
