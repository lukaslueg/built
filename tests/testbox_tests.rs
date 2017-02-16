#![deny(warnings, bad_style, future_incompatible, unused, missing_docs, unused_comparisons)]
extern crate testbox;
extern crate time;
extern crate semver;
extern crate built;

use testbox::built_info;

#[test]
fn testbox() {
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
}

#[test]
fn parse_dependencies() {
    let v = built_info::DEPENDENCIES;
    assert!(built::util::parse_versions(&v)
        .any(|(name, ver)| name == "cmake" && ver >= semver::Version::parse("0.1.0").unwrap()));
}

#[test]
fn built_time() {
    assert!((built::util::strptime(built_info::BUILT_TIME_UTC) - time::now()).num_days() <= 1);
}
