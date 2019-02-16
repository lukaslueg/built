// MIT License
//
// Copyright (c) 2017 Lukas Lueg
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
//! Provides a crate with information from the time it was built.
//!
//! `built` is used as a build-time dependency to collect various information
//! about the build environment, serialize it into Rust-code and compile
//! it into the final crate. The information collected by `built` include:
//!
//!  * Various metadata like version, authors, homepage etc. as set by `Cargo.toml`
//!  * The tag or commit id if the crate was being compiled from within a git repo.
//!  * The values of various `cfg!`, like `target_os` and `target_arch`.
//!  * The features the crate was compiled with.
//!  * The various dependencies, dependencies of dependencies and their versions
//!    cargo ultimately chose to compile.
//!  * The presence of a CI-platform like `Travis CI` and `AppVeyor`.
//!  * The used compiler and it's version; the used documentation generator and
//!    it's version.
//!
//! `built` does not add any further dependencies to a crate; all information
//! is serialized as types from `stdlib`. One can include `built` as a
//! runtime-dependency and use it's convenience functions.  The code generated
//! by `built` will not interfere with
//! `#![deny(warnings, bad_style, future_incompatible, unused, missing_docs, unused_comparisons)]`.
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
//! built = "0.3"
//! ```
//!
//! Add or modify a build script. In `build.rs`:
//!
//! ```rust,ignore
//! extern crate built;
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
//! ```rust,ignore
//! extern crate built;
//! extern crate time;
//! extern crate semver;
//!
//! if (built_info::PKG_VERSION_PRE != "" || built_info::GIT_VERSION.is_some())
//!    && (built::util::strptime(built_info::BUILT_TIME_UTC) - time::now()).num_days() > 180 {
//!     println!("You are running a development version that is really old. Update soon!");
//! }
//!
//! if built_info::CI_PLATFORM.is_some() {
//!     panic!("Muahahaha, there will be no commit for you, Peter Pan!");
//! }
//!
//! let deps = built_info::DEPENDENCIES;
//! if built::util::parse_versions(&deps)
//!                 .any(|(name, ver)| name == "DeleteAllMyFiles"
//!                                    && ver < semver::Version::parse("1.1.4").unwrap())) {
//!     warn!("DeleteAllMyFiles < 1.1.4 is known to sometimes not really delete all your files. Beware!");
//! }
//!
//! ```

#![deny(
    warnings,
    bad_style,
    future_incompatible,
    unused,
    missing_docs,
    unused_comparisons
)]

#[cfg(feature = "serialized_time")]
extern crate chrono;
#[cfg(all(test, feature = "serialized_git"))]
extern crate git2;
#[cfg(all(test, feature = "serialized_git"))]
extern crate tempdir;
use toml;

pub mod util;

use std::collections;
use std::env;
use std::ffi;
use std::fmt;
use std::fs;
use std::io;
use std::io::{Read, Write};
use std::path;
use std::process;

/// Various Continuous Integration platforms whose presence can be detected.
pub enum CIPlatform {
    /// https://travis-ci.org
    Travis,
    /// https://circleci.com
    Circle,
    /// https://about.gitlab.com/gitlab-ci/
    GitLab,
    /// https://www.appveyor.com/
    AppVeyor,
    /// https://codeship.com/
    Codeship,
    /// https://github.com/drone/drone
    Drone,
    /// https://magnum-ci.com/
    Magnum,
    /// https://semaphoreci.com/
    Semaphore,
    /// https://jenkins.io/
    Jenkins,
    /// https://www.atlassian.com/software/bamboo
    Bamboo,
    /// https://www.visualstudio.com/de/tfs/
    TFS,
    /// https://www.jetbrains.com/teamcity/
    TeamCity,
    /// https://buildkite.com/
    Buildkite,
    /// http://hudson-ci.org/
    Hudson,
    /// https://github.com/taskcluster
    TaskCluster,
    /// https://www.gocd.io/
    GoCD,
    /// https://bitbucket.org
    BitBucket,
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
            ("BITBUCKET_COMMIT", BitBucket)
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

fn get_build_deps<P: AsRef<path::Path>>(manifest_location: P) -> io::Result<Vec<(String, String)>> {
    let mut lock_buf = String::new();
    fs::File::open(manifest_location.as_ref().join("Cargo.lock"))?.read_to_string(&mut lock_buf)?;
    Ok(parse_dependencies(&lock_buf))
}

fn parse_dependencies(lock_toml_buf: &str) -> Vec<(String, String)> {
    let lock_toml: toml::Value = lock_toml_buf.parse().unwrap();
    let mut deps = Vec::new();

    // Get the table of [[package]]s. This is the deep list of dependencies and
    // dependencies of dependencies.
    for package in lock_toml["package"].as_array().unwrap() {
        let package = package.as_table().unwrap();
        deps.push((
            package.get("name").unwrap().as_str().unwrap().to_owned(),
            package.get("version").unwrap().as_str().unwrap().to_owned(),
        ));
    }
    deps.sort();
    deps
}

fn get_version_from_cmd<P: AsRef<ffi::OsStr>>(executable: P) -> io::Result<String> {
    let output = process::Command::new(executable).arg("-V").output()?;
    let mut v = String::from_utf8(output.stdout).unwrap();
    v.pop(); // remove newline
    Ok(v)
}

fn write_compiler_version<P: AsRef<ffi::OsStr> + fmt::Display, T: io::Write>(
    rustc: P,
    rustdoc: P,
    w: &mut T,
) -> io::Result<()> {
    let rustc_version = get_version_from_cmd(&rustc)?;
    let rustdoc_version = get_version_from_cmd(&rustdoc)?;

    writeln!(w, "/// The output of `{} -V`", &rustc)?;
    writeln!(w, "pub const RUSTC_VERSION: &str = \"{}\";", &rustc_version)?;
    writeln!(w, "/// The output of `{} -V`", &rustdoc)?;
    writeln!(
        w,
        "pub const RUSTDOC_VERSION: &str = \"{}\";",
        &rustdoc_version
    )?;
    Ok(())
}

fn fmt_option_str<S: fmt::Display>(o: Option<S>) -> String {
    match o {
        Some(s) => format!("Some(\"{}\")", s),
        None => "None".to_owned(),
    }
}

#[cfg(feature = "serialized_git")]
fn write_git_version<P: AsRef<path::Path>, T: io::Write>(
    manifest_location: P,
    w: &mut T,
) -> io::Result<()> {
    // CIs will do shallow clones of repositories, causing libgit2 to error
    // out. We try to detect if we are running on a CI and ignore the
    // error.
    let tag = match util::get_repo_description(&manifest_location) {
        Ok(tag) => tag,
        Err(_) => None,
    };
    w.write_all(
        b"/// If the crate was compiled from within a git-repository, `GIT_VERSION` \
contains HEAD's tag. The short commit id is used if HEAD is not tagged.\n",
    )?;
    writeln!(
        w,
        "pub const GIT_VERSION: Option<&str> = {};",
        &fmt_option_str(tag)
    )?;
    Ok(())
}

fn write_ci<T: io::Write>(envmap: &EnvironmentMap, w: &mut T) -> io::Result<()> {
    w.write_all(b"/// The Continuous Integration platform detected during compilation.\n")?;
    writeln!(
        w,
        "pub const CI_PLATFORM: Option<&str> = {};",
        &fmt_option_str(CIPlatform::detect_from_envmap(envmap))
    )?;
    Ok(())
}

fn write_features<T: io::Write>(envmap: &EnvironmentMap, w: &mut T) -> io::Result<()> {
    let prefix = "CARGO_FEATURE_";
    let mut features = Vec::new();
    for name in envmap.keys() {
        if name.starts_with(&prefix) {
            features.push(name[prefix.len()..].to_owned());
        }
    }
    features.sort();

    w.write_all(b"/// The features that were enabled during compilation.\n")?;
    writeln!(
        w,
        "pub const FEATURES: [&str; {}] = {:?};",
        features.len(),
        features
    )?;

    let features_str = features.join(", ");
    w.write_all(b"/// The features as a comma-separated string.\n")?;
    writeln!(w, "pub const FEATURES_STR: &str = \"{}\";", features_str)?;
    Ok(())
}

fn write_env<T: io::Write>(envmap: &EnvironmentMap, w: &mut T) -> io::Result<()> {
    macro_rules! write_env_str {
        ($(($name:ident, $env_name:expr,$doc:expr)),*) => {$(
            writeln!(w, "#[doc={}]\npub const {}: &str = \"{}\";",
                    stringify!($doc), stringify!($name), envmap.get($env_name).unwrap())?;
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
    writeln!(
        w,
        "#[doc=\"Value of OPT_LEVEL for the profile used during compilation.\"]\npub const \
         OPT_LEVEL: &str = \"{}\";",
        env::var("OPT_LEVEL").unwrap()
    )?;
    writeln!(
        w,
        "#[doc=\"The parallelism that was specified during compilation.\"]\npub const \
         NUM_JOBS: u32 = {};",
        env::var("NUM_JOBS").unwrap()
    )?;
    writeln!(
        w,
        "#[doc=\"Value of DEBUG for the profile used during compilation.\"]\npub const \
         DEBUG: bool = {};",
        env::var("DEBUG").unwrap() == "true"
    )?;
    Ok(())
}

fn write_dependencies<P: AsRef<path::Path>, T: io::Write>(
    manifest_location: P,
    w: &mut T,
) -> io::Result<()> {
    let deps = get_build_deps(manifest_location)?;
    w.write_all(b"/// An array of effective dependencies as documented by `Cargo.lock`.\n")?;
    writeln!(
        w,
        "pub const DEPENDENCIES: [(&str, &str); {}] = {:?};",
        deps.len(),
        deps
    )?;
    w.write_all(b"/// The effective dependencies as a comma-separated string.\n")?;
    writeln!(
        w,
        "pub const DEPENDENCIES_STR: &str = \"{}\";",
        deps.iter()
            .map(|&(ref n, ref v)| format!("{} {}", n, v))
            .collect::<Vec<_>>()
            .join(", ")
    )?;
    Ok(())
}

#[cfg(feature = "serialized_time")]
fn write_time<T: io::Write>(w: &mut T) -> io::Result<()> {
    let now = chrono::offset::Utc::now();
    w.write_all(b"/// The built-time in RFC2822, UTC\n")?;
    writeln!(
        w,
        "pub const BUILT_TIME_UTC: &str = \"{}\";",
        now.to_rfc2822()
    )?;
    Ok(())
}

fn write_cfg<T: io::Write>(w: &mut T) -> io::Result<()> {
    macro_rules! get_cfg {
        ($i:ident : $($s:expr),+) => (
            let $i = || { $( if cfg!($i=$s) { return $s; } );+ "unknown"};
        )
    }

    get_cfg!(target_arch: "x86", "x86_64", "mips", "powerpc", "powerpc64", "arm", "aarch64");
    get_cfg!(target_endian: "little", "big");
    get_cfg!(target_env: "musl", "msvc", "gnu");
    get_cfg!(target_family: "unix", "windows");
    get_cfg!(target_os: "windows", "macos", "ios", "linux", "android", "freebsd", "dragonfly",
                        "bitrig", "openbsd", "netbsd");
    get_cfg!(target_pointer_width: "32", "64");

    w.write_all(b"/// The target architecture, given by `cfg!(target_arch)`.\n")?;
    writeln!(
        w,
        "pub const CFG_TARGET_ARCH: &str = \"{}\";",
        target_arch()
    )?;

    w.write_all(b"/// The endianness, given by `cfg!(target_endian)`.\n")?;
    writeln!(w, "pub const CFG_ENDIAN: &str = \"{}\";", target_endian())?;

    w.write_all(b"/// The toolchain-environment, given by `cfg!(target_env)`.\n")?;
    writeln!(w, "pub const CFG_ENV: &str = \"{}\";", target_env())?;

    w.write_all(b"/// The OS-family, given by `cfg!(target_family)`.\n")?;
    writeln!(w, "pub const CFG_FAMILY: &str = \"{}\";", target_family())?;

    w.write_all(b"/// The operating system, given by `cfg!(target_os)`.\n")?;
    writeln!(w, "pub const CFG_OS: &str = \"{}\";", target_os())?;

    w.write_all(b"/// The pointer width, given by `cfg!(target_pointer_width)`.\n")?;
    writeln!(
        w,
        "pub const CFG_POINTER_WIDTH: &str = \"{}\";",
        target_pointer_width()
    )?;

    Ok(())
}

/// Selects which information `built` should retrieve and write as Rust code.
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

    /// Detecting and writing the tag or commit id of the crate's git repository (if any).
    ///
    /// This option is only available if `built` was compiled with the
    /// `serialized_git` feature.
    ///
    /// Try to open the git-repository at `manifest_location` and retrieve `HEAD`
    /// tag or commit id.  The result will be something like
    ///
    /// ```rust,no_run
    /// pub const GIT_VERSION: Option<&str> = Some("0.1");
    /// ```
    ///
    /// Continuous Integration platforms like `Travis` and `AppVeyor` will
    /// do shallow clones, causing `libgit2` to be unable to get a meaningful
    /// result. The `GIT_VERSION` will therefor always be `None` if a CI-platform
    /// is detected.
    ///
    #[cfg(feature = "serialized_git")]
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
    /// `serialized_time` feature.
    ///
    /// If `built` is included as a runtime-dependency, it can parse the
    /// string-representation into a `time:Tm` with the help
    /// of `built::util::strptime()`.
    ///
    /// ```rust,no_run
    /// /// The built-time in RFC822, UTC
    /// pub const BUILT_TIME_UTC: &str = "Tue, 14 Feb 2017 01:12:35 GMT";
    /// ```
    #[cfg(feature = "serialized_time")]
    pub fn set_time(&mut self, enabled: bool) -> &mut Self {
        self.time = enabled;
        self
    }

    /// Writing the configuration attributes.
    ///
    /// `built` writes something like
    ///
    /// ```rust,no_run
    /// /// The target architecture, given by `cfg!(target_arch)`.
    /// pub const CFG_TARGET_ARCH: &str = "x86_64";
    /// /// The endianness, given by `cfg!(target_endian)`.
    /// pub const CFG_ENDIAN: &str = "little";
    /// /// The toolchain-environment, given by `cfg!(target_env)`.
    /// pub const CFG_ENV: &str = "gnu";
    /// /// The OS-family, given by `cfg!(target_family)`.
    /// pub const CFG_FAMILY: &str = "unix";
    /// /// The operating system, given by `cfg!(target_os)`.
    /// pub const CFG_OS: &str = "linux";
    /// /// The pointer width, given by `cfg!(target_pointer_width)`.
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
pub fn write_built_file_with_opts<P: AsRef<path::Path>, Q: AsRef<path::Path>>(
    options: &Options,
    manifest_location: P,
    dst: Q,
) -> io::Result<()> {
    let mut built_file = fs::File::create(&dst)?;
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
            write_compiler_version(&envmap["RUSTC"], &envmap["RUSTDOC"], &mut built_file)?
        );
        #[cfg(feature = "serialized_git")]
        {
            o!(git, write_git_version(&manifest_location, &mut built_file)?);
        }
    }
    o!(
        deps,
        write_dependencies(&manifest_location, &mut built_file)?
    );
    #[cfg(feature = "serialized_time")]
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
pub fn write_built_file() -> io::Result<()> {
    let src = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dst = path::Path::new(&env::var("OUT_DIR").unwrap()).join("built.rs");
    write_built_file_with_opts(&Options::default(), &src, &dst)?;
    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    #[cfg(feature = "serialized_git")]
    fn parse_git_repo() {
        use super::git2;
        use super::tempdir;
        use super::util;
        use std::fs;
        use std::io::Write;
        use std::path;

        let repo_root = tempdir::TempDir::new("builttest").unwrap();
        assert_eq!(util::get_repo_description(&repo_root), Ok(None));

        let repo = git2::Repository::init_opts(
            &repo_root,
            git2::RepositoryInitOptions::new()
                .external_template(false)
                .mkdir(false)
                .no_reinit(true)
                .mkpath(false),
        )
        .unwrap();

        let cruft_path = repo_root.path().join("cruftfile");
        let mut cruft_file = fs::File::create(cruft_path).unwrap();
        writeln!(cruft_file, "Who? Me?").unwrap();
        drop(cruft_file);

        let project_root = repo_root.path().join("project_root");
        fs::create_dir(&project_root).unwrap();

        let sig = git2::Signature::now("foo", "bar").unwrap();
        let mut idx = repo.index().unwrap();
        idx.add_path(path::Path::new("cruftfile")).unwrap();
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

        assert_ne!(
            util::get_repo_description(&project_root).unwrap().unwrap(),
            "".to_owned()
        );

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

        assert_eq!(
            util::get_repo_description(&project_root),
            Ok(Some("foobar".to_owned()))
        );
    }

    #[test]
    fn parse_deps() {
        let lock_toml_buf = r#"
            [root]
            dependencies = [
                "normal_dep 1.2.3 (r+g)",
                "local_dep 4.5.6",
            ]

            [[package]]
            name = "normal_dep"
            version = "1.2.3"
            source = "r+g"
            dependencies = [
                "dep_of_dep 7.8.9 (r+g)",
            ]

            [[package]]
            name = "local_dep"
            version = "4.5.6"

            [[package]]
            name = "dep_of_dep"
            version = "7.8.9"
            source = "r+g""#;
        let deps = super::parse_dependencies(&lock_toml_buf);
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
