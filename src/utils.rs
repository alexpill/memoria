use chrono::{DateTime, Utc};

pub fn get_utc_time() -> String {
    let now: DateTime<Utc> = Utc::now();
    now.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
}
