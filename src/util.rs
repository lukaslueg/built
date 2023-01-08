//! Various convenience functions for `built` at runtime.

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
#[cfg(feature = "chrono")]
pub fn strptime(s: &str) -> chrono::DateTime<chrono::offset::Utc> {
    chrono::DateTime::parse_from_rfc2822(s)
        .unwrap()
        .with_timezone(&chrono::offset::Utc)
}

/// Retrieves the git-tag or hash describing the exact version and a boolean
/// that indicates if the repository currently has dirty/staged files.
///
/// If a valid git-repo can't be discovered at or above the given path,
/// `Ok(None)` is returned instead of an `Err`-value.
///
/// # Errors
/// Errors from `git2` are returned if the repository does exists at all.
#[cfg(feature = "git2")]
pub fn get_repo_description(root: &std::path::Path) -> Result<Option<(String, bool)>, git2::Error> {
    match git2::Repository::discover(root) {
        Ok(repo) => {
            let mut desc_opt = git2::DescribeOptions::new();
            desc_opt.describe_tags().show_commit_oid_as_fallback(true);
            let tag = repo
                .describe(&desc_opt)
                .and_then(|desc| desc.format(None))?;
            let mut st_opt = git2::StatusOptions::new();
            st_opt.include_ignored(false);
            st_opt.include_untracked(false);
            let dirty = repo
                .statuses(Some(&mut st_opt))?
                .iter()
                .any(|status| !matches!(status.status(), git2::Status::CURRENT));
            Ok(Some((tag, dirty)))
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

/// Retrieves the branch name and hash of HEAD.
///
/// The returned value is a tuple of head's reference-name, long-hash and short-hash. The
/// branch name will be `None` if the head is detached, or it's not valid UTF-8.
///
/// If a valid git-repo can't be discovered at or above the given path,
/// `Ok(None)` is returned instead of an `Err`-value.
///
/// # Errors
/// Errors from `git2` are returned if the repository does exists at all.
#[cfg(feature = "git2")]
pub fn get_repo_head(
    root: &std::path::Path,
) -> Result<Option<(Option<String>, String, String)>, git2::Error> {
    match git2::Repository::discover(root) {
        Ok(repo) => {
            // Supposed to be the reference pointed to by HEAD, but it's HEAD
            // itself, if detached
            let head_ref = repo.head()?;
            let branch = {
                // Check whether `head` is realy the pointed to reference and
                // not HEAD itself.
                if !repo.head_detached()? {
                    head_ref.name()
                } else {
                    None
                }
            };
            let head = head_ref.peel_to_commit()?;
            let commit = head.id();
            let commit_short = head.into_object().short_id()?;
            Ok(Some((
                branch.map(ToString::to_string),
                format!("{}", commit),
                format!("{}", commit_short.as_str().unwrap_or_default()),
            )))
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
#[must_use]
pub fn detect_ci() -> Option<super::CIPlatform> {
    super::CIPlatform::detect_from_envmap(&super::get_environment())
}
