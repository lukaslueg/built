use crate::{write_str_variable, write_variable};
use std::{fs, io};

/// Parse a time-string as formatted by `built`.
///
/// ```
/// use chrono::Datelike;
///
/// pub mod build_info {
///     pub const BUILT_TIME_UTC: &'static str = "Tue, 14 Feb 2017 05:21:41 GMT";
/// }
///
/// assert_eq!(built::util::strptime(&build_info::BUILT_TIME_UTC).year(), 2017);
/// ```
///
/// # Panics
/// If the string can't be parsed. This should never happen with input provided
/// by `built`.
#[must_use]
pub fn strptime(s: &str) -> chrono::DateTime<chrono::offset::Utc> {
    chrono::DateTime::parse_from_rfc2822(s)
        .unwrap()
        .with_timezone(&chrono::offset::Utc)
}

pub fn write_time(mut w: &fs::File) -> io::Result<()> {
    use io::Write;

    let now = chrono::offset::Utc::now();
    write_str_variable!(
        w,
        "BUILT_TIME_UTC",
        now.to_rfc2822(),
        "The build time in RFC2822, UTC."
    );
    Ok(())
}
