// MIT License
//
// Copyright (c) 2017-2022 Lukas Lueg
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

#![allow(clippy::needless_doctest_main)]

//! Provides a crate with information from the time it was built.
//!
//! `built` is used as a build-time dependency to collect various information
//! about the build environment, serialize it into Rust-code and compile
//! it into the final crate. The information collected by `built` include:
//!
//!  * Various metadata like version, authors, homepage etc. as set by `Cargo.toml`
//!  * The tag or commit id if the crate was being compiled from within a git repo.
//!  * The values of `CARGO_CFG_*` build script env variables, like `CARGO_CFG_TARGET_OS` and
//!    `CARGO_CFG_TARGET_ARCH`.
//!  * The features the crate was compiled with.
//!  * The various dependencies, dependencies of dependencies and their versions
//!    cargo ultimately chose to compile.
//!  * The presence of a CI-platform like `Travis CI` and `AppVeyor`.
//!  * The used compiler and it's version; the used documentation generator and
//!    it's version.
//!
//! See the [`Options`][options]-type regarding what `built` can serialize.
//!
//! `built` does not add any further dependencies to a crate; all information
//! is serialized as types from `stdlib`. One can include `built` as a
//! runtime-dependency and use it's convenience functions.
//!
//! To add `built` to a crate, add it as a build-time dependency, use a build
//! script that collects and serializes the build-time information and `include!`
//! that as code.
//!
//! Add this to `Cargo.toml`:
//!
//! ```toml
//! [package]
//! build = "build.rs"
//!
//! [build-dependencies]
//! built = "0.5"
//! ```
//!
//! Add or modify a build script. In `build.rs`:
//!
//! ```rust,no_run
//! fn main() {
//!     built::write_built_file().expect("Failed to acquire build-time information");
//! }
//! ```
//!
//! The build-script will by default write a file named `built.rs` into Cargo's output
//! directory. It can be picked up in  `main.rs` (or anywhere else) like this:
//!
//! ```rust,ignore
//! // Use of a mod or pub mod is not actually necessary.
//! pub mod built_info {
//!    // The file has been placed there by the build script.
//!    include!(concat!(env!("OUT_DIR"), "/built.rs"));
//! }
//! ```
//!
//! And then used somewhere in the crate's code:
//!
//! ```rust
//! # mod built_info {
//! #    pub const PKG_VERSION_PRE: &str = "";
//! #    pub const CI_PLATFORM: Option<&str> = None;
//! #    pub const GIT_VERSION: Option<&str> = None;
//! #    pub const DEPENDENCIES: [(&str, &str); 0] = [];
//! #    pub const BUILT_TIME_UTC: &str = "Tue, 14 Feb 2017 05:21:41 GMT";
//! # }
//! #
//! # enum LogLevel { TRACE, ERROR };
//! /// Determine if current version is a pre-release or was built from a git-repo
//! fn release_is_unstable() -> bool {
//!     return !built_info::PKG_VERSION_PRE.is_empty() || built_info::GIT_VERSION.is_some()
//! }
//!
//! /// Default log-level, enhanced on CI
//! fn default_log_level() -> LogLevel {
//!     if built_info::CI_PLATFORM.is_some() {
//!         LogLevel::TRACE
//!     } else {
//!         LogLevel::ERROR
//!     }
//! }
//!
//! /// The time this crate was built
//! #[cfg(feature = "chrono")]
//! fn built_time() -> built::chrono::DateTime<built::chrono::Local> {
//!     built::util::strptime(built_info::BUILT_TIME_UTC)
//!         .with_timezone(&built::chrono::offset::Local)
//! }
//!
//! /// If another crate pulls in a dependency we don't like, print a warning
//! #[cfg(feature = "semver")]
//! fn check_sane_dependencies() {
//!     if built::util::parse_versions(&built_info::DEPENDENCIES)
//!                     .any(|(name, ver)| name == "DeleteAllMyFiles"
//!                                        && ver < built::semver::Version::parse("1.1.4").unwrap()) {
//!         eprintln!("DeleteAllMyFiles < 1.1.4 may not delete all your files. Beware!");
//!     }
//! }
//! ```
//!
//! A full `built.rs` will look something like:
//! ```
//! /// The Continuous Integration platform detected during compilation.
//! pub const CI_PLATFORM: Option<&str> = None;
//! #[doc="The full version."]
//! pub const PKG_VERSION: &str = "0.1.0";
//! #[doc="The major version."]
//! pub const PKG_VERSION_MAJOR: &str = "0";
//! #[doc="The minor version."]
//! pub const PKG_VERSION_MINOR: &str = "1";
//! #[doc="The patch version."]
//! pub const PKG_VERSION_PATCH: &str = "0";
//! #[doc="The pre-release version."]
//! pub const PKG_VERSION_PRE: &str = "";
//! #[doc="A colon-separated list of authors."]
//! pub const PKG_AUTHORS: &str = "Lukas Lueg <lukas.lueg@gmail.com>";
//! #[doc="The name of the package."]
//! pub const PKG_NAME: &str = "example_project";
//! #[doc="The description."]
//! pub const PKG_DESCRIPTION: &str = "";
//! #[doc="The homepage."]
//! pub const PKG_HOMEPAGE: &str = "";
//! #[doc="The license."]
//! pub const PKG_LICENSE: &str = "MIT";
//! #[doc="The source repository as advertised in Cargo.toml."]
//! pub const PKG_REPOSITORY: &str = "";
//! #[doc="The target triple that was being compiled for."]
//! pub const TARGET: &str = "x86_64-unknown-linux-gnu";
//! #[doc="The host triple of the rust compiler."]
//! pub const HOST: &str = "x86_64-unknown-linux-gnu";
//! #[doc="`release` for release builds, `debug` for other builds."]
//! pub const PROFILE: &str = "debug";
//! #[doc="The compiler that cargo resolved to use."]
//! pub const RUSTC: &str = "rustc";
//! #[doc="The documentation generator that cargo resolved to use."]
//! pub const RUSTDOC: &str = "rustdoc";
//! #[doc="Value of OPT_LEVEL for the profile used during compilation."]
//! pub const OPT_LEVEL: &str = "0";
//! #[doc="The parallelism that was specified during compilation."]
//! pub const NUM_JOBS: u32 = 8;
//! #[doc="Value of DEBUG for the profile used during compilation."]
//! pub const DEBUG: bool = true;
//! /// The features that were enabled during compilation.
//! pub const FEATURES: [&str; 0] = [];
//! /// The features as a comma-separated string.
//! pub const FEATURES_STR: &str = "";
//! /// The output of `rustc -V`
//! pub const RUSTC_VERSION: &str = "rustc 1.43.1 (8d69840ab 2020-05-04)";
//! /// The output of `rustdoc -V`
//! pub const RUSTDOC_VERSION: &str = "rustdoc 1.43.1 (8d69840ab 2020-05-04)";
//! /// If the crate was compiled from within a git-repository, `GIT_VERSION` contains HEAD's tag. The short commit id is used if HEAD is not tagged.
//! pub const GIT_VERSION: Option<&str> = Some("0.4.1-10-gca2af4f");
//! /// If the repository had dirty/staged files.
//! pub const GIT_DIRTY: Option<bool> = Some(true);
//! /// If the crate was compiled from within a git-repository, `GIT_HEAD_REF` contains full name to the reference pointed to by HEAD (e.g.: `refs/heads/master`). If HEAD is detached or the branch name is not valid UTF-8 `None` will be stored.
//! pub const GIT_HEAD_REF: Option<&str> = Some("refs/heads/master");
//! /// If the crate was compiled from within a git-repository, `GIT_COMMIT_HASH` contains HEAD's full commit SHA-1 hash.
//! pub const GIT_COMMIT_HASH: Option<&str> = Some("ca2af4f11bb8f4f6421c4cccf428bf4862573daf");
//! /// If the crate was compiled from within a git-repository, `GIT_COMMIT_HASH_SHORT` contains HEAD's short commit SHA-1 hash.
//! pub const GIT_COMMIT_HASH_SHORT: Option<&str> = Some("ca2af4f");
//! /// An array of effective dependencies as documented by `Cargo.lock`.
//! pub const DEPENDENCIES: [(&str, &str); 37] = [("autocfg", "1.0.0"), ("bitflags", "1.2.1"), ("built", "0.4.1"), ("cargo-lock", "4.0.1"), ("cc", "1.0.54"), ("cfg-if", "0.1.10"), ("chrono", "0.4.11"), ("example_project", "0.1.0"), ("git2", "0.13.6"), ("idna", "0.2.0"), ("jobserver", "0.1.21"), ("libc", "0.2.71"), ("libgit2-sys", "0.12.6+1.0.0"), ("libz-sys", "1.0.25"), ("log", "0.4.8"), ("matches", "0.1.8"), ("num-integer", "0.1.42"), ("num-traits", "0.2.11"), ("percent-encoding", "2.1.0"), ("pkg-config", "0.3.17"), ("proc-macro2", "1.0.17"), ("quote", "1.0.6"), ("semver", "1.0.0"), ("serde", "1.0.110"), ("serde_derive", "1.0.110"), ("smallvec", "1.4.0"), ("syn", "1.0.25"), ("time", "0.1.43"), ("toml", "0.5.6"), ("unicode-bidi", "0.3.4"), ("unicode-normalization", "0.1.12"), ("unicode-xid", "0.2.0"), ("url", "2.1.1"), ("vcpkg", "0.2.8"), ("winapi", "0.3.8"), ("winapi-i686-pc-windows-gnu", "0.4.0"), ("winapi-x86_64-pc-windows-gnu", "0.4.0")];
//! /// The effective dependencies as a comma-separated string.
//! pub const DEPENDENCIES_STR: &str = "autocfg 1.0.0, bitflags 1.2.1, built 0.4.1, cargo-lock 4.0.1, cc 1.0.54, cfg-if 0.1.10, chrono 0.4.11, example_project 0.1.0, git2 0.13.6, idna 0.2.0, jobserver 0.1.21, libc 0.2.71, libgit2-sys 0.12.6+1.0.0, libz-sys 1.0.25, log 0.4.8, matches 0.1.8, num-integer 0.1.42, num-traits 0.2.11, percent-encoding 2.1.0, pkg-config 0.3.17, proc-macro2 1.0.17, quote 1.0.6, semver 1.0.0, serde 1.0.110, serde_derive 1.0.110, smallvec 1.4.0, syn 1.0.25, time 0.1.43, toml 0.5.6, unicode-bidi 0.3.4, unicode-normalization 0.1.12, unicode-xid 0.2.0, url 2.1.1, vcpkg 0.2.8, winapi 0.3.8, winapi-i686-pc-windows-gnu 0.4.0, winapi-x86_64-pc-windows-gnu 0.4.0";
//! /// The built-time in RFC2822, UTC
//! pub const BUILT_TIME_UTC: &str = "Wed, 27 May 2020 18:12:39 +0000";
//! /// The target architecture, given by `CARGO_CFG_TARGET_ARCH`.
//! pub const CFG_TARGET_ARCH: &str = "x86_64";
//! /// The endianness, given by `CARGO_CFG_TARGET_ENDIAN`.
//! pub const CFG_ENDIAN: &str = "little";
//! /// The toolchain-environment, given by `CARGO_CFG_TARGET_ENV`.
//! pub const CFG_ENV: &str = "gnu";
//! /// The OS-family, given by `CARGO_CFG_TARGET_FAMILY`.
//! pub const CFG_FAMILY: &str = "unix";
//! /// The operating system, given by `CARGO_CFG_TARGET_OS`.
//! pub const CFG_OS: &str = "linux";
//! /// The pointer width, given by `CARGO_CFG_TARGET_POINTER_WIDTH`.
//! pub const CFG_POINTER_WIDTH: &str = "64";
//! ```
//! [options]: struct.Options.html

pub mod util;

use std::{
    collections, env, ffi, fmt, fs, io,
    io::{Read, Write},
    path, process,
};

#[cfg(feature = "semver")]
pub use semver;

#[cfg(feature = "chrono")]
pub use chrono;

#[doc = include_str!("../README.md")]
#[allow(dead_code)]
type _READMETEST = ();

macro_rules! write_variable {
    ($writer:expr, $name:expr, $datatype:expr, $value:expr, $doc:expr) => {
        writeln!(
            $writer,
            "#[doc=r#\"{}\"#]\n#[allow(dead_code)]\npub const {}: {} = {};",
            $doc, $name, $datatype, $value
        )?;
    };
}

macro_rules! write_str_variable {
    ($writer:expr, $name:expr, $value:expr, $doc:expr) => {
        write_variable!(
            $writer,
            $name,
            "&str",
            format!("r\"{}\"", $value.escape_default()),
            $doc
        );
    };
}

/// Various Continuous Integration platforms whose presence can be detected.
pub enum CIPlatform {
    /// <https://travis-ci.org>
    Travis,
    /// <https://circleci.com>
    Circle,
    /// <https://about.gitlab.com/gitlab-ci>
    GitLab,
    /// <https://www.appveyor.com>
    AppVeyor,
    /// <https://codeship.com>
    Codeship,
    /// <https://github.com/drone/drone>
    Drone,
    /// <https://magnum-ci.com>
    Magnum,
    /// <https://semaphoreci.com>
    Semaphore,
    /// <https://jenkins.io>
    Jenkins,
    /// <https://www.atlassian.com/software/bamboo>
    Bamboo,
    /// <https://www.visualstudio.com/de/tfs>
    TFS,
    /// <https://www.jetbrains.com/teamcity>
    TeamCity,
    /// <https://buildkite.com>
    Buildkite,
    /// <http://hudson-ci.org>
    Hudson,
    /// <https://github.com/taskcluster>
    TaskCluster,
    /// <https://www.gocd.io>
    GoCD,
    /// <https://bitbucket.org>
    BitBucket,
    /// <https://github.com/features/actions>
    GitHubActions,
    /// Unspecific
    Generic,
}

impl fmt::Display for CIPlatform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            CIPlatform::Travis => "Travis CI",
            CIPlatform::Circle => "CircleCI",
            CIPlatform::GitLab => "GitLab",
            CIPlatform::AppVeyor => "AppVeyor",
            CIPlatform::Codeship => "CodeShip",
            CIPlatform::Drone => "Drone",
            CIPlatform::Magnum => "Magnum",
            CIPlatform::Semaphore => "Semaphore",
            CIPlatform::Jenkins => "Jenkins",
            CIPlatform::Bamboo => "Bamboo",
            CIPlatform::TFS => "Team Foundation Server",
            CIPlatform::TeamCity => "TeamCity",
            CIPlatform::Buildkite => "Buildkite",
            CIPlatform::Hudson => "Hudson",
            CIPlatform::TaskCluster => "TaskCluster",
            CIPlatform::GoCD => "GoCD",
            CIPlatform::BitBucket => "BitBucket",
            CIPlatform::GitHubActions => "GitHub Actions",
            CIPlatform::Generic => "Generic CI",
        })
    }
}

type EnvironmentMap = collections::HashMap<String, String>;

fn get_environment() -> EnvironmentMap {
    let mut envmap = EnvironmentMap::new();
    for (k, v) in env::vars_os() {
        let k = k.into_string();
        let v = v.into_string();
        if let (Ok(k), Ok(v)) = (k, v) {
            envmap.insert(k, v);
        }
    }
    envmap
}

impl CIPlatform {
    fn detect_from_envmap(envmap: &EnvironmentMap) -> Option<CIPlatform> {
        macro_rules! detect {
            ($(($k:expr, $v:expr, $i:ident)),*) => {$(
                    if envmap.get($k).map_or(false, |v| v == $v) {
                        return Some(CIPlatform::$i);
                    }
                    )*};
            ($(($k:expr, $i:ident)),*) => {$(
                    if envmap.contains_key($k) {
                        return Some(CIPlatform::$i);
                    }
                    )*};
            ($($k:expr),*) => {$(
                if envmap.contains_key($k) {
                    return Some(CIPlatform::Generic);
                }
            )*};
        }
        // Variable names collected by watson/ci-info
        detect!(
            ("TRAVIS", Travis),
            ("CIRCLECI", Circle),
            ("GITLAB_CI", GitLab),
            ("APPVEYOR", AppVeyor),
            ("DRONE", Drone),
            ("MAGNUM", Magnum),
            ("SEMAPHORE", Semaphore),
            ("JENKINS_URL", Jenkins),
            ("bamboo_planKey", Bamboo),
            ("TF_BUILD", TFS),
            ("TEAMCITY_VERSION", TeamCity),
            ("BUILDKITE", Buildkite),
            ("HUDSON_URL", Hudson),
            ("GO_PIPELINE_LABEL", GoCD),
            ("BITBUCKET_COMMIT", BitBucket),
            ("GITHUB_ACTIONS", GitHubActions)
        );

        if envmap.contains_key("TASK_ID") && envmap.contains_key("RUN_ID") {
            return Some(CIPlatform::TaskCluster);
        }

        detect!(("CI_NAME", "codeship", Codeship));

        detect!(
            "CI",                     // Could be Travis, Circle, GitLab, AppVeyor or CodeShip
            "CONTINUOUS_INTEGRATION", // Probably Travis
            "BUILD_NUMBER"            // Jenkins, TeamCity
        );
        None
    }
}

fn get_build_deps(manifest_location: &path::Path) -> io::Result<Vec<(String, String)>> {
    let mut lock_buf = String::new();
    fs::File::open(manifest_location.join("Cargo.lock"))?.read_to_string(&mut lock_buf)?;
    Ok(parse_dependencies(&lock_buf))
}

fn parse_dependencies(lock_toml_buf: &str) -> Vec<(String, String)> {
    let lockfile: cargo_lock::Lockfile = lock_toml_buf.parse().expect("Failed to parse lockfile");
    let mut deps = Vec::new();

    for package in lockfile.packages {
        deps.push((package.name.to_string(), package.version.to_string()));
    }
    deps.sort_unstable();
    deps
}

fn get_version_from_cmd(executable: &ffi::OsStr) -> io::Result<String> {
    let output = process::Command::new(executable).arg("-V").output()?;
    let mut v = String::from_utf8(output.stdout).unwrap();
    v.pop(); // remove newline
    Ok(v)
}

fn write_compiler_version(
    rustc: &ffi::OsStr,
    rustdoc: &ffi::OsStr,
    w: &mut fs::File,
) -> io::Result<()> {
    let rustc_version = get_version_from_cmd(rustc)?;
    let rustdoc_version = get_version_from_cmd(rustdoc)?;

    let doc = format!("The output of `{} -V`", rustc.to_string_lossy());
    write_str_variable!(w, "RUSTC_VERSION", rustc_version, doc);

    let doc = format!("The output of `{} -V`", rustdoc.to_string_lossy());
    write_str_variable!(w, "RUSTDOC_VERSION", rustdoc_version, doc);
    Ok(())
}

fn fmt_option_str<S: fmt::Display>(o: Option<S>) -> String {
    match o {
        Some(s) => format!("Some(\"{s}\")"),
        None => "None".to_owned(),
    }
}

#[cfg(all(feature = "git2", not(feature = "gitoxide")))]
fn write_git_version(manifest_location: &path::Path, w: &mut fs::File) -> io::Result<()> {
    // CIs will do shallow clones of repositories, causing libgit2 to error
    // out. We try to detect if we are running on a CI and ignore the
    // error.
    let (tag, dirty) = match util::get_repo_description(manifest_location) {
        Ok(Some((tag, dirty))) => (Some(tag), Some(dirty)),
        _ => (None, None),
    };
    write_variable!(
        w,
        "GIT_VERSION",
        "Option<&str>",
        fmt_option_str(tag),
        "If the crate was compiled from within a git-repository, \
        `GIT_VERSION` contains HEAD's tag. The short commit id is used if HEAD is not tagged."
    );
    write_variable!(
        w,
        "GIT_DIRTY",
        "Option<bool>",
        match dirty {
            Some(true) => "Some(true)",
            Some(false) => "Some(false)",
            None => "None",
        },
        "If the repository had dirty/staged files."
    );

    let (branch, commit, commit_short) = match util::get_repo_head(manifest_location) {
        Ok(Some((b, c, cs))) => (b, Some(c), Some(cs)),
        _ => (None, None, None),
    };

    let doc = "If the crate was compiled from within a git-repository, `GIT_HEAD_REF` \
        contains full name to the reference pointed to by HEAD \
        (e.g.: `refs/heads/master`). If HEAD is detached or the branch name is not \
        valid UTF-8 `None` will be stored.\n";
    write_variable!(
        w,
        "GIT_HEAD_REF",
        "Option<&str>",
        fmt_option_str(branch),
        doc
    );

    write_variable!(
        w,
        "GIT_COMMIT_HASH",
        "Option<&str>",
        fmt_option_str(commit),
        "If the crate was compiled from within a git-repository, `GIT_COMMIT_HASH` \
    contains HEAD's full commit SHA-1 hash."
    );

    write_variable!(
        w,
        "GIT_COMMIT_HASH_SHORT",
        "Option<&str>",
        fmt_option_str(commit_short),
        "If the crate was compiled from within a git-repository, `GIT_COMMIT_HASH_SHORT` \
    contains HEAD's short commit SHA-1 hash."
    );

    Ok(())
}

fn write_ci(envmap: &EnvironmentMap, w: &mut fs::File) -> io::Result<()> {
    write_variable!(
        w,
        "CI_PLATFORM",
        "Option<&str>",
        fmt_option_str(CIPlatform::detect_from_envmap(envmap)),
        "The Continuous Integration platform detected during compilation."
    );
    Ok(())
}

fn write_features(envmap: &EnvironmentMap, w: &mut fs::File) -> io::Result<()> {
    let mut features = Vec::new();
    for name in envmap.keys() {
        if let Some(feat) = name.strip_prefix("CARGO_FEATURE_") {
            features.push(feat.to_owned());
        }
    }
    features.sort();

    write_variable!(
        w,
        "FEATURES",
        format!("[&str; {}]", features.len()),
        format!("{features:?}"),
        "The features that were enabled during compilation."
    );

    let features_str = features.join(", ");
    write_str_variable!(
        w,
        "FEATURES_STR",
        features_str,
        "The features as a comma-separated string."
    );
    Ok(())
}

fn write_env(envmap: &EnvironmentMap, w: &mut fs::File) -> io::Result<()> {
    macro_rules! write_env_str {
        ($(($name:ident, $env_name:expr,$doc:expr)),*) => {$(
            write_str_variable!(
                w,
                stringify!($name),
                envmap.get($env_name)
                    .expect(stringify!(Missing expected environment variable $env_name)),
                    $doc
            );
        )*}
    }

    write_env_str!(
        (PKG_VERSION, "CARGO_PKG_VERSION", "The full version."),
        (
            PKG_VERSION_MAJOR,
            "CARGO_PKG_VERSION_MAJOR",
            "The major version."
        ),
        (
            PKG_VERSION_MINOR,
            "CARGO_PKG_VERSION_MINOR",
            "The minor version."
        ),
        (
            PKG_VERSION_PATCH,
            "CARGO_PKG_VERSION_PATCH",
            "The patch version."
        ),
        (
            PKG_VERSION_PRE,
            "CARGO_PKG_VERSION_PRE",
            "The pre-release version."
        ),
        (
            PKG_AUTHORS,
            "CARGO_PKG_AUTHORS",
            "A colon-separated list of authors."
        ),
        (PKG_NAME, "CARGO_PKG_NAME", "The name of the package."),
        (PKG_DESCRIPTION, "CARGO_PKG_DESCRIPTION", "The description."),
        (PKG_HOMEPAGE, "CARGO_PKG_HOMEPAGE", "The homepage."),
        (PKG_LICENSE, "CARGO_PKG_LICENSE", "The license."),
        (
            PKG_REPOSITORY,
            "CARGO_PKG_REPOSITORY",
            "The source repository as advertised in Cargo.toml."
        ),
        (
            TARGET,
            "TARGET",
            "The target triple that was being compiled for."
        ),
        (HOST, "HOST", "The host triple of the rust compiler."),
        (
            PROFILE,
            "PROFILE",
            "`release` for release builds, `debug` for other builds."
        ),
        (RUSTC, "RUSTC", "The compiler that cargo resolved to use."),
        (
            RUSTDOC,
            "RUSTDOC",
            "The documentation generator that cargo resolved to use."
        )
    );
    write_str_variable!(
        w,
        "OPT_LEVEL",
        env::var("OPT_LEVEL").unwrap(),
        "Value of OPT_LEVEL for the profile used during compilation."
    );
    write_variable!(
        w,
        "NUM_JOBS",
        "u32",
        env::var("NUM_JOBS").unwrap(),
        "The parallelism that was specified during compilation."
    );
    write_variable!(
        w,
        "DEBUG",
        "bool",
        env::var("DEBUG").unwrap() == "true",
        "Value of DEBUG for the profile used during compilation."
    );
    Ok(())
}

fn write_dependencies(manifest_location: &path::Path, w: &mut fs::File) -> io::Result<()> {
    let deps = get_build_deps(manifest_location)?;
    write_variable!(
        w,
        "DEPENDENCIES",
        format!("[(&str, &str); {}]", deps.len()),
        format!("{deps:?}"),
        "An array of effective dependencies as documented by `Cargo.lock`."
    );
    write_str_variable!(
        w,
        "DEPENDENCIES_STR",
        deps.iter()
            .map(|(n, v)| format!("{n} {v}"))
            .collect::<Vec<_>>()
            .join(", "),
        "The effective dependencies as a comma-separated string."
    );
    Ok(())
}

#[cfg(feature = "chrono")]
fn write_time(w: &mut fs::File) -> io::Result<()> {
    let now = chrono::offset::Utc::now();
    write_str_variable!(
        w,
        "BUILT_TIME_UTC",
        now.to_rfc2822(),
        "The build time in RFC2822, UTC."
    );
    Ok(())
}

fn write_cfg(w: &mut fs::File) -> io::Result<()> {
    fn get_env(name: &str) -> String {
        env::var(name).unwrap_or_default()
    }

    let target_arch = get_env("CARGO_CFG_TARGET_ARCH");
    let target_endian = get_env("CARGO_CFG_TARGET_ENDIAN");
    let target_env = get_env("CARGO_CFG_TARGET_ENV");
    let target_family = get_env("CARGO_CFG_TARGET_FAMILY");
    let target_os = get_env("CARGO_CFG_TARGET_OS");
    let target_pointer_width = get_env("CARGO_CFG_TARGET_POINTER_WIDTH");

    write_str_variable!(
        w,
        "CFG_TARGET_ARCH",
        target_arch,
        "The target architecture, given by `CARGO_CFG_TARGET_ARCH`."
    );

    write_str_variable!(
        w,
        "CFG_ENDIAN",
        target_endian,
        "The endianness, given by `CARGO_CFG_TARGET_ENDIAN`."
    );

    write_str_variable!(
        w,
        "CFG_ENV",
        target_env,
        "The toolchain-environment, given by `CARGO_CFG_TARGET_ENV`."
    );

    write_str_variable!(
        w,
        "CFG_FAMILY",
        target_family,
        "The OS-family, given by `CARGO_CFG_TARGET_FAMILY`."
    );

    write_str_variable!(
        w,
        "CFG_OS",
        target_os,
        "The operating system, given by `CARGO_CFG_TARGET_OS`."
    );

    write_str_variable!(
        w,
        "CFG_POINTER_WIDTH",
        target_pointer_width,
        "The pointer width, given by `CARGO_CFG_TARGET_POINTER_WIDTH`."
    );

    Ok(())
}

#[cfg(feature = "gitoxide")]
#[allow(dead_code, unused_imports, unused_variables)]
mod gitoxide_impls {
    use crate::fmt_option_str;
    use gix;
    use std::{fs, io, io::Write, path};

    // NOTE: There are a few opportunities to make this code more maintainable by refactoring / deduplicating, but I wanted
    // to keep it simple to showcase the changes in the most simple way. Happy to adjust this once this PR moves further.

    #[derive(Debug, Default, PartialEq)]
    struct RepoInfo {
        branch: Option<String>,
        tag: Option<String>,
        dirty: Option<bool>,
        commit_id: Option<String>,
        commit_id_short: Option<String>,
    }

    fn get_repo_info(manifest_location: &path::Path) -> Option<RepoInfo> {
        let repo = gix::discover(manifest_location).ok()?;

        let branch = repo.head_name().ok()?.map(|n| n.to_string());

        let repo_info = if let Ok(commit) = repo.head_commit() {
            RepoInfo {
                branch,
                tag: commit.describe().format().ok().map(|f| f.to_string()),
                dirty: is_dirty(manifest_location),
                commit_id: Some(commit.id().to_string()),
                commit_id_short: commit.id().shorten().ok().map(|p| p.to_string()),
            }
        } else {
            RepoInfo {
                branch,
                ..Default::default()
            }
        };

        Some(repo_info)
    }

    // TODO: replace git2 with gitoxide once this functionality becomes available in git-repository.
    fn is_dirty(manifest_location: &path::Path) -> Option<bool> {
        let mut options = git2::StatusOptions::new();
        options.include_ignored(false);
        options.include_untracked(false);

        let dirty = git2::Repository::discover(manifest_location)
            .ok()?
            .statuses(Some(&mut options))
            .ok()?
            .iter()
            .any(|status| !matches!(status.status(), git2::Status::CURRENT));

        Some(dirty)
    }

    pub(crate) fn write_git_version(
        manifest_location: &path::Path,
        w: &mut fs::File,
    ) -> io::Result<()> {
        let info = get_repo_info(manifest_location).unwrap_or_default();

        write_variable!(
            w,
            "GIT_VERSION",
            "Option<&str>",
            fmt_option_str(info.tag),
            "If the crate was compiled from within a git-repository, \
            `GIT_VERSION` contains HEAD's tag. The short commit id is used if HEAD is not tagged."
        );
        write_variable!(
            w,
            "GIT_DIRTY",
            "Option<bool>",
            match info.dirty {
                Some(true) => "Some(true)",
                Some(false) => "Some(false)",
                None => "None",
            },
            "If the repository had dirty/staged files."
        );

        let doc = "If the crate was compiled from within a git-repository, `GIT_HEAD_REF` \
        contains full name to the reference pointed to by HEAD \
        (e.g.: `refs/heads/master`). If HEAD is detached or the branch name is not \
        valid UTF-8 `None` will be stored.\n";
        write_variable!(
            w,
            "GIT_HEAD_REF",
            "Option<&str>",
            fmt_option_str(info.branch),
            doc
        );

        write_variable!(
            w,
            "GIT_COMMIT_HASH",
            "Option<&str>",
            fmt_option_str(info.commit_id),
            "If the crate was compiled from within a git-repository, `GIT_COMMIT_HASH` \
    contains HEAD's full commit SHA-1 hash."
        );

        write_variable!(
            w,
            "GIT_COMMIT_HASH_SHORT",
            "Option<&str>",
            fmt_option_str(info.commit_id_short),
            "If the crate was compiled from within a git-repository, `GIT_COMMIT_HASH_SHORT` \
    contains HEAD's short commit SHA-1 hash."
        );

        Ok(())
    }

    // NOTE: Copy-pasted test and replaced functions from `util::*` with `gitoxide_impls::get_repo_info`.

    #[cfg(test)]
    mod tests {
        #[test]
        fn parse_git_repo() {
            use std::fs;
            use std::path;

            let repo_root = tempfile::tempdir().unwrap();
            assert_eq!(super::get_repo_info(repo_root.as_ref()), None);

            let repo = git2::Repository::init_opts(
                &repo_root,
                git2::RepositoryInitOptions::new()
                    .external_template(false)
                    .mkdir(false)
                    .no_reinit(true)
                    .mkpath(false),
            )
            .unwrap();

            let cruft_file = repo_root.path().join("cruftfile");
            std::fs::write(&cruft_file, "Who? Me?").unwrap();

            let project_root = repo_root.path().join("project_root");
            fs::create_dir(&project_root).unwrap();

            let sig = git2::Signature::now("foo", "bar").unwrap();
            let mut idx = repo.index().unwrap();
            idx.add_path(path::Path::new("cruftfile")).unwrap();
            idx.write().unwrap();
            let commit_oid = repo
                .commit(
                    Some("HEAD"),
                    &sig,
                    &sig,
                    "Testing testing 1 2 3",
                    &repo.find_tree(idx.write_tree().unwrap()).unwrap(),
                    &[],
                )
                .unwrap();

            let binding = repo
                .find_commit(commit_oid)
                .unwrap()
                .into_object()
                .short_id()
                .unwrap();

            let commit_oid_short = binding.as_str().unwrap();

            let commit_hash = format!("{}", commit_oid);
            let commit_hash_short = commit_oid_short.to_string();

            assert!(commit_hash.starts_with(&commit_hash_short));

            // The commit, the commit-id is something and the repo is not dirty
            let repo_info = super::get_repo_info(&project_root).unwrap();
            assert!(!repo_info.tag.unwrap().is_empty());
            assert_eq!(repo_info.dirty, Some(false));

            // Tag the commit, it should be retrieved
            repo.tag(
                "foobar",
                &repo
                    .find_object(commit_oid, Some(git2::ObjectType::Commit))
                    .unwrap(),
                &sig,
                "Tagged foobar",
                false,
            )
            .unwrap();

            let repo_info = super::get_repo_info(&project_root).unwrap();
            assert_eq!(repo_info.tag, Some(String::from("foobar")));
            assert_eq!(repo_info.dirty, Some(false));

            // Make some dirt
            std::fs::write(cruft_file, "now dirty").unwrap();
            let repo_info = super::get_repo_info(&project_root).unwrap();
            assert_eq!(repo_info.tag, Some(String::from("foobar")));
            assert_eq!(repo_info.dirty, Some(true));

            let branch_short_name = "baz";
            let branch_name = "refs/heads/baz";
            let commit = repo.find_commit(commit_oid).unwrap();
            repo.branch(branch_short_name, &commit, true).unwrap();
            repo.set_head(branch_name).unwrap();

            let repo_info = super::get_repo_info(&project_root).unwrap();
            assert_eq!(repo_info.branch, Some(branch_name.to_owned()));
            assert_eq!(repo_info.commit_id, Some(commit_hash));
            assert_eq!(repo_info.commit_id_short, Some(commit_hash_short));
        }
    }
}

/// Selects which information `built` should retrieve and write as Rust code.
/// Used in conjunction with [`write_built_file_with_opts`][wrt].
///
/// [wrt]: fn.write_built_file_with_opts.html
#[allow(unused)]
pub struct Options {
    compiler: bool,
    git: bool,
    ci: bool,
    env: bool,
    deps: bool,
    features: bool,
    time: bool,
    cfg: bool,
}

impl Default for Options {
    /// A new struct with almost all options enabled.
    ///
    /// Parsing and writing information about dependencies is disabled by default
    /// because this information is not available for crates compiled as dependencies;
    /// only the top-level crate gets to do this. See `set_dependencies()` for
    /// further information.
    ///
    fn default() -> Options {
        Options {
            compiler: true,
            git: true,
            ci: true,
            env: true,
            deps: false,
            features: true,
            time: true,
            cfg: true,
        }
    }
}

impl Options {
    /// Detecting and writing the version of `RUSTC` and `RUSTDOC`.
    ///
    /// Call the values of `RUSTC` and `RUSTDOC` as provided by Cargo to get a version string. The
    /// result will something like
    ///
    /// ```rust,no_run
    /// pub const RUSTC_VERSION: &str = "rustc 1.15.0";
    /// pub const RUSTDOC_VERSION: &str = "rustdoc 1.15.0";
    /// ```
    pub fn set_compiler(&mut self, enabled: bool) -> &mut Self {
        self.compiler = enabled;
        self
    }

    /// Detecting and writing the tag or commit-id and a flag to indicate
    /// a dirty working directory of the crate's git repository (if any).
    ///
    /// This option is only available if `built` was compiled with the
    /// `git2` feature.
    ///
    /// Try to open the git-repository at `manifest_location` and retrieve `HEAD`
    /// tag or commit id.  The result will be something like
    ///
    /// ```rust,no_run
    /// pub const GIT_VERSION: Option<&str> = Some("0.1");
    /// pub const GIT_DIRTY: Option<bool> = Some(false);
    /// pub const GIT_COMMIT_HASH: Option<&str> = Some("18b2eabfb47998c296f9d5183f617f1b1cc2d321");
    /// pub const GIT_COMMIT_HASH_SHORT: Option<&str> = Some("18b2eab");
    /// pub const GIT_HEAD_REF: Option<&str> = Some("refs/heads/master");
    /// ```
    ///
    /// Notice that `GIT_HEAD_REF` is `None` if `HEAD` is detached or not valid UTF-8.
    ///
    /// Continuous Integration platforms like `Travis` and `AppVeyor` will
    /// do shallow clones, causing `libgit2` to be unable to get a meaningful
    /// result. `GIT_VERSION` and `GIT_DIRTY` will therefor always be `None` if
    /// a CI-platform is detected.
    ///
    #[cfg(any(feature = "git2", feature = "gitoxide"))]
    pub fn set_git(&mut self, enabled: bool) -> &mut Self {
        self.git = enabled;
        self
    }

    /// Detecting and writing the Continuous Integration Platforms we are running on.
    ///
    /// Detect various CI-platforms (named or not) and write something like
    ///
    /// ```rust
    /// pub const CI_PLATFORM: Option<&str> = Some("AppVeyor");
    /// ```
    pub fn set_ci(&mut self, enabled: bool) -> &mut Self {
        self.ci = enabled;
        self
    }

    /// Detecting various information provided through the environment, especially Cargo.
    ///
    /// `built` writes something like
    ///
    /// ```rust,no_run
    /// #[doc="The full version."]
    /// pub const PKG_VERSION: &str = "1.2.3-rc1";
    /// #[doc="The major version."]
    /// pub const PKG_VERSION_MAJOR: &str = "1";
    /// #[doc="The minor version."]
    /// pub const PKG_VERSION_MINOR: &str = "2";
    /// #[doc="The patch version."]
    /// pub const PKG_VERSION_PATCH: &str = "3";
    /// #[doc="The pre-release version."]
    /// pub const PKG_VERSION_PRE: &str = "rc1";
    /// #[doc="A colon-separated list of authors."]
    /// pub const PKG_AUTHORS: &str = "Joe:Bob:Harry:Potter";
    /// #[doc="The name of the package."]
    /// pub const PKG_NAME: &str = "testbox";
    /// #[doc="The description."]
    /// pub const PKG_DESCRIPTION: &str = "xobtset";
    /// #[doc="The home page."]
    /// pub const PKG_HOMEPAGE: &str = "localhost";
    /// #[doc="The target triple that was being compiled for."]
    /// pub const TARGET: &str = "x86_64-apple-darwin";
    /// #[doc="The host triple of the rust compiler."]
    /// pub const HOST: &str = "x86_64-apple-darwin";
    /// #[doc="`release` for release builds, `debug` for other builds."]
    /// pub const PROFILE: &str = "debug";
    /// #[doc="The compiler that cargo resolved to use."]
    /// pub const RUSTC: &str = "rustc";
    /// #[doc="The documentation generator that cargo resolved to use."]
    /// pub const RUSTDOC: &str = "rustdoc";
    /// #[doc="Value of OPT_LEVEL for the profile used during compilation."]
    /// pub const OPT_LEVEL: &str = "0";
    /// #[doc="The parallelism that was specified during compilation."]
    /// pub const NUM_JOBS: u32 = 8;
    /// #[doc="Value of DEBUG for the profile used during compilation."]
    /// pub const DEBUG: bool = true;
    /// ```
    ///
    pub fn set_env(&mut self, enabled: bool) -> &mut Self {
        self.env = enabled;
        self
    }

    /// Parsing `Cargo.lock`and writing lists of dependencies and their versions.
    ///
    /// For this to work, `Cargo.lock` needs to actually be there; this is (usually)
    /// only true for executables and not for libraries. Cargo will only create a
    /// `Cargo.lock` for the top-level crate in a dependency-tree. In case
    /// of a library, the top-level crate will decide which crate/version
    /// combination to compile and there will be no `Cargo.lock` while the library
    /// gets compiled as a dependency.
    ///
    /// Parsing `Cargo.lock` instead of `Cargo.toml` allows us to serialize the
    /// precise versions Cargo chose to compile. One can't, however, distinguish
    /// `build-dependencies`, `dev-dependencies` and `dependencies`. Furthermore,
    /// some dependencies never show up if Cargo had not been forced to
    /// actually use them (e.g. `dev-dependencies` with `cargo test` never
    /// having been executed).
    ///
    /// ```rust,no_run
    /// /// An array of effective dependencies as documented by `Cargo.lock`
    /// pub const DEPENDENCIES: [(&str, &str); 2] = [("built", "0.1.0"), ("time", "0.1.36")];
    /// /// The effective dependencies as a comma-separated string.
    /// pub const DEPENDENCIES_STR: &str = "built 0.1.0, time 0.1.36";
    /// ```
    pub fn set_dependencies(&mut self, enabled: bool) -> &mut Self {
        self.deps = enabled;
        self
    }

    /// Writing features enabled during build.
    ///
    /// One should not rely on this besides convenient debug output. If the runtime
    /// depends on enabled features, use `#[cfg(feature = "foo")]` instead.
    ///
    /// ```rust,no_run
    /// /// The features that were enabled during compilation.
    /// pub const FEATURES: [&str; 2] = ["DEFAULT", "WAYLAND"];
    /// /// The features as a comma-separated string.
    /// pub const FEATURES_STR: &str = "DEFAULT, WAYLAND";
    /// ```
    pub fn set_features(&mut self, enabled: bool) -> &mut Self {
        self.features = enabled;
        self
    }

    /// Writing the current timestamp.
    ///
    /// This option is only available if `built` is compiled with the
    /// `chrono` feature.
    ///
    /// If `built` is included as a runtime-dependency, it can parse the
    /// string-representation into a `time:Tm` with the help
    /// of `built::util::strptime()`.
    ///
    /// ```rust,no_run
    /// /// The built-time in RFC822, UTC
    /// pub const BUILT_TIME_UTC: &str = "Tue, 14 Feb 2017 01:12:35 GMT";
    /// ```
    #[cfg(feature = "chrono")]
    pub fn set_time(&mut self, enabled: bool) -> &mut Self {
        self.time = enabled;
        self
    }

    /// Writing the configuration attributes.
    ///
    /// `built` writes something like
    ///
    /// ```rust,no_run
    /// /// The target architecture, given by `CARGO_CFG_TARGET_ARCH`.
    /// pub const CFG_TARGET_ARCH: &str = "x86_64";
    /// /// The endianness, given by `CARGO_CFG_TARGET_ENDIAN`.
    /// pub const CFG_ENDIAN: &str = "little";
    /// /// The toolchain-environment, given by `CARGO_CFG_TARGET_ENV`.
    /// pub const CFG_ENV: &str = "gnu";
    /// /// The OS-family, given by `CARGO_CFG_TARGET_FAMILY`.
    /// pub const CFG_FAMILY: &str = "unix";
    /// /// The operating system, given by `CARGO_CFG_TARGET_OS`.
    /// pub const CFG_OS: &str = "linux";
    /// /// The pointer width, given by `CARGO_CFG_TARGET_POINTER_WIDTH`.
    /// pub const CFG_POINTER_WIDTH: &str = "64";
    /// ```
    pub fn set_cfg(&mut self, enabled: bool) -> &mut Self {
        self.cfg = enabled;
        self
    }
}

/// Writes rust-code describing the crate at `manifest_location` to a new file named `dst`.
///
/// # Errors
/// The function returns an error if the file at `dst` already exists or can't
/// be written to. This should not be a concern if the filename points to
/// `OUR_DIR`.
pub fn write_built_file_with_opts(
    options: &Options,
    manifest_location: &path::Path,
    dst: &path::Path,
) -> io::Result<()> {
    let mut built_file = fs::File::create(dst)?;
    built_file.write_all(
        r#"//
// EVERYTHING BELOW THIS POINT WAS AUTO-GENERATED DURING COMPILATION. DO NOT MODIFY.
//
"#
        .as_ref(),
    )?;

    macro_rules! o {
        ($i:ident, $b:stmt) => {
            if options.$i {
                $b
            }
        };
    }
    if options.ci || options.env || options.features || options.compiler {
        let envmap = get_environment();
        o!(ci, write_ci(&envmap, &mut built_file)?);
        o!(env, write_env(&envmap, &mut built_file)?);
        o!(features, write_features(&envmap, &mut built_file)?);
        o!(
            compiler,
            write_compiler_version(
                envmap["RUSTC"].as_ref(),
                envmap["RUSTDOC"].as_ref(),
                &mut built_file
            )?
        );
        #[cfg(all(feature = "git2", not(feature = "gitoxide")))]
        {
            o!(git, write_git_version(manifest_location, &mut built_file)?);
        }
        #[cfg(feature = "gitoxide")]
        {
            o!(
                git,
                gitoxide_impls::write_git_version(manifest_location, &mut built_file)?
            );
        }
    }
    o!(
        deps,
        write_dependencies(manifest_location, &mut built_file)?
    );
    #[cfg(feature = "chrono")]
    {
        o!(time, write_time(&mut built_file)?);
    }
    o!(cfg, write_cfg(&mut built_file)?);
    built_file.write_all(
        r#"//
// EVERYTHING ABOVE THIS POINT WAS AUTO-GENERATED DURING COMPILATION. DO NOT MODIFY.
//
"#
        .as_ref(),
    )?;
    Ok(())
}

/// A shorthand for calling `write_built_file()` with `CARGO_MANIFEST_DIR` and
/// `[OUT_DIR]/built.rs`.
///
/// # Errors
/// Same as `write_built_file_with_opts()`.
///
/// # Panics
/// If `CARGO_MANIFEST_DIR` or `OUT_DIR` are not set.
pub fn write_built_file() -> io::Result<()> {
    let src = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dst = path::Path::new(&env::var("OUT_DIR").unwrap()).join("built.rs");
    write_built_file_with_opts(&Options::default(), src.as_ref(), &dst)?;
    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    #[cfg(all(feature = "git2", not(feature = "gitoxide")))]
    fn parse_git_repo() {
        use super::util;
        use std::fs;
        use std::path;

        let repo_root = tempfile::tempdir().unwrap();
        assert_eq!(util::get_repo_description(repo_root.as_ref()), Ok(None));

        let repo = git2::Repository::init_opts(
            &repo_root,
            git2::RepositoryInitOptions::new()
                .external_template(false)
                .mkdir(false)
                .no_reinit(true)
                .mkpath(false),
        )
        .unwrap();

        let cruft_file = repo_root.path().join("cruftfile");
        std::fs::write(&cruft_file, "Who? Me?").unwrap();

        let project_root = repo_root.path().join("project_root");
        fs::create_dir(&project_root).unwrap();

        let sig = git2::Signature::now("foo", "bar").unwrap();
        let mut idx = repo.index().unwrap();
        idx.add_path(path::Path::new("cruftfile")).unwrap();
        idx.write().unwrap();
        let commit_oid = repo
            .commit(
                Some("HEAD"),
                &sig,
                &sig,
                "Testing testing 1 2 3",
                &repo.find_tree(idx.write_tree().unwrap()).unwrap(),
                &[],
            )
            .unwrap();

        let binding = repo
            .find_commit(commit_oid)
            .unwrap()
            .into_object()
            .short_id()
            .unwrap();

        let commit_oid_short = binding.as_str().unwrap();

        let commit_hash = format!("{commit_oid}");
        let commit_hash_short = commit_oid_short.to_string();

        assert!(commit_hash.starts_with(&commit_hash_short));

        // The commit, the commit-id is something and the repo is not dirty
        let (tag, dirty) = util::get_repo_description(&project_root).unwrap().unwrap();
        assert!(!tag.is_empty());
        assert!(!dirty);

        // Tag the commit, it should be retrieved
        repo.tag(
            "foobar",
            &repo
                .find_object(commit_oid, Some(git2::ObjectType::Commit))
                .unwrap(),
            &sig,
            "Tagged foobar",
            false,
        )
        .unwrap();

        let (tag, dirty) = util::get_repo_description(&project_root).unwrap().unwrap();
        assert_eq!(tag, "foobar");
        assert!(!dirty);

        // Make some dirt
        std::fs::write(cruft_file, "now dirty").unwrap();
        let (tag, dirty) = util::get_repo_description(&project_root).unwrap().unwrap();
        assert_eq!(tag, "foobar");
        assert!(dirty);

        let branch_short_name = "baz";
        let branch_name = "refs/heads/baz";
        let commit = repo.find_commit(commit_oid).unwrap();
        repo.branch(branch_short_name, &commit, true).unwrap();
        repo.set_head(branch_name).unwrap();

        assert_eq!(
            util::get_repo_head(&project_root),
            Ok(Some((
                Some(branch_name.to_owned()),
                commit_hash,
                commit_hash_short
            )))
        );
    }

    #[test]
    #[cfg(all(feature = "git2", not(feature = "gitoxide")))]
    fn detached_head_repo() {
        let repo_root = tempfile::tempdir().unwrap();
        let repo = git2::Repository::init_opts(
            &repo_root,
            git2::RepositoryInitOptions::new()
                .external_template(false)
                .mkdir(false)
                .no_reinit(true)
                .mkpath(false),
        )
        .unwrap();
        let sig = git2::Signature::now("foo", "bar").unwrap();
        let commit_oid = repo
            .commit(
                Some("HEAD"),
                &sig,
                &sig,
                "Testing",
                &repo
                    .find_tree(repo.index().unwrap().write_tree().unwrap())
                    .unwrap(),
                &[],
            )
            .unwrap();

        let binding = repo
            .find_commit(commit_oid)
            .unwrap()
            .into_object()
            .short_id()
            .unwrap();

        let commit_oid_short = binding.as_str().unwrap();

        let commit_hash = format!("{commit_oid}");
        let commit_hash_short = commit_oid_short.to_string();

        assert!(commit_hash.starts_with(&commit_hash_short));

        repo.set_head_detached(commit_oid).unwrap();
        assert_eq!(
            super::util::get_repo_head(repo_root.as_ref()),
            Ok(Some((None, commit_hash, commit_hash_short)))
        );
    }

    #[test]
    fn parse_deps() {
        let lock_toml_buf = r#"
            [root]
            name = "foobar"
            version = "1.0.0"
            dependencies = [
                "normal_dep 1.2.3",
                "local_dep 4.5.6",
            ]

            [[package]]
            name = "normal_dep"
            version = "1.2.3"
            dependencies = [
                "dep_of_dep 7.8.9",
            ]

            [[package]]
            name = "local_dep"
            version = "4.5.6"

            [[package]]
            name = "dep_of_dep"
            version = "7.8.9""#;
        let deps = super::parse_dependencies(lock_toml_buf);
        assert_eq!(
            deps,
            [
                ("dep_of_dep".to_owned(), "7.8.9".to_owned()),
                ("local_dep".to_owned(), "4.5.6".to_owned()),
                ("normal_dep".to_owned(), "1.2.3".to_owned()),
            ]
        );
    }
}
