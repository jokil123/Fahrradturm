use std::time::Duration;

use prost_types::Timestamp;

pub fn timestamp_to_duration(timestamp: Timestamp) -> Duration {
    Duration::from_secs(timestamp.seconds as u64) + Duration::from_nanos(timestamp.nanos as u64)
}
