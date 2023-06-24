// MIT License
//
// Copyright (c) Lukas Lueg <lukas.lueg@gmail.com>
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
//! built = "0.6"
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
//! /// The features as above, as lowercase strings.
//! pub const FEATURES_LOWERCASE: [&str; 0] = [];
//! /// The feature-string as above, from lowercase strings.
//! pub const FEATURES_LOWERCASE_STR: &str = "";
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

mod dependencies;
mod environment;
#[cfg(feature = "git2")]
mod git;
#[cfg(feature = "chrono")]
mod krono;
pub mod util;

use std::{env, fmt, fs, io, io::Write, path};

#[cfg(feature = "semver")]
pub use semver;

#[cfg(feature = "chrono")]
pub use chrono;

pub use environment::CIPlatform;

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
pub(crate) use write_variable;

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
pub(crate) use write_str_variable;

pub(crate) fn fmt_option_str<S: fmt::Display>(o: Option<S>) -> String {
    match o {
        Some(s) => format!("Some(\"{s}\")"),
        None => "None".to_owned(),
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
    #[cfg(feature = "git2")]
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
    /// /// The features as above, as lowercase strings.
    /// pub const FEATURES_LOWERCASE: [&str; 2] = ["default", "wayland"];
    /// /// The feature-string as above, from lowercase strings.
    /// pub const FEATURES_LOWERCASE_STR: &str = "default, wayland";
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
    /// /// The built-time in RFC2822, UTC
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
        let envmap = environment::EnvironmentMap::new();
        o!(ci, envmap.write_ci(&built_file)?);
        o!(env, envmap.write_env(&built_file)?);
        o!(features, envmap.write_features(&built_file)?);
        o!(compiler, envmap.write_compiler_version(&built_file)?);
        o!(cfg, envmap.write_cfg(&built_file)?);
        #[cfg(feature = "git2")]
        {
            o!(git, git::write_git_version(manifest_location, &built_file)?);
        }
    }
    o!(
        deps,
        dependencies::write_dependencies(manifest_location, &built_file)?
    );
    #[cfg(feature = "chrono")]
    {
        o!(time, krono::write_time(&built_file)?);
    }
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
