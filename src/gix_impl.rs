use gix::{bstr::ByteSlice, discover::upwards};
use std::path;

pub use gix::Error as GixError;

/// Retrieves the git-tag or hash describing the exact version and a boolean
/// that indicates if the repository currently has dirty/staged files.
///
/// If a valid git-repo can't be discovered at or above the given path,
/// `None` is returned. Other repository errors are returned as `Err`.
pub fn get_repo_description(
    manifest_location: &path::Path,
) -> Result<Option<(String, bool)>, GixError> {
    match gix::discover(manifest_location) {
        Ok(repo) => {
            let commit = repo.head_commit()?;
            let tag = commit.describe().format()?.to_string();
            let dirty = repo.is_dirty()?;
            Ok(Some((tag, dirty)))
        }
        Err(e) if e.downcast_any_ref::<upwards::Error>().is_some() => Ok(None),
        Err(e) => Err(e),
    }
}

/// Retrieves the branch name and hash of HEAD.
///
/// The returned value is a tuple of head's reference-name, long-hash and short-hash. The
/// branch name will be `None` if the head is detached, or it's not valid UTF-8.
///
/// If a valid git-repo can't be discovered at or above the given path,
/// `None` is returned. Other repository errors are returned as `Err`.
pub fn get_repo_head(
    manifest_location: &path::Path,
) -> Result<Option<(Option<String>, String, String)>, GixError> {
    match gix::discover(manifest_location) {
        Ok(repo) => {
            let mut head = repo.head()?;
            let branch = head
                .clone()
                .referent_name()
                .and_then(|rn| rn.as_bstr().to_str().ok().map(ToOwned::to_owned));
            if let Some(commit_id) = head.try_peel_to_id()? {
                let commit_id_short = commit_id.shorten_or_id().to_string();
                Ok(Some((branch, commit_id.to_string(), commit_id_short)))
            } else {
                Ok(None)
            }
        }
        Err(e) if e.downcast_any_ref::<upwards::Error>().is_some() => Ok(None),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::{get_repo_description, get_repo_head};

    #[test]
    fn discovery_errors() {
        let root = tempfile::tempdir().unwrap();
        assert_eq!(get_repo_description(root.path()).unwrap(), None);
        assert_eq!(get_repo_head(root.path()).unwrap(), None);

        gix::init(root.path()).unwrap();
        std::fs::write(root.path().join(".git/config"), "[invalid").unwrap();
        assert!(get_repo_description(root.path()).is_err());
        assert!(get_repo_head(root.path()).is_err());
    }
}
