use crate::constants::DATE_FORMAT;
use chrono::{DateTime, Utc};

pub trait DateFmtExt {
    fn ymd(&self) -> String;
}

impl DateFmtExt for DateTime<Utc> {
    fn ymd(&self) -> String {
        self.format(DATE_FORMAT).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_fmt_ext() {
        let dt = Utc::now();
        assert_eq!(dt.ymd(), dt.format(DATE_FORMAT).to_string());
    }
}
