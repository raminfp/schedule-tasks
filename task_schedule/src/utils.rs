use std::time::Duration;

pub fn convert_duration_to_seconds(duration_in_seconds: u64) -> Duration{
    Duration::from_secs(duration_in_seconds)
}

pub fn convert_duration_to_minutes(duration_in_minutes: u64) -> Duration {
    Duration::from_secs(duration_in_minutes * 60)
}

pub fn convert_duration_to_hours(duration_in_hours: u64) -> Duration {
    Duration::from_secs(duration_in_hours * 3600)
}

pub fn convert_duration_to_days(duration_in_day: u64) -> Duration {
    Duration::from_secs(duration_in_day * 86400)
}

pub fn convert_duration_to_weeks(duration_in_week: u64) -> Duration {
    Duration::from_secs(duration_in_week * 604800)
}

