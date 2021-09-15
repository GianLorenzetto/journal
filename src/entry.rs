use crate::prelude::*;
use chrono::{Date, Local};
use std::time::Duration;

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
}
