
use std::time::{SystemTime, UNIX_EPOCH};


pub fn get_current_timestamp() -> u128 {
    let now = SystemTime::now();
    let timestamp_ms = now.duration_since(UNIX_EPOCH)
                        .expect("Time went backwards")
                        .as_millis();
    timestamp_ms
}