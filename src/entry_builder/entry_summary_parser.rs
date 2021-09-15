pub fn from_str(summary: &str) -> Result<&str, String> {
    let s = summary.trim();
    if s.is_empty() {
        return Err(String::from("Summary is empty"));
    }
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_valid_summary_str_returns_ok() {
        let summary_str = "A summary";

        let result = from_str(summary_str);

        assert!(result.is_ok());

        let s = result.unwrap();
        assert_eq!(s, summary_str);
    }

    #[test]
    fn given_empty_summary_str_returns_err() {
        let summary_str = "";

        let result = from_str(summary_str);

        assert!(result.is_err());

        let s = result.unwrap_err();
        assert_eq!(s, "Summary is empty");
    }
}
