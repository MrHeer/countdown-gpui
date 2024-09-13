use chrono::{DateTime, Duration, Local};

pub fn format_datetime(datetime: &DateTime<Local>) -> String {
    format!("{}", datetime.format("%H:%M"))
}

pub fn format_duration(duration: &Duration) -> String {
    let hours = duration.num_hours();
    let minutes = duration.num_minutes() % 60;
    format!("{:02}:{:02}", hours, minutes)
}
