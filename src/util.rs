//! Various convenience functions for `built` at runtime.

#[cfg(feature = "git2")]
pub use crate::git::{get_repo_description, get_repo_head};

#[cfg(feature = "chrono")]
pub use crate::krono::strptime;

/// Parses version-strings with `semver::Version::parse()`.
///
/// This function is only available if `built` was compiled with the
/// `semver` feature.
///
/// The function takes a reference to an array of names and version numbers as
/// serialized by `built` and returns an iterator over the unchanged names
/// and parsed version numbers.
///
/// ```
/// pub mod build_info {
///     pub const DEPENDENCIES: [(&'static str, &'static str); 1] = [("built", "0.1.0")];
/// }
///
/// let deps = build_info::DEPENDENCIES;
/// assert!(built::util::parse_versions(&deps)
///                      .any(|(name, ver)| name == "built" &&
///                                         ver >= semver::Version::parse("0.1.0").unwrap()));
/// ```
///
/// # Panics
/// If a version can't be parsed by `semver::Version::parse()`. This should never
/// happen with version strings provided by Cargo and `built`.
#[cfg(feature = "semver")]
pub fn parse_versions<'a, T>(
    name_and_versions: T,
) -> impl Iterator<Item = (&'a str, semver::Version)>
where
    T: IntoIterator<Item = &'a (&'a str, &'a str)>,
{
    fn parse_version<'a>(t: &'a (&'a str, &'a str)) -> (&'a str, semver::Version) {
        (t.0, t.1.parse().unwrap())
    }
    name_and_versions.into_iter().map(parse_version)
}

/// Detect execution on various Continuous Integration platforms.
///
/// CI-platforms are detected by the presence of known environment variables.
/// This allows to detect specific CI-platform (like `GitLab`); various
/// generic environment variables are also checked, which may result in
/// `CIPlatform::Generic`.
///
/// Since some platforms have fairly generic environment variables to begin with
/// (e.g. `TASK_ID`), this function may have false positives.
#[must_use]
pub fn detect_ci() -> Option<super::CIPlatform> {
    crate::environment::EnvironmentMap::new().detect_ci()
}
