use chrono::{Date, Local};
use std::time::Duration;

use crate::entry::{entry_date_parser, entry_duration_parser, entry_summary_parser};

#[derive(Debug)]
pub struct Entry {
    pub date: Date<Local>,
    pub duration: std::time::Duration,
    pub summary: String,
}

impl Entry {
    pub fn new(date: Date<Local>, duration: Duration, summary: &str) -> Self {
        Self {
            date,
            duration,
            summary: String::from(summary),
        }
    }

    pub fn from(date: &str, duration: &str, summary: &str) -> Result<Entry, String> {
        let date_result = entry_date_parser::from_str(date);
        if date_result.is_err() {
            return Err(date_result.unwrap_err().to_string());
        }

        let duration_result = entry_duration_parser::from_str(duration);
        if duration_result.is_err() {
            return Err(duration_result.unwrap_err().to_string());
        }

        let summary_result = entry_summary_parser::from_str(summary);
        if summary_result.is_err() {
            return Err(summary_result.unwrap_err().to_string());
        }

        Ok(Entry::new(date_result.unwrap(), duration_result.unwrap(), summary_result.unwrap()))
    }
}