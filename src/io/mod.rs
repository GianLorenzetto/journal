use crate::prelude::*;
use chrono::{Local, Datelike, Date, DateTime, TimeZone};
use std::error::Error;
use serde::{Serialize,Deserialize};
use std::convert::TryFrom;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
struct EntryRecord {
    pub created: String,
    pub date: String,
    pub duration: String,
    pub summary: String,
}

impl EntryRecord {
    pub fn create(entry: &Entry) -> Self {
        Self {
            created: Local::now().to_rfc3339(),
            date: entry.date.and_hms(0, 0, 0).to_rfc3339(),
            duration: entry.duration.as_secs().to_string(),
            summary: entry.summary.clone(),
        }
    }

    fn as_entry(&self) -> Result<Entry, Box<dyn Error>> {
        let date = match DateTime::parse_from_rfc3339(&self.date) {
            Ok(d) => d.date().with_timezone(&Local),
            Err(e) => {
                return Err(e.to_string().into())
            },
        };
        let secs = self.duration.parse::<u64>()?;
        let duration = Duration::from_secs(secs);
        Ok(Entry::new(date, duration, &self.summary))
    }
}

fn file_path_from_date(date: &Date<Local>) -> String {
    format!("Journal_{}.csv", date.format("%b%Y"))
}

pub fn write_entry(entry: &Entry, date: &Date<Local>) -> Result<String, Box<dyn Error>> {
    let record = EntryRecord::create(entry);
    let file_name = file_path_from_date(date);
    let mut wtr = csv::Writer::from_path(&file_name)?;
    wtr.serialize(&record)?;
    wtr.flush()?;
    Ok(file_name)
}

pub fn read_entry(date: &Date<Local>) -> Result<Entry, Box< dyn Error>> {
    let file_name = file_path_from_date(date);
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_name)?;
    for result in rdr.deserialize() {
        let record: EntryRecord = result?;
        let entry: Entry = record.as_entry()?;
        return Ok(entry)
    }
    Err(String::from("Unable to read entry").into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use std::path::Path;
    use::std::fs;
    use std::fs::remove_file;

    #[test]
    fn given_entry_when_write_then_file_is_created() -> Result<(), Box<dyn Error>>{
        // Arrange
        let today = Local::today();
        let duration = Duration::from_secs(3600);
        let summary = "Description of the entry";
        let entry = Entry::new(today, duration, summary);

        // Clean up any existing file
        match remove_file(file_path_from_date(&today)) {
            Err(e) => println!("Error: {}", e),
            _ => {},
        }

        // Act - Write Entry
        let write_result = write_entry(&entry, &today)?;

        // Assert
        assert_eq!(write_result, format!("Journal_{}.csv", today.format("%b%Y")));
        assert!(Path::new(&write_result).exists());

        // Act - Read Entry
        let read_result = read_entry(&today)?;

        // Assert
        assert_eq!(read_result.summary, summary);

        Ok(())
    }
}