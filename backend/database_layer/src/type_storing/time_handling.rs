use chrono::{DateTime, Utc};

/// empty struct for importing all time handling methods
pub struct TimeHandling {}

impl TimeHandling {
    /// Create and encode a current timestamp into text format
    /// used for storing in the database
    ///
    /// Returns
    /// ---
    /// - string representing a current timestamp
    pub fn store() -> String {
        Utc::now().to_string()
    }

    /// Load a timestamp from the database and convert it into datetime
    ///
    /// Returns
    /// - Ok(date) if the conversion was successful
    /// - Err(_) if an error occurred while parsing the input
    pub fn load_timestamp(input: &str) -> anyhow::Result<DateTime<Utc>> {
        Ok(input.parse::<DateTime<Utc>>()?)
    }
}
