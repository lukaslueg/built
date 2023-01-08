use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::path;
use std::process;

struct Project {
    root: tempfile::TempDir,
    files: Vec<(path::PathBuf, Vec<u8>)>,
}

impl Project {
    fn new() -> Project {
        Project {
            root: tempfile::tempdir().unwrap(),
            files: Vec::new(),
        }
    }

    fn add_file<N: Into<path::PathBuf>, C: Into<Vec<u8>>>(
        &mut self,
        name: N,
        content: C,
    ) -> &mut Self {
        self.files.push((name.into(), content.into()));
        self
    }

    #[cfg(any(target_os = "windows", feature = "git2"))]
    fn bootstrap(&mut self) -> &mut Self {
        let built_root = get_built_root();
        let features = if cfg!(feature = "git2") {
            r#"["git2"]"#
        } else {
            "[]"
        };
        self.add_file(
            "Cargo.toml",
            format!(
                r#"
[package]
name = "testbox"
version = "0.0.1"
build = "build.rs"

[build-dependencies]
built = {{ path = {:?}, features = {} }}"#,
                &built_root, &features,
            ),
        )
        .add_file(
            "build.rs",
            r#"
extern crate built;
fn main() {
    built::write_built_file().expect("writing failed");
}"#,
        );
        self
    }

    /// Hold on to the tempdir, it will be removed when dropped!
    fn create(self) -> io::Result<tempfile::TempDir> {
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

    fn create_and_run(self) {
        let root = self.create().expect("Creating the project failed");
        Self::run(root.as_ref())
    }

    fn run(root: &std::path::Path) {
        let cargo_result = process::Command::new("cargo")
            .current_dir(root)
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

    #[cfg(feature = "git2")]
    fn init_git(&self) -> git2::Repository {
        git2::Repository::init_opts(
            &self.root,
            git2::RepositoryInitOptions::new()
                .external_template(false)
                .mkdir(false)
                .no_reinit(true)
                .mkpath(false),
        )
        .expect("git-init failed")
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

    p.add_file(
        "Cargo.toml",
        format!(
            "
[package]
name = \"testbox\"
version = \"1.2.3-rc1\"
authors = [\"Joe\", \"Bob\", \"Harry:Potter\"]
build = \"build.rs\"
description = \"xobtset\"
homepage = \"localhost\"
repository = \"https://dev.example.com/sources/testbox/\"
license = \"MIT\"

[dependencies]
built = {{ path = {:?}, features=[\"git2\", \"chrono\", \"semver\"] }}

[build-dependencies]
built = {{ path = {:?}, features=[\"git2\", \"chrono\", \"semver\"] }}

[features]
default = [\"SuperAwesome\", \"MegaAwesome\"]
SuperAwesome = []
MegaAwesome = []",
            &built_root, &built_root
        ),
    );

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
    built::write_built_file_with_opts(&options, src.as_ref(), &dst).unwrap();
}"#,
    );

    p.add_file(
        "src/main.rs",
        r#"
//! The testbox.

mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn main() {
    assert_eq!(built_info::GIT_VERSION, None);
    assert_eq!(built_info::GIT_DIRTY, None);
    assert_eq!(built_info::GIT_COMMIT_HASH, None);
    assert_eq!(built_info::GIT_COMMIT_HASH_SHORT, None);
    assert_eq!(built_info::GIT_HEAD_REF, None);
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
    assert_eq!(built_info::PKG_LICENSE, "MIT");
    assert_eq!(built_info::PKG_REPOSITORY, "https://dev.example.com/sources/testbox/");
    assert!(built_info::NUM_JOBS > 0);
    assert!(built_info::OPT_LEVEL == "0");
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
    assert_ne!(built_info::CFG_FAMILY, "");
    assert_ne!(built_info::CFG_OS, "");
    assert_ne!(built_info::CFG_POINTER_WIDTH, "");
    // For CFG_ENV, empty string is a possible value.
    let _: &'static str = built_info::CFG_ENV;

    assert!(built::util::parse_versions(built_info::DEPENDENCIES.iter())
        .any(|(name, ver)| name == "toml" && ver >= built::semver::Version::parse("0.1.0").unwrap()));

    assert!((built::chrono::offset::Utc::now() - built::util::strptime(built_info::BUILT_TIME_UTC)).num_days() <= 1);
}"#,
    );
    p.create_and_run();
}

#[test]
#[cfg(feature = "git2")]
fn clean_then_dirty_git() {
    let mut p = Project::new();
    p.bootstrap().add_file(
        "src/main.rs",
        r#"
mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn main() {
    assert_eq!(built_info::GIT_DIRTY, Some(false));
}
"#,
    );
    let repo = p.init_git();
    let root = p.create().expect("Creating the project failed");

    let sig = git2::Signature::now("foo", "bar").unwrap();

    let mut idx = repo.index().unwrap();
    for p in &["src/main.rs", "build.rs"] {
        idx.add_path(path::Path::new(p)).unwrap();
    }
    idx.write().unwrap();
    repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        "Testing testing 1 2 3",
        &repo.find_tree(idx.write_tree().unwrap()).unwrap(),
        &[],
    )
    .unwrap();
    Project::run(root.as_ref());

    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&root.path().join("src/main.rs"))
        .unwrap();
    f.write_all(
        r#"
mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn main() {
    assert_eq!(built_info::GIT_DIRTY, Some(true));
    assert!(built_info::GIT_COMMIT_HASH.is_some());
    assert!(built_info::GIT_COMMIT_HASH_SHORT.is_some());
    assert!(built_info::GIT_COMMIT_HASH.unwrap().starts_with(built_info::GIT_COMMIT_HASH_SHORT.unwrap()));
}
"#
        .as_bytes(),
    )
    .unwrap();

    Project::run(root.as_ref());
}

#[test]
#[cfg(feature = "git2")]
fn empty_git() {
    // Issue #7, git can be there and still fail
    let mut p = Project::new();
    p.bootstrap().add_file(
        "src/main.rs",
        r#"
mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn main() {}
"#,
    );
    p.init_git();
    p.create_and_run();
}

#[cfg(target_os = "windows")]
#[test]
fn absolute_paths() {
    // Issue #35. Usually binaries we refer to are simply executables names but sometimes they are
    // absolute paths, containing backslashes, and everything gets sad on this devilish platform.

    let mut p = Project::new();
    p.bootstrap().add_file(
        "src/main.rs",
        r#"
mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn main() {}
"#,
    );

    let rustc_exe_buf = String::from_utf8(
        process::Command::new("where")
            .arg("rustc")
            .output()
            .expect("Unable to locate absolute path to rustc using `where`")
            .stdout,
    )
    .unwrap();
    let rustc_exe = rustc_exe_buf.split("\r\n").next().unwrap();

    // There should at least be `C:\`
    assert!(rustc_exe.contains('\\'));

    let root = p.create().expect("Creating the project failed");
    let cargo_result = process::Command::new("cargo")
        .current_dir(&root)
        .arg("run")
        .env("RUSTC", &rustc_exe)
        .output()
        .expect("cargo failed");
    if !cargo_result.status.success() {
        panic!(
            "cargo failed with {}",
            String::from_utf8_lossy(&cargo_result.stderr)
        );
    }
}
