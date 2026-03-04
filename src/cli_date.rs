use crate::constants::DATE_FORMAT;
use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc, Weekday};

enum DateOffset {
    Last,
    Next,
}

pub fn parse_raw_date(raw_date: &str) -> Result<DateTime<Utc>, String> {
    if let Ok(date) = NaiveDate::parse_from_str(raw_date, DATE_FORMAT) {
        return to_midnight_utc(&date);
    }

    let s = raw_date.trim().to_lowercase();
    let today = Utc::now().date_naive();

    match s.as_str() {
        "today" => return to_midnight_utc(&today),
        "yesterday" => return to_midnight_utc(&(today - Duration::days(1))),
        "tomorrow" => return to_midnight_utc(&(today + Duration::days(1))),
        _ => {}
    }

    let parts: Vec<_> = s.split_whitespace().collect();
    match parts.as_slice() {
        ["last", weekday] if parse_weekday_name(weekday).is_some() => {
            let target = parse_weekday_name(weekday).unwrap();
            let date = calculate_weekday_offset(target, DateOffset::Last, today);
            to_midnight_utc(&date)
        }
        ["next", weekday] if parse_weekday_name(weekday).is_some() => {
            let target = parse_weekday_name(weekday).unwrap();
            let date = calculate_weekday_offset(target, DateOffset::Next, today);
            to_midnight_utc(&date)
        }
        _ => Err(format!("Failed parsing date from \"{}\"", raw_date)),
    }
}

fn to_midnight_utc(date: &NaiveDate) -> Result<DateTime<Utc>, String> {
    date.and_hms_opt(0, 0, 0)
        .ok_or_else(|| "Failed to set time to midnight".to_string())
        .map(|dt| dt.and_utc())
}

fn calculate_weekday_offset(target: Weekday, offset_type: DateOffset, today: NaiveDate) -> NaiveDate {
    let current = today.weekday() as i64;
    let target = target as i64;

    let delta = match offset_type {
        DateOffset::Last => {
            let mut d = (current - target) % 7;
            if d <= 0 {
                d += 7;
            }
            -d
        }
        DateOffset::Next => {
            let mut d = (target - current) % 7;
            if d <= 0 {
                d += 7;
            }
            d
        }
    };

    today + Duration::days(delta)
}

fn parse_weekday_name(s: &str) -> Option<Weekday> {
    match s.trim() {
        "mon" | "monday" => Some(Weekday::Mon),
        "tue" | "tues" | "tuesday" => Some(Weekday::Tue),
        "wed" | "weds" | "wednesday" => Some(Weekday::Wed),
        "thu" | "thur" | "thurs" | "thursday" => Some(Weekday::Thu),
        "fri" | "friday" => Some(Weekday::Fri),
        "sat" | "saturday" => Some(Weekday::Sat),
        "sun" | "sunday" => Some(Weekday::Sun),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_date(input: &str, expected_year: i32, expected_month: u32, expected_day: u32) {
        let result = parse_raw_date(input).unwrap();
        assert_eq!(result.year(), expected_year);
        assert_eq!(result.month(), expected_month);
        assert_eq!(result.day(), expected_day);
    }

    #[test]
    fn test_absolute_date() {
        assert_date("2025-12-31", 2025, 12, 31);
    }

    #[test]
    fn test_relative_today() {
        let today = Utc::now();
        let result = parse_raw_date("today").unwrap();
        assert_eq!(result.date_naive(), today.date_naive());
    }

    #[test]
    fn test_relative_yesterday() {
        let yesterday = Utc::now() - Duration::days(1);
        let result = parse_raw_date("yesterday").unwrap();
        assert_eq!(result.date_naive(), yesterday.date_naive());
    }

    #[test]
    fn test_relative_tomorrow() {
        let tomorrow = Utc::now() + Duration::days(1);
        let result = parse_raw_date("tomorrow").unwrap();
        assert_eq!(result.date_naive(), tomorrow.date_naive());
    }

    #[test]
    fn test_last_weekday() {
        assert!(parse_raw_date("last monday").is_ok());
        assert!(parse_raw_date("last tuesday").is_ok());
        assert!(parse_raw_date("last wednesday").is_ok());
        assert!(parse_raw_date("last thursday").is_ok());
        assert!(parse_raw_date("last friday").is_ok());
        assert!(parse_raw_date("last saturday").is_ok());
        assert!(parse_raw_date("last sunday").is_ok());
    }

    #[test]
    fn test_next_weekday() {
        assert!(parse_raw_date("next monday").is_ok());
        assert!(parse_raw_date("next tuesday").is_ok());
        assert!(parse_raw_date("next wednesday").is_ok());
        assert!(parse_raw_date("next thursday").is_ok());
        assert!(parse_raw_date("next friday").is_ok());
        assert!(parse_raw_date("next saturday").is_ok());
        assert!(parse_raw_date("next sunday").is_ok());
    }

    #[test]
    fn test_invalid_date() {
        assert!(parse_raw_date("").is_err());
        assert!(parse_raw_date("invalid").is_err());
        assert!(parse_raw_date("2024-13-01").is_err());
        assert!(parse_raw_date("last invalid").is_err());
    }
}
