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
//! about the build-environment, serialize this information into Rust-code and
//! provide that to the crate. The information collected by `built` include:
//!
//!  * Various metadata like version, authors, homepage etc. as set by `Cargo.toml`
//!  * The tag or commit id if the crate was being compiled from within a Git repository.
//!  * The values of `CARGO_CFG_*` build script environment variables, like `CARGO_CFG_TARGET_OS` and `CARGO_CFG_TARGET_ARCH`.
//!  * The features the crate was compiled with.
//!  * The various dependencies, dependencies of dependencies and their versions Cargo ultimately chose to compile.
//!  * The presence of a CI-platform like `Github Actions`, `Travis CI` and `AppVeyor`.
//!  * The compiler and it's version; the documentation-generator and it's version.
//!
//! `built` does not add any further runtime-dependencies to a crate; all information
//! is serialized as types from `stdlib`. One can include `built` as a
//! runtime-dependency and use it's convenience functions.
//!
//! To add `built` to a crate, add it as a build-time dependency, use a build-script
//! to collect and serialize the build-time information, and `include!` the generated code.
//!
//! Add this to `Cargo.toml`:
//!
//! ```toml
//! [package]
//! build = "build.rs"
//!
//! [build-dependencies]
//! built = "0.7"
//! ```
//!
//! Add or modify a build-script. In `build.rs`:
//!
//! ```rust,no_run
//! fn main() {
//!     built::write_built_file().expect("Failed to acquire build-time information");
//! }
//! ```
//!
//! The build-script will by default write a file named `built.rs` into Cargo's output
//! directory. It can be picked up in `main.rs` (or anywhere else) like this:
//!
//! ```rust,ignore
//! // Use of a mod or pub mod is not actually necessary.
//! pub mod built_info {
//!    // The file has been placed there by the build script.
//!    include!(concat!(env!("OUT_DIR"), "/built.rs"));
//! }
//! ```
//!
//! ...and then used somewhere in the crate's code:
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
//! ---
//!
//! ## Feature flags
//! The information that `built` collects and makes available in `built.rs` depends
//! on the features that were enabled on the build-time dependency.
//!
//! ### Always available
//! The following information is available regardless of feature-flags.
//!
//! ```
//! /// The Continuous Integration platform detected during compilation.
//! pub const CI_PLATFORM: Option<&str> = None;
//!
//! /// The full version.
//! pub const PKG_VERSION: &str = "0.1.0";
//! /// The major version.
//! pub const PKG_VERSION_MAJOR: &str = "0";
//! /// The minor version.
//! pub const PKG_VERSION_MINOR: &str = "1";
//! /// "The patch version.
//! pub const PKG_VERSION_PATCH: &str = "0";
//! /// "The pre-release version.
//! pub const PKG_VERSION_PRE: &str = "";
//!
//! /// "A colon-separated list of authors.
//! pub const PKG_AUTHORS: &str = "Lukas Lueg <lukas.lueg@gmail.com>";
//!
//! /// The name of the package.
//! pub const PKG_NAME: &str = "example_project";
//! /// "The description.
//! pub const PKG_DESCRIPTION: &str = "";
//! /// "The homepage.
//! pub const PKG_HOMEPAGE: &str = "";
//! /// "The license.
//! pub const PKG_LICENSE: &str = "MIT";
//! /// The source repository as advertised in Cargo.toml.
//! pub const PKG_REPOSITORY: &str = "";
//!
//! /// The target triple that was being compiled for.
//! pub const TARGET: &str = "x86_64-unknown-linux-gnu";
//! /// The host triple of the rust compiler.
//! pub const HOST: &str = "x86_64-unknown-linux-gnu";
//! /// `release` for release builds, `debug` for other builds.
//! pub const PROFILE: &str = "debug";
//!
//! /// The compiler that cargo resolved to use.
//! pub const RUSTC: &str = "rustc";
//! /// The documentation-generator that cargo resolved to use.
//! pub const RUSTDOC: &str = "rustdoc";
//! /// The output of `rustc -V`
//! pub const RUSTC_VERSION: &str = "rustc 1.43.1 (8d69840ab 2020-05-04)";
//! /// The output of `rustdoc -V`
//! pub const RUSTDOC_VERSION: &str = "rustdoc 1.43.1 (8d69840ab 2020-05-04)";
//!
//! /// Value of OPT_LEVEL for the profile used during compilation.
//! pub const OPT_LEVEL: &str = "0";
//! /// The parallelism that was specified during compilation.
//! pub const NUM_JOBS: u32 = 8;
//! /// "Value of DEBUG for the profile used during compilation.
//! pub const DEBUG: bool = true;
//!
//! /// The features that were enabled during compilation.
//! pub const FEATURES: [&str; 0] = [];
//! /// The features as a comma-separated string.
//! pub const FEATURES_STR: &str = "";
//! /// The features as above, as lowercase strings.
//! pub const FEATURES_LOWERCASE: [&str; 0] = [];
//! /// The feature-string as above, from lowercase strings.
//! pub const FEATURES_LOWERCASE_STR: &str = "";
//!
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
//!
//! ### `cargo-lock`
//! Parses `Cargo.lock`and generates representations of  dependencies and their versions.
//!
//! For this to work, `Cargo.lock` needs to actually be there; this is (usually)
//! only true for executables and not for libraries. Cargo will only create a
//! `Cargo.lock` for the top-level crate in a dependency-tree. In case
//! of a library, the top-level crate will decide which crate/version
//! combination to compile and there will be no `Cargo.lock` while the library
//! gets compiled as a dependency.
//!
//! Parsing `Cargo.lock` instead of `Cargo.toml` allows to serialize the
//! precise versions Cargo chose to compile. One can't, however, distinguish
//! `build-dependencies`, `dev-dependencies` and `dependencies`. Furthermore,
//! some dependencies never show up if Cargo had not been forced to
//! actually use them (e.g. `dev-dependencies` with `cargo test` never
//! having been executed).
//!
//! Note that if the `dependency-tree`-feature is not active, the list of dependencies
//! contains the root-package(s) as well.
//!
//! ```
//! /// An array of effective dependencies as documented by `Cargo.lock`.
//! pub const DEPENDENCIES: [(&str, &str); 37] = [("autocfg", "1.0.0"), ("bitflags", "1.2.1"), ("built", "0.4.1"), ("cargo-lock", "4.0.1"), ("cc", "1.0.54"), ("cfg-if", "0.1.10"), ("chrono", "0.4.11"), ("example_project", "0.1.0"), ("git2", "0.13.6"), ("idna", "0.2.0"), ("jobserver", "0.1.21"), ("libc", "0.2.71"), ("libgit2-sys", "0.12.6+1.0.0"), ("libz-sys", "1.0.25"), ("log", "0.4.8"), ("matches", "0.1.8"), ("num-integer", "0.1.42"), ("num-traits", "0.2.11"), ("percent-encoding", "2.1.0"), ("pkg-config", "0.3.17"), ("proc-macro2", "1.0.17"), ("quote", "1.0.6"), ("semver", "1.0.0"), ("serde", "1.0.110"), ("serde_derive", "1.0.110"), ("smallvec", "1.4.0"), ("syn", "1.0.25"), ("time", "0.1.43"), ("toml", "0.5.6"), ("unicode-bidi", "0.3.4"), ("unicode-normalization", "0.1.12"), ("unicode-xid", "0.2.0"), ("url", "2.1.1"), ("vcpkg", "0.2.8"), ("winapi", "0.3.8"), ("winapi-i686-pc-windows-gnu", "0.4.0"), ("winapi-x86_64-pc-windows-gnu", "0.4.0")];
//! /// The effective dependencies as a comma-separated string.
//! pub const DEPENDENCIES_STR: &str = "autocfg 1.0.0, bitflags 1.2.1, built 0.4.1, cargo-lock 4.0.1, cc 1.0.54, cfg-if 0.1.10, chrono 0.4.11, example_project 0.1.0, git2 0.13.6, idna 0.2.0, jobserver 0.1.21, libc 0.2.71, libgit2-sys 0.12.6+1.0.0, libz-sys 1.0.25, log 0.4.8, matches 0.1.8, num-integer 0.1.42, num-traits 0.2.11, percent-encoding 2.1.0, pkg-config 0.3.17, proc-macro2 1.0.17, quote 1.0.6, semver 1.0.0, serde 1.0.110, serde_derive 1.0.110, smallvec 1.4.0, syn 1.0.25, time 0.1.43, toml 0.5.6, unicode-bidi 0.3.4, unicode-normalization 0.1.12, unicode-xid 0.2.0, url 2.1.1, vcpkg 0.2.8, winapi 0.3.8, winapi-i686-pc-windows-gnu 0.4.0, winapi-x86_64-pc-windows-gnu 0.4.0";
//! ```
//!
//! ### `dependency-tree` (implies `cargo-lock`)
//! Solve the dependency-graph in `Cargo.lock` to discern direct and indirect
//! dependencies.
//!
//! "Direct" dependencies are those which the root-package(s) depends on.
//! "Indirect" dependencies are those which are not direct dependencies.
//!
//! ```
//! /// An array of direct dependencies as documented by `Cargo.lock`.
//! pub const DIRECT_DEPENDENCIES: [(&str, &str); 1] = [("built", "0.6.1")];
//! /// The direct dependencies as a comma-separated string.
//! pub const DIRECT_DEPENDENCIES_STR: &str = r"built 0.6.1";
//!
//! /// An array of indirect dependencies as documented by `Cargo.lock`.
//! pub const INDIRECT_DEPENDENCIES: [(&str, &str); 64] = [("android-tzdata", "0.1.1"), ("android_system_properties", "0.1.5"), ("autocfg", "1.1.0"), ("bitflags", "2.4.0"), ("bumpalo", "3.13.0"), ("cargo-lock", "9.0.0"), ("cc", "1.0.83"), ("cfg-if", "1.0.0"), ("chrono", "0.4.29"), ("core-foundation-sys", "0.8.4"), ("equivalent", "1.0.1"), ("example_project", "0.1.0"), ("fixedbitset", "0.4.2"), ("form_urlencoded", "1.2.0"), ("git2", "0.18.0"), ("hashbrown", "0.14.0"), ("iana-time-zone", "0.1.57"), ("iana-time-zone-haiku", "0.1.2"), ("idna", "0.4.0"), ("indexmap", "2.0.0"), ("jobserver", "0.1.26"), ("js-sys", "0.3.64"), ("libc", "0.2.147"), ("libgit2-sys", "0.16.1+1.7.1"), ("libz-sys", "1.1.12"), ("log", "0.4.20"), ("memchr", "2.6.3"), ("num-traits", "0.2.16"), ("once_cell", "1.18.0"), ("percent-encoding", "2.3.0"), ("petgraph", "0.6.4"), ("pkg-config", "0.3.27"), ("proc-macro2", "1.0.66"), ("quote", "1.0.33"), ("semver", "1.0.18"), ("serde", "1.0.188"), ("serde_derive", "1.0.188"), ("serde_spanned", "0.6.3"), ("syn", "2.0.31"), ("tinyvec", "1.6.0"), ("tinyvec_macros", "0.1.1"), ("toml", "0.7.6"), ("toml_datetime", "0.6.3"), ("toml_edit", "0.19.14"), ("unicode-bidi", "0.3.13"), ("unicode-ident", "1.0.11"), ("unicode-normalization", "0.1.22"), ("url", "2.4.1"), ("vcpkg", "0.2.15"), ("wasm-bindgen", "0.2.87"), ("wasm-bindgen-backend", "0.2.87"), ("wasm-bindgen-macro", "0.2.87"), ("wasm-bindgen-macro-support", "0.2.87"), ("wasm-bindgen-shared", "0.2.87"), ("windows", "0.48.0"), ("windows-targets", "0.48.5"), ("windows_aarch64_gnullvm", "0.48.5"), ("windows_aarch64_msvc", "0.48.5"), ("windows_i686_gnu", "0.48.5"), ("windows_i686_msvc", "0.48.5"), ("windows_x86_64_gnu", "0.48.5"), ("windows_x86_64_gnullvm", "0.48.5"), ("windows_x86_64_msvc", "0.48.5"), ("winnow", "0.5.15")];
//! /// The indirect dependencies as a comma-separated string.
//! pub const INDIRECT_DEPENDENCIES_STR: &str = r"android-tzdata 0.1.1, android_system_properties 0.1.5, autocfg 1.1.0, bitflags 2.4.0, bumpalo 3.13.0, cargo-lock 9.0.0, cc 1.0.83, cfg-if 1.0.0, chrono 0.4.29, core-foundation-sys 0.8.4, equivalent 1.0.1, example_project 0.1.0, fixedbitset 0.4.2, form_urlencoded 1.2.0, git2 0.18.0, hashbrown 0.14.0, iana-time-zone 0.1.57, iana-time-zone-haiku 0.1.2, idna 0.4.0, indexmap 2.0.0, jobserver 0.1.26, js-sys 0.3.64, libc 0.2.147, libgit2-sys 0.16.1+1.7.1, libz-sys 1.1.12, log 0.4.20, memchr 2.6.3, num-traits 0.2.16, once_cell 1.18.0, percent-encoding 2.3.0, petgraph 0.6.4, pkg-config 0.3.27, proc-macro2 1.0.66, quote 1.0.33, semver 1.0.18, serde 1.0.188, serde_derive 1.0.188, serde_spanned 0.6.3, syn 2.0.31, tinyvec 1.6.0, tinyvec_macros 0.1.1, toml 0.7.6, toml_datetime 0.6.3, toml_edit 0.19.14, unicode-bidi 0.3.13, unicode-ident 1.0.11, unicode-normalization 0.1.22, url 2.4.1, vcpkg 0.2.15, wasm-bindgen 0.2.87, wasm-bindgen-backend 0.2.87, wasm-bindgen-macro 0.2.87, wasm-bindgen-macro-support 0.2.87, wasm-bindgen-shared 0.2.87, windows 0.48.0, windows-targets 0.48.5, windows_aarch64_gnullvm 0.48.5, windows_aarch64_msvc 0.48.5, windows_i686_gnu 0.48.5, windows_i686_msvc 0.48.5, windows_x86_64_gnu 0.48.5, windows_x86_64_gnullvm 0.48.5, windows_x86_64_msvc 0.48.5, winnow 0.5.15";
//! ```
//!
//! ### `git2`
//! Try to open the git-repository at `manifest_location` and retrieve `HEAD`
//! tag or commit id.
//!
//! Notice that `GIT_HEAD_REF` is `None` if `HEAD` is detached or not valid UTF-8.
//!
//! Continuous Integration platforms like `Travis` and `AppVeyor` will
//! do shallow clones, causing `libgit2` to be unable to get a meaningful
//! result. `GIT_VERSION` and `GIT_DIRTY` will therefor always be `None` if
//! a CI-platform is detected.
//! ```
//! /// If the crate was compiled from within a git-repository,
//! /// `GIT_VERSION` contains HEAD's tag. The short commit id is used
//! /// if HEAD is not tagged.
//! pub const GIT_VERSION: Option<&str> = Some("0.4.1-10-gca2af4f");
//!
//! /// If the repository had dirty/staged files.
//! pub const GIT_DIRTY: Option<bool> = Some(true);
//!
//! /// If the crate was compiled from within a git-repository,
//! /// `GIT_HEAD_REF` contains full name to the reference pointed to by
//! /// HEAD (e.g.: `refs/heads/master`). If HEAD is detached or the branch
//! /// name is not valid UTF-8 `None` will be stored.
//! pub const GIT_HEAD_REF: Option<&str> = Some("refs/heads/master");
//!
//! /// If the crate was compiled from within a git-repository,
//! /// `GIT_COMMIT_HASH` contains HEAD's full commit SHA-1 hash.
//! pub const GIT_COMMIT_HASH: Option<&str> = Some("ca2af4f11bb8f4f6421c4cccf428bf4862573daf");
//!
//! /// If the crate was compiled from within a git-repository,
//! /// `GIT_COMMIT_HASH_SHORT` contains HEAD's short commit SHA-1 hash.
//! pub const GIT_COMMIT_HASH_SHORT: Option<&str> = Some("ca2af4f");
//! ```
//!
//! ### `chrono`
//!
//! The build-time is recorded as `BUILT_TIME_UTC`. If `built` is included as a runtime-dependency,
//! it can parse the string-representation into a `time:Tm` with the help of
//! `built::util::strptime()`.
//! ```
//! /// The built-time in RFC2822, UTC
//! pub const BUILT_TIME_UTC: &str = "Wed, 27 May 2020 18:12:39 +0000";
//! ```

#[cfg(feature = "cargo-lock")]
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

/// Writes rust-code describing the crate at `manifest_location` to a new file named `dst`.
///
/// # Errors
/// The function returns an error if the file at `dst` already exists or can't
/// be written to. This should not be a concern if the filename points to
/// `OUR_DIR`.
pub fn write_built_file_with_opts(
    #[cfg(any(feature = "cargo-lock", feature = "git2"))] manifest_location: Option<&path::Path>,
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

    let envmap = environment::EnvironmentMap::new();
    envmap.write_ci(&built_file)?;
    envmap.write_env(&built_file)?;
    envmap.write_features(&built_file)?;
    envmap.write_compiler_version(&built_file)?;
    envmap.write_cfg(&built_file)?;

    #[cfg(feature = "git2")]
    {
        if let Some(manifest_location) = manifest_location {
            git::write_git_version(manifest_location, &built_file)?;
        }
    }

    #[cfg(feature = "cargo-lock")]
    if let Some(manifest_location) = manifest_location {
        dependencies::write_dependencies(manifest_location, &built_file)?;
    }

    #[cfg(feature = "chrono")]
    krono::write_time(&built_file)?;

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
    let dst = path::Path::new(&env::var("OUT_DIR").expect("OUT_DIR not set")).join("built.rs");
    write_built_file_with_opts(
        #[cfg(any(feature = "cargo-lock", feature = "git2"))]
        Some(
            env::var("CARGO_MANIFEST_DIR")
                .expect("CARGO_MANIFEST_PATH")
                .as_ref(),
        ),
        &dst,
    )?;
    Ok(())
}
