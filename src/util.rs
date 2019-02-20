//! Various convenience functions for `built` at runtime.
#[cfg(feature = "serialized_time")]
extern crate chrono;
#[cfg(feature = "serialized_git")]
extern crate git2;
#[cfg(feature = "serialized_version")]
extern crate semver;

#[cfg(feature = "serialized_git")]
use std::path;

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
) -> impl Iterator<Item = (&'a str, semver::Version)>
where
    T: IntoIterator<Item = &'a (&'a str, &'a str)>,
{
    fn parse_version<'a>(t: &'a (&'a str, &'a str)) -> (&'a str, semver::Version) {
        (t.0, t.1.parse().unwrap())
    }
    name_and_versions.into_iter().map(parse_version)
}

/// Parse a time-string as formatted by `built`.
///
/// ```
/// extern crate built;
/// extern crate chrono;
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
#[cfg(feature = "serialized_time")]
pub fn strptime(s: &str) -> chrono::DateTime<chrono::offset::Utc> {
    chrono::DateTime::parse_from_rfc2822(s)
        .unwrap()
        .with_timezone(&chrono::offset::Utc)
}

/// Retrieves the git-tag or hash describing the exact version.
///
/// If a valid git-repo can't be discovered at or above the given path,
/// `Ok(None)` is returned instead of an `Err`-value.
///
/// # Errors
/// Errors from `git2` are returned if the repository does exists at all.
#[cfg(feature = "serialized_git")]
pub fn get_repo_description<P: AsRef<path::Path>, S: AsRef<str>>(root: P, dirty_suffix: S, rerun_on_git_change: bool) -> Result<Option<String>, git2::Error> {
    use git2::*;
    match Repository::discover(root) {
        Ok(repo) => {
            if rerun_on_git_change {
                let path = repo.path();
                let tags = path.join("refs/tags");
                // If a new tag is added, it should appear here. Creating a new tag will hopefully change the directory modify time.
                // Tags should never change, so if they do then it's probably ok to rerun.
                println!("cargo:rerun-if-changed={}", tags.to_string_lossy());
                if tags.exists() && tags.is_dir() {
                    for read_dir in ::std::fs::read_dir(tags) {
                        read_dir
                          .filter_map(|r|r.ok())
                          .for_each(|entry| {
                              entry
                                .file_type()
                                .ok()
                                .filter(std::fs::FileType::is_file)
                                .map(|_| println!("cargo:rerun-if-changed={}", entry.path().to_string_lossy()));
                          });
                    }
                }
                // HEAD changes on checkout of a branch/tag/commit.
                println!("cargo:rerun-if-changed={}", path.join("HEAD").to_string_lossy());
                // The ref that head points at (in case of a branch at refs/heads/<branch name>) changes as commits are added,
                // so we want to trigger reruns since that would change the output of the build script
                repo.head()
                  .and_then(|r|r.resolve())
                  .ok()
                  .and_then(|r|
                    r.name()
                      .map(|n|
                        println!("cargo:rerun-if-changed={}", path.join(n).to_string_lossy())
                      )
                  );
            }
            let mut desc_opt = DescribeOptions::new();
            desc_opt.describe_tags().show_commit_oid_as_fallback(true);
            let mut format_opts = DescribeFormatOptions::new();
            format_opts.dirty_suffix(dirty_suffix.as_ref());
            Ok(Some(
                repo.describe(&desc_opt)
                    .and_then(|desc| desc.format(Some(&format_opts)))?,
            ))
        }
        Err(ref e)
            if e.class() == ErrorClass::Repository
                && e.code() == ErrorCode::NotFound =>
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
