use chrono::NaiveDateTime;

pub fn format_date(unix_timestamp: i64) -> String {
  NaiveDateTime::from_timestamp(unix_timestamp, 0)
    .format("%e %b %Y")
    .to_string()
}

pub fn format_timestamp(unix_timestamp: i64) -> String {
  NaiveDateTime::from_timestamp(unix_timestamp, 0)
    .format("%e %b %Y %H:%M:%S")
    .to_string()
}
