use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc, Weekday};

const DATE_FORMAT: &str = "%Y-%m-%d";

pub fn parse_raw_date(raw_date: &str) -> Result<DateTime<Utc>, String> {
    // Absolute
    if let Ok(date) = NaiveDate::parse_from_str(raw_date, DATE_FORMAT) {
        return date
            .and_hms_opt(0, 0, 0)
            .ok_or_else(|| String::from("Failed altering date time"))
            .map(|naive_date| naive_date.and_utc());
    }

    // Relative
    let s = raw_date.trim().to_lowercase();
    let today = Utc::now().date_naive();

    let to_midnight_utc = |date: &NaiveDate| {
        date.and_hms_opt(0, 0, 0)
            .ok_or_else(|| "Failed altering datetime.".to_string())
    };

    match s.as_str() {
        "today" => return Ok(to_midnight_utc(&today)?.and_utc()),
        "yesterday" => return Ok(to_midnight_utc(&(today - Duration::days(1)))?.and_utc()),
        "tomorrow" => return Ok(to_midnight_utc(&(today + Duration::days(1)))?.and_utc()),
        _ => {}
    }

    let parts: Vec<_> = s.split_whitespace().collect();
    match parts.as_slice() {
        ["last", rest] if parse_weekday_name(rest).is_some() => {
            let target_wd = parse_weekday_name(rest).unwrap();
            let current = today.weekday() as i64;
            let target = target_wd as i64;
            let mut delta = (current - target) % 7;
            if delta <= 0 {
                delta += 7;
            }
            let d = today - Duration::days(delta);
            return Ok(to_midnight_utc(&d)?.and_utc());
        }
        _ => {}
    }

    Err(format!("Failed parsing date part from \"{}\"", raw_date))
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
    fn test_invalid_date() {
        assert!(parse_raw_date("").is_err());
        assert!(parse_raw_date("invalid").is_err());
        assert!(parse_raw_date("2024-13-01").is_err());
        assert!(parse_raw_date("last invalid").is_err());
    }
}
