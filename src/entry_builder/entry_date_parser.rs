use chrono::{Date, Local, NaiveDate, Duration, TimeZone};
use chrono::format::ParseError;

pub fn from_str(date: &str) -> Result<Date<Local>, ParseError> {
    let date_result = match date {
        "today" => Local::today(),
        "tomorrow" => Local::today().checked_add_signed(Duration::days(1)).unwrap(),
        "yesterday" => Local::today().checked_sub_signed(Duration::days(1)).unwrap(),
        _ => {
            let nd = match NaiveDate::parse_from_str(date, "%y-%m-%d") {
                Ok(d) => d,
                Err(err) => return Err(err),
            };
            Local.from_local_date(&nd).unwrap()
        },
    };
    Ok(date_result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::{Add, Sub};
    use chrono::{Local, Datelike, Duration};

    #[test]
    fn given_valid_date_str_returns_ok() {
        let date_str = "21-7-13";

        let result = from_str(date_str);
        
        assert!(result.is_ok());

        let d = result.unwrap();
        assert_eq!(d.year(), 2021);
        assert_eq!(d.month(), 7);
        assert_eq!(d.day(), 13);
    }

    #[test]
    fn given_invalid_date_str_returns_err() {
        let result = from_str("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "premature end of input");

        let result = from_str("21-7");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "premature end of input");

        let result = from_str("21-13-7");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "input is out of range");
    }

    #[test]
    fn given_today_create_with_todays_date() {
        let date_str = "today";

        let result = from_str(date_str);

        assert!(result.is_ok());

        let d = result.unwrap();
        let today = Local::today();
        assert_eq!(d.year(), today.year());
        assert_eq!(d.month(), today.month());
        assert_eq!(d.day(), today.day());
    }

    #[test]
    fn given_tomorrow_create_with_tomorrows_date() {
        let date_str = "tomorrow";

        let result = from_str(date_str);

        assert!(result.is_ok());

        let d = result.unwrap();
        let tomorrow = Local::today().add(Duration::days(1));
        assert_eq!(d.year(), tomorrow.year());
        assert_eq!(d.month(), tomorrow.month());
        assert_eq!(d.day(), tomorrow.day());
    }

    #[test]
    fn given_yesterday_create_with_yesterdayss_date() {
        let date_str = "yesterday";

        let result = from_str(date_str);

        assert!(result.is_ok());

        let d = result.unwrap();
        let tomorrow = Local::today().sub(Duration::days(1));
        assert_eq!(d.year(), tomorrow.year());
        assert_eq!(d.month(), tomorrow.month());
        assert_eq!(d.day(), tomorrow.day());
    }
}