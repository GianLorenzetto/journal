use crate::prelude::*;

mod entry_date_parser;
mod entry_duration_parser;
mod entry_summary_parser;

pub struct EntryBuilder {
    pub entry: Option<Entry>,
}

impl EntryBuilder {
    pub fn from_str(raw_str: &str) -> Result<Entry, String> {
        let mut items: Vec<&str> = vec![];
        raw_str
            .split_whitespace()
            .for_each(|itm| items.push(itm.trim()));

        if items.len() < 4 {
            return Err(String::from("Not enough parameters"))
        }

        let (_, summary_items) = items.split_at(2);
        EntryBuilder::from(items[0], items[1], &summary_items.join(" "))
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
