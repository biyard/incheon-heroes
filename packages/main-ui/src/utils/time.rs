use chrono::{Local, TimeZone, Timelike, Utc};

pub fn formatted_timestamp(timestamp: i64) -> String {
    let datetime = Utc
        .timestamp_opt(timestamp, 0)
        .single()
        .expect("Invalid timestamp");

    datetime.format("%y.%m.%d").to_string()
}

pub fn time_diff_from_10() -> String {
    let now = Local::now();
    let naive_now = now.naive_local();

    let current_hour = naive_now.time().hour();

    let today_10 = now
        .date_naive()
        .and_hms_opt(10, 0, 0)
        .expect("Invalid time");

    let target_time = if current_hour < 10 {
        today_10
    } else {
        let tomorrow = now
            .date_naive()
            .succ_opt()
            .expect("Failed to get tomorrow's date");
        tomorrow.and_hms_opt(10, 0, 0).expect("Invalid time")
    };

    let target_time_local = Local
        .from_local_datetime(&target_time)
        .single()
        .expect("Invalid target time");

    let duration = target_time_local - now;

    let hours = duration.num_hours();
    let minutes = duration.num_minutes() % 60;
    let seconds = duration.num_seconds() % 60;

    format!("{}시간 {}분 {}초", hours, minutes, seconds)
}
