use chrono::{TimeZone, Utc};

pub fn formatted_timestamp(timestamp: i64) -> String {
    let datetime = Utc
        .timestamp_opt(timestamp, 0)
        .single()
        .expect("Invalid timestamp");

    datetime.format("%y.%m.%d").to_string()
}
