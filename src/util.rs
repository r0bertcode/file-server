use chrono::prelude::DateTime;
use chrono::Utc;
use std::fs::read;
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;
/**
 * Helper to get a timestamp in u64(Seconds) format
 */
pub fn get_timestamp() -> u64 {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    timestamp
}
/**
 * Helper to get human readable timestamp from u64
 */
pub fn get_readable_timestamp(tiemstamp: u64) -> String {
    let d = UNIX_EPOCH + Duration::from_secs(tiemstamp);
    let date_time = DateTime::<Utc>::from(d);
    let readable_timestamp = date_time.format("%Y-%m-%d %H:%M:%S.%f").to_string();

    readable_timestamp
}
/**
 * Helper to get a Uuid v4 in string format
 */
pub fn get_uuid() -> String {
    let uuid = Uuid::new_v4().to_string();

    uuid
}
/**
 * Helper to get file data from a path
 */
pub fn get_file_data(path: &str) -> Option<Vec<u8>> {
    if !Path::new(path).exists() {
        return None;
    }

    let read_result = read(path);
    if read_result.is_err() {
        return None;
    }

    let buff = read_result.unwrap();

    Some(buff)
}
