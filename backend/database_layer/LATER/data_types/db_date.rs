use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DateType {
    #[serde(with = "ts_milliseconds")]
    time: DateTime<Utc>,
}

impl DateType {
    pub fn new(date: DateTime<Utc>) -> DateType {
        DateType { time: date }
    }
}

impl Default for DateType {
    fn default() -> Self {
        Self { time: Utc::now() }
    }
}
