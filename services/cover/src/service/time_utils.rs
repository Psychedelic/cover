use chrono::{DateTime, SecondsFormat, Utc};

fn u64_to_utc(time: u64) -> DateTime<Utc> {
    let sys_time = std::time::UNIX_EPOCH + std::time::Duration::from_nanos(time);
    DateTime::<Utc>::from(sys_time)
}

#[cfg(not(test))]
pub fn now_to_str() -> String {
    let time = ic_kit::ic::time();
    let utc = u64_to_utc(time);
    utc.to_rfc3339_opts(SecondsFormat::Millis, false)
}

#[cfg(test)]
pub fn now_to_str() -> String {
    let time = 0;
    let utc = u64_to_utc(time);
    utc.to_rfc3339_opts(SecondsFormat::Millis, false)
}
