pub trait DateFmtExt {
    fn ymd(&self) -> String;
}
const DATE_FORMAT: &str = "%Y-%m-%d";

impl DateFmtExt for chrono::DateTime<chrono::Utc> {
    fn ymd(&self) -> String {
        self.format(DATE_FORMAT).to_string()
    }
}
