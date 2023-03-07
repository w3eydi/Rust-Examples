use std::time::{SystemTime};
use chrono::{DateTime, Utc};

fn iso_date() -> String {
    let now = SystemTime::now();
    let now: DateTime<Utc> = now.into();
    now.to_rfc3339()
}