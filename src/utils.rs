use time::OffsetDateTime;

pub fn format_date_time(datetime: &OffsetDateTime) -> String {
    format!(
        "{}:{}:{} {}.{}.{}",
        datetime.hour(),
        datetime.minute(),
        datetime.second(),
        datetime.day(),
        datetime.month(),
        datetime.year()
    )
}
