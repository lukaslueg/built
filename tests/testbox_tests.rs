#![deny(warnings, bad_style, future_incompatible, unused, missing_docs, unused_comparisons)]
extern crate git2;
extern crate tempdir;

use std::env;
use std::fs;
use std::io;
use std::process;
use std::io::Write;
use std::path;


struct Project {
    root: tempdir::TempDir,
    files: Vec<(path::PathBuf, Vec<u8>)>,
}

impl Project {
    fn new() -> Project {
        Project {
            root: tempdir::TempDir::new("built_integration").unwrap(),
            files: Vec::new(),
        }
    }

    fn add_file<N: Into<path::PathBuf>, C: Into<Vec<u8>>>(&mut self, name: N, content: C) {
        self.files.push((name.into(), content.into()))
    }

    /// Hold on to the tempdir, it will be removed when dropped!
    fn create(self) -> io::Result<tempdir::TempDir> {
        fs::DirBuilder::new()
            .create(&self.root.path().join("src"))
            .unwrap();
        for (name, content) in self.files {
            let fname = self.root.path().join(name);
            let mut file = fs::File::create(&fname)?;
            file.write_all(&content)?;
        }
        Ok(self.root)
    }

    fn init_git(&self) -> git2::Repository {
        git2::Repository::init(&self.root).expect("git-init failed")
    }
}

/// Tries to find built's Cargo.toml, panics if it ends up in /
fn get_built_root() -> path::PathBuf {
    env::current_exe()
        .map(|path| {
            let mut path = path;
            loop {
                if path.join("Cargo.toml").exists() {
                    break;
                }
                path = path::PathBuf::from(path.parent().unwrap());
            }
            path
        })
        .unwrap()
}

#[test]
fn new_testbox() {
    let mut p = Project::new();

    let built_root = get_built_root();

    p.add_file("Cargo.toml",
               format!("
[package]
name = \"testbox\"
version = \"1.2.3-rc1\"
authors = [\"Joe\", \"Bob\", \"Harry:Potter\"]
build = \"build.rs\"
description = \"xobtset\"
homepage = \"localhost\"

[dependencies]
time = \"0.1\"
semver = \"0.9\"
built = {{ path = {:?}, features=[\"serialized_git\", \"serialized_time\", \"serialized_version\"] }}

[build-dependencies]
built = {{ path = {:?}, features=[\"serialized_git\", \"serialized_time\", \"serialized_version\"] }}

[features]
default = [\"SuperAwesome\", \"MegaAwesome\"]
SuperAwesome = []
MegaAwesome = []", &built_root, &built_root));

    p.add_file(
        "build.rs",
        r#"
use std::env;
use std::path;
extern crate built;

fn main() {
    // Teleport to a CI-platform, should get detected
    env::set_var("CONTINUOUS_INTEGRATION", "1");

    let mut options = built::Options::default();
    options.set_dependencies(true);
    let src = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dst = path::Path::new(&env::var("OUT_DIR").unwrap()).join("built.rs");
    built::write_built_file_with_opts(&options, &src, &dst).unwrap();
}"#,
    );

    p.add_file(
        "src/main.rs",
        r#"
//! The testbox.
#![deny(warnings, bad_style, future_incompatible, unused, missing_docs, unused_comparisons)]

extern crate built;
extern crate semver;
extern crate time;

mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn main() {
    assert_eq!(built_info::GIT_VERSION, None);
    assert!(built_info::CI_PLATFORM.is_some());
    assert_eq!(built_info::PKG_VERSION, "1.2.3-rc1");
    assert_eq!(built_info::PKG_VERSION_MAJOR, "1");
    assert_eq!(built_info::PKG_VERSION_MINOR, "2");
    assert_eq!(built_info::PKG_VERSION_PATCH, "3");
    assert_eq!(built_info::PKG_VERSION_PRE, "rc1");
    assert_eq!(built_info::PKG_AUTHORS, "Joe:Bob:Harry:Potter");
    assert_eq!(built_info::PKG_NAME, "testbox");
    assert_eq!(built_info::PKG_DESCRIPTION, "xobtset");
    assert_eq!(built_info::PKG_HOMEPAGE, "localhost");
    assert!(built_info::NUM_JOBS > 0);
    assert!(built_info::OPT_LEVEL == 0);
    assert!(built_info::DEBUG);
    assert_eq!(built_info::PROFILE, "debug");
    assert_eq!(built_info::FEATURES,
               ["DEFAULT", "MEGAAWESOME", "SUPERAWESOME"]);
    assert_eq!(built_info::FEATURES_STR,
               "DEFAULT, MEGAAWESOME, SUPERAWESOME");
    assert_ne!(built_info::RUSTC_VERSION, "");
    assert_ne!(built_info::RUSTDOC_VERSION, "");
    assert_ne!(built_info::DEPENDENCIES_STR, "");
    assert_ne!(built_info::HOST, "");
    assert_ne!(built_info::TARGET, "");
    assert_ne!(built_info::RUSTC, "");
    assert_ne!(built_info::RUSTDOC, "");
    assert_ne!(built_info::CFG_TARGET_ARCH, "");
    assert_ne!(built_info::CFG_ENDIAN, "");
    assert_ne!(built_info::CFG_ENV, "");
    assert_ne!(built_info::CFG_FAMILY, "");
    assert_ne!(built_info::CFG_OS, "");
    assert_ne!(built_info::CFG_POINTER_WIDTH, "");

    let v = built_info::DEPENDENCIES;
    assert!(built::util::parse_versions(&v)
        .any(|(name, ver)| name == "cmake" && ver >= semver::Version::parse("0.1.0").unwrap()));

    assert!((built::util::strptime(built_info::BUILT_TIME_UTC) - time::now()).num_days() <= 1);
}"#,
    );

    let root = p.create().expect("Creating the project failed");
    let cargo_result = process::Command::new("cargo")
        .current_dir(&root)
        .arg("run")
        .arg("-q")
        .output()
        .expect("cargo failed");
    if !cargo_result.status.success() {
        panic!(
            "cargo failed with {}",
            String::from_utf8_lossy(&cargo_result.stderr)
        );
    }
}

#[test]
fn empty_git() {
    // Issue #7, git can be there and still fail
    let mut p = Project::new();
    let built_root = get_built_root();
    p.add_file("Cargo.toml", format!(r#"
[package]
name = "testbox"
version = "0.0.1"
build = "build.rs"

[build-dependencies]
built = {{ path = {:?} }}"#, &built_root));
    p.add_file("build.rs", r#"
extern crate built;
fn main() {
    built::write_built_file().expect("writing failed");
}"#);
    p.add_file("src/main.rs", "fn main() {}");
    p.init_git();
    let root = p.create().expect("Creating the project failed");
    let cargo_result = process::Command::new("cargo")
        .current_dir(&root)
        .arg("run")
        .output()
        .expect("cargo failed");
    if !cargo_result.status.success() {
        panic!(
            "cargo failed with {}",
            String::from_utf8_lossy(&cargo_result.stderr)
        );
    }
}
