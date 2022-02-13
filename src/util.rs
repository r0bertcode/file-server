use chrono::prelude::DateTime;
use chrono::Utc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
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
