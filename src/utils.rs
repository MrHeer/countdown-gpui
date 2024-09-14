use chrono::Duration;

pub fn format_hm(hour: u32, minute: u32) -> String {
    format!("{:02}:{:02}", hour, minute)
}

pub fn format_duration(duration: &Duration) -> String {
    let hour = duration.num_hours() as u32;
    let minute = (duration.num_minutes() % 60) as u32;
    format_hm(hour, minute)
}
