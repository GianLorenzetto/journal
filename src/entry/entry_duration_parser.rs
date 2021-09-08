use parse_duration::parse;
use parse_duration::parse::Error;
use std::time::Duration;

pub fn from_str(duration: &str) -> Result<Duration, Error> {
   parse(duration)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entry::entry_duration_parser;

    #[test]
    fn given_empty_str_returns_err() {
        let duration_str = "";

        let result = entry_duration_parser::from_str(duration_str);
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "NoValueFoundError: no value found in the string \"\"");
    }

    #[test]
    fn given_invalid_unit_returns_err() {
        let duration_str = "10 x";

        let result = entry_duration_parser::from_str(duration_str);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "UnknownUnitError: \"x\" is not a known unit");
    }

    #[test]
    fn given_invalid_duration_returns_err() {
        let duration_str = "a mins";

        let result = entry_duration_parser::from_str(duration_str);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "NoValueFoundError: no value found in the string \"a mins\"");
    }

    #[test]
    fn given_valid_str_returns_ok() {
        let duration_str = "10 mins";

        let result = entry_duration_parser::from_str(duration_str);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_secs(), 600);
    }
}