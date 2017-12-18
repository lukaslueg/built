//! Various convenience functions for `built` at runtime.
#[cfg(feature = "serialized_git")]
extern crate git2;
#[cfg(feature = "serialized_version")]
extern crate semver;
#[cfg(feature = "serialized_time")]
extern crate time;

#[cfg(feature = "serialized_version")]
use std::iter;
#[cfg(feature = "serialized_git")]
use std::path;

#[cfg(feature = "serialized_version")]
type VersionParser<'a> = fn(&'a (&'a str, &'a str)) -> (&'a str, semver::Version);

/// Parses version-strings with `semver::Version::parse()`.
///
/// This function is only available if `built` was compiled with the
/// `serialized_version` feature.
///
/// The function takes a reference to an array of names and version numbers as
/// serialized by `built` and returns an iterator over the unchanged names
/// and parsed version numbers.
///
/// ```
/// extern crate built;
/// extern crate semver;
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
#[cfg(feature = "serialized_version")]
pub fn parse_versions<'a, T>(
    name_and_versions: T,
) -> iter::Map<<T as iter::IntoIterator>::IntoIter, VersionParser<'a>>
where
    T: IntoIterator<Item = &'a (&'a str, &'a str)>,
{
    fn parse_version<'a>(t: &'a (&'a str, &'a str)) -> (&'a str, semver::Version) {
        (t.0, t.1.parse().unwrap())
    }
    name_and_versions.into_iter().map(parse_version)
}

/// Parse a time-string as formatted by `built` into a `time::Tm`.
///
/// ```
/// extern crate built;
/// pub mod build_info {
///     pub const BUILT_TIME_UTC: &'static str = "Tue, 14 Feb 2017 05:21:41 GMT";
/// }
///
/// assert_eq!(built::util::strptime(&build_info::BUILT_TIME_UTC).tm_year, 117);
/// ```
///
/// # Panics
/// If the string can't be parsed. This should never happen with input provided
/// by `built`.
#[cfg(feature = "serialized_time")]
pub fn strptime(s: &str) -> time::Tm {
    time::strptime(s, "%a, %d %b %Y %T GMT").unwrap()
}

/// Retrieves the git-tag or hash describing the exact version.
///
/// If a valid git-repo can't be discovered at or above the given path,
/// `Ok(None)` is returned instead of an `Err`-value.
///
/// # Errors
/// Errors from `git2` are returned if the repository does exists at all.
#[cfg(feature = "serialized_git")]
pub fn get_repo_description<P: AsRef<path::Path>>(root: P) -> Result<Option<String>, git2::Error> {
    match git2::Repository::discover(root) {
        Ok(repo) => {
            let mut desc_opt = git2::DescribeOptions::new();
            desc_opt.describe_tags().show_commit_oid_as_fallback(true);
            Ok(Some(repo.describe(&desc_opt)
                .and_then(|desc| desc.format(None))?))
        }
        Err(ref e)
            if e.class() == git2::ErrorClass::Repository
                && e.code() == git2::ErrorCode::NotFound =>
        {
            Ok(None)
        }
        Err(e) => Err(e),
    }
}

/// Detect execution on various Continiuous Integration platforms.
///
/// CI-platforms are detected by the presence of known environment variables.
/// This allows to detect specific CI-platform (like `GitLab`); various
/// generic environment variables are also checked, which may result in
/// `CIPlatform::Generic`.
///
/// Since some platforms have fairly generic environment variables to begin with
/// (e.g. `TASK_ID`), this function may have false positives.
pub fn detect_ci() -> Option<super::CIPlatform> {
    super::CIPlatform::detect_from_envmap(&super::get_environment())
}
