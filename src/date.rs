use chrono::{Datelike, Duration, Local, NaiveDate, Weekday};

pub const DATE_FORMAT: &str = "%Y-%m-%d";

/// Today's date on the local calendar.
pub fn today() -> NaiveDate {
    Local::now().date_naive()
}

/// Parses a CLI date argument. Accepts an absolute `YYYY-MM-DD` date or one of
/// the relative forms `today`, `yesterday`, `tomorrow`, `last <weekday>` and
/// `next <weekday>`.
pub fn parse_raw_date(raw: &str) -> Result<NaiveDate, String> {
    parse_relative_to(raw, today())
}

fn parse_relative_to(raw: &str, today: NaiveDate) -> Result<NaiveDate, String> {
    let normalized = raw.trim().to_lowercase();
    if let Ok(date) = NaiveDate::parse_from_str(&normalized, DATE_FORMAT) {
        return Ok(date);
    }

    let parts: Vec<&str> = normalized.split_whitespace().collect();
    match parts.as_slice() {
        ["today"] => Ok(today),
        ["yesterday"] => Ok(today - Duration::days(1)),
        ["tomorrow"] => Ok(today + Duration::days(1)),
        ["last", weekday] => parse_weekday(weekday)
            .map(|target| shift_to_weekday(today, target, Direction::Last))
            .ok_or_else(|| invalid(raw)),
        ["next", weekday] => parse_weekday(weekday)
            .map(|target| shift_to_weekday(today, target, Direction::Next))
            .ok_or_else(|| invalid(raw)),
        _ => Err(invalid(raw)),
    }
}

fn invalid(raw: &str) -> String {
    format!("Failed parsing date from \"{raw}\"")
}

#[derive(Clone, Copy)]
enum Direction {
    Last,
    Next,
}

/// The closest strictly-past (`Last`) or strictly-future (`Next`) occurrence of
/// `target`, so that "last monday" on a Monday means one week ago.
fn shift_to_weekday(today: NaiveDate, target: Weekday, direction: Direction) -> NaiveDate {
    let current = i64::from(today.weekday().num_days_from_monday());
    let target = i64::from(target.num_days_from_monday());

    let days = match direction {
        Direction::Last => (current - target).rem_euclid(7),
        Direction::Next => (target - current).rem_euclid(7),
    };
    let days = if days == 0 { 7 } else { days };

    match direction {
        Direction::Last => today - Duration::days(days),
        Direction::Next => today + Duration::days(days),
    }
}

fn parse_weekday(s: &str) -> Option<Weekday> {
    match s {
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

    fn ymd(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    // 2025-12-31 is a Wednesday.
    fn wednesday() -> NaiveDate {
        ymd(2025, 12, 31)
    }

    #[test]
    fn absolute_date() {
        assert_eq!(parse_raw_date("2025-12-31"), Ok(ymd(2025, 12, 31)));
        assert_eq!(parse_raw_date("  2025-12-31 "), Ok(ymd(2025, 12, 31)));
    }

    #[test]
    fn relative_days() {
        let today = wednesday();
        assert_eq!(parse_relative_to("today", today), Ok(today));
        assert_eq!(parse_relative_to("Yesterday", today), Ok(ymd(2025, 12, 30)));
        assert_eq!(parse_relative_to(" tomorrow ", today), Ok(ymd(2026, 1, 1)));
    }

    #[test]
    fn relative_days_use_current_date() {
        assert_eq!(parse_raw_date("today"), Ok(today()));
        assert_eq!(parse_raw_date("yesterday"), Ok(today() - Duration::days(1)));
        assert_eq!(parse_raw_date("tomorrow"), Ok(today() + Duration::days(1)));
    }

    #[test]
    fn last_weekday() {
        let today = wednesday();
        assert_eq!(
            parse_relative_to("last monday", today),
            Ok(ymd(2025, 12, 29))
        );
        assert_eq!(parse_relative_to("last tue", today), Ok(ymd(2025, 12, 30)));
        assert_eq!(
            parse_relative_to("last wednesday", today),
            Ok(ymd(2025, 12, 24))
        );
        assert_eq!(
            parse_relative_to("last thursday", today),
            Ok(ymd(2025, 12, 25))
        );
        assert_eq!(
            parse_relative_to("Last Friday", today),
            Ok(ymd(2025, 12, 26))
        );
        assert_eq!(
            parse_relative_to("last saturday", today),
            Ok(ymd(2025, 12, 27))
        );
        assert_eq!(
            parse_relative_to("last sunday", today),
            Ok(ymd(2025, 12, 28))
        );
    }

    #[test]
    fn next_weekday() {
        let today = wednesday();
        assert_eq!(parse_relative_to("next monday", today), Ok(ymd(2026, 1, 5)));
        assert_eq!(
            parse_relative_to("next tuesday", today),
            Ok(ymd(2026, 1, 6))
        );
        assert_eq!(
            parse_relative_to("next wednesday", today),
            Ok(ymd(2026, 1, 7))
        );
        assert_eq!(parse_relative_to("next thu", today), Ok(ymd(2026, 1, 1)));
        assert_eq!(parse_relative_to("next friday", today), Ok(ymd(2026, 1, 2)));
        assert_eq!(
            parse_relative_to("next saturday", today),
            Ok(ymd(2026, 1, 3))
        );
        assert_eq!(parse_relative_to("next sunday", today), Ok(ymd(2026, 1, 4)));
    }

    #[test]
    fn invalid_date() {
        for input in [
            "",
            "invalid",
            "2024-13-01",
            "last invalid",
            "next",
            "last monday tuesday",
        ] {
            assert!(parse_raw_date(input).is_err(), "{input:?} should not parse");
        }
    }
}
