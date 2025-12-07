use chrono::{DateTime, Utc};
use prost_types::Timestamp;

pub fn datetime_to_timestamp(dt: DateTime<Utc>) -> Option<Timestamp> {
    Some(Timestamp {
        seconds: dt.timestamp(),
        nanos: dt.timestamp_subsec_nanos() as i32,
    })
}