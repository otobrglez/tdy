use crate::cli_date;
use chrono::{DateTime, Duration, NaiveDate, Utc};
use log::info;

pub type RawWhen = Option<DateTime<Utc>>;
#[derive(PartialEq, Debug)]
pub enum When {
    Date(DateTime<Utc>),
    Query(String),
}

pub fn parse(raw: &str) -> Option<When> {
    info!("When parsing: {}", raw);

    match cli_date::parse_raw_date(raw) {
        Ok(date) => Some(When::Date(date)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_when_parse() {
        let today = Utc::now()
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc();

        assert_eq!(parse(""), None);

        assert_eq!(parse("today"), Some(When::Date(today)));

        assert_eq!(
            parse("tomorrow"),
            Some(When::Date(today + Duration::days(1)))
        );

        assert_eq!(
            parse("yesterday"),
            Some(When::Date(today - Duration::days(1)))
        );
    }
}
