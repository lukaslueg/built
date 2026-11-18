use crate::{environment, util::ParseFromEnv, write_str_variable, write_variable};
use jiff::{Timestamp, fmt::rfc2822, tz::TimeZone};
use std::{fs, io};

impl ParseFromEnv<'_> for Timestamp {
    type Err = jiff::Error;

    fn parse_from_env(s: &str) -> Result<Self, Self::Err> {
        rfc2822::DateTimeParser::new().parse_timestamp(s)
    }
}

/// Parse an RFC 2822 build time as a [`Timestamp`].
///
/// ```
/// use built::jiff::tz::TimeZone;
///
/// pub mod build_info {
///     pub static BUILT_TIME_UTC: &'static str = "Tue, 14 Feb 2017 05:21:41 GMT";
/// }
///
/// let timestamp = built::util::strptime(build_info::BUILT_TIME_UTC);
/// assert_eq!(timestamp.to_zoned_datetime(TimeZone::UTC).year(), 2017);
/// ```
///
/// # Panics
/// If the string can't be parsed. This should never happen with input provided
/// by `built`.
#[must_use]
pub fn strptime(s: &str) -> Timestamp {
    Timestamp::parse_from_env(s).unwrap()
}

fn get_source_date_epoch_from_env() -> Option<Timestamp> {
    match std::env::var(crate::SOURCE_DATE_EPOCH) {
        Ok(val) => {
            let ts = match val.parse::<i64>() {
                Ok(ts) => ts,
                Err(_) => {
                    eprintln!("SOURCE_DATE_EPOCH defined, but not a i64");
                    return None;
                }
            };
            match Timestamp::from_second(ts) {
                Ok(now) => Some(now),
                Err(_) => {
                    eprintln!("SOURCE_DATE_EPOCH can't be represented as a UTC-time");
                    None
                }
            }
        }
        Err(_) => None,
    }
}

pub fn write_time(mut w: &fs::File, envmap: &environment::EnvironmentMap) -> io::Result<()> {
    use io::Write;

    let now = match envmap.get_override_var("BUILT_TIME_UTC") {
        Some(v) => v,
        None => get_source_date_epoch_from_env().unwrap_or_else(Timestamp::now),
    };
    let utc_time =
        rfc2822::to_string(&now.to_zoned_datetime(TimeZone::UTC)).map_err(io::Error::other)?;
    write_str_variable!(
        w,
        "BUILT_TIME_UTC",
        utc_time,
        "The build time in RFC2822, UTC."
    );
    Ok(())
}
