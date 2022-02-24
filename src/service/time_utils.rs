use chrono::{DateTime, SecondsFormat, Utc};

fn u64_to_utc(time: u64) -> DateTime<Utc> {
    let sys_time = std::time::UNIX_EPOCH + std::time::Duration::from_nanos(time);
    DateTime::<Utc>::from(sys_time)
}

pub fn now_to_str() -> String {
    get_now().to_rfc3339_opts(SecondsFormat::Millis, false)
}

#[cfg(not(test))]
pub fn get_now() -> DateTime<Utc> {
    let time = ic_kit::ic::time();
    u64_to_utc(time)
}

#[cfg(test)]
pub fn get_now() -> DateTime<Utc> {
    let time = 0;
    u64_to_utc(time)
}
