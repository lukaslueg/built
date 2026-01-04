use gix::bstr::ByteSlice;
use std::convert::Infallible;
use std::path;

/// An adapter to add a `Result` around the `Option` we natively return. This is the easiest way
/// to have one shared implementation with `git2` which exports these functions.
#[cfg_attr(feature = "git2", allow(unused))]
pub(crate) fn get_repo_head(
    manifest_location: &path::Path,
) -> Result<Option<(Option<String>, String, String)>, Infallible> {
    Ok(get_repo_head_inner(manifest_location))
}

/// An adapter to add a `Result` around the `Option` we natively return. This is the easiest way
/// to have one shared implementation with `git2` which exports these functions.
#[cfg_attr(feature = "git2", allow(unused))]
pub(crate) fn get_repo_description(
    manifest_location: &path::Path,
) -> Result<Option<(String, bool)>, Infallible> {
    Ok(get_repo_description_inner(manifest_location))
}

/// Retrieves the git-tag or hash describing the exact version and a boolean
/// that indicates if the repository currently has dirty/staged files.
///
/// If a valid git-repo can't be discovered at or above the given path,
/// or if any operation on the repository fails, `None` is returned.
fn get_repo_description_inner(manifest_location: &path::Path) -> Option<(String, bool)> {
    let repo = gix::discover(manifest_location).ok()?;
    let commit = repo.head_commit().ok()?;
    let tag = commit.describe().format().ok()?.to_string();
    let dirty = repo.is_dirty().ok()?;

    Some((tag, dirty))
}

/// Retrieves the branch name and hash of HEAD.
///
/// The returned value is a tuple of head's reference-name, long-hash and short-hash. The
/// branch name will be `None` if the head is detached, or it's not valid UTF-8.
///
/// If a valid git-repo can't be discovered at or above the given path,
/// or if any operation on the repository fails, `None` is returned.
fn get_repo_head_inner(manifest_location: &path::Path) -> Option<(Option<String>, String, String)> {
    let repo = gix::discover(manifest_location).ok()?;
    let mut head = repo.head().ok()?;
    let branch = head
        .clone()
        .referent_name()
        .and_then(|rn| rn.as_bstr().to_str().ok().map(ToOwned::to_owned));
    let commit_id = head.try_peel_to_id().ok().flatten()?;
    let commit_id_short = commit_id.shorten_or_id().to_string();

    Some((branch, commit_id.to_string(), commit_id_short))
}
