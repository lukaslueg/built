//! Internal crate to provide semver-checks on generated code.
//!
//! This ensures that the code *generated* by `built` does not semver-break
//! downstream crates.
//! All we do here is to assign/use items generated by `built` in a way that
//! was documented on the last semver-compatible version.
//! If `built` breaks this crate, it always breaks downstream crates. If updating
//! this crate is required, and this crate semver-breaks, `built` breaks downstream
//! crates as well. Both cases require a semver-bump of `built`.

mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub const CI_PLATFORM: Option<&str> = built_info::CI_PLATFORM;
pub const PKG_VERSION: &str = built_info::PKG_VERSION;
pub const PKG_VERSION_MAJOR: &str = built_info::PKG_VERSION_MAJOR;
pub const PKG_VERSION_MINOR: &str = built_info::PKG_VERSION_MINOR;
pub const PKG_VERSION_PATCH: &str = built_info::PKG_VERSION_PATCH;
pub const PKG_VERSION_PRE: &str = built_info::PKG_VERSION_PRE;
pub const PKG_AUTHORS: &str = built_info::PKG_AUTHORS;
pub const PKG_NAME: &str = built_info::PKG_NAME;
pub const PKG_DESCRIPTION: &str = built_info::PKG_DESCRIPTION;
pub const PKG_HOMEPAGE: &str = built_info::PKG_HOMEPAGE;
pub const PKG_LICENSE: &str = built_info::PKG_LICENSE;
pub const PKG_REPOSITORY: &str = built_info::PKG_REPOSITORY;
pub const TARGET: &str = built_info::TARGET;
pub const HOST: &str = built_info::HOST;
pub const PROFILE: &str = built_info::PROFILE;
pub const RUSTC: &str = built_info::RUSTC;
pub const RUSTDOC: &str = built_info::RUSTDOC;
pub const RUSTC_VERSION: &str = built_info::RUSTC_VERSION;
pub const RUSTDOC_VERSION: &str = built_info::RUSTDOC_VERSION;
pub const OPT_LEVEL: &str = built_info::OPT_LEVEL;
pub const NUM_JOBS: u32 = built_info::NUM_JOBS;
pub const DEBUG: bool = built_info::DEBUG;
pub const FEATURES: &[&str] = &built_info::FEATURES;
pub const FEATURES_STR: &str = built_info::FEATURES_STR;
pub const FEATURES_LOWERCASE: &[&str] = &built_info::FEATURES_LOWERCASE;
pub const FEATURES_LOWERCASE_STR: &str = built_info::FEATURES_LOWERCASE_STR;
pub const CFG_TARGET_ARCH: &str = built_info::CFG_TARGET_ARCH;
pub const CFG_ENDIAN: &str = built_info::CFG_ENDIAN;
pub const CFG_ENV: &str = built_info::CFG_ENV;
pub const CFG_FAMILY: &str = built_info::CFG_FAMILY;
pub const CFG_OS: &str = built_info::CFG_OS;
pub const CFG_POINTER_WIDTH: &str = built_info::CFG_POINTER_WIDTH;

// cargo-lock
pub const DEPENDENCIES: &[(&str, &str)] = &built_info::DEPENDENCIES;
pub const DEPENDENCIES_STR: &str = built_info::DEPENDENCIES_STR;

// dependency-tree
pub const DIRECT_DEPENDENCIES: &[(&str, &str)] = &built_info::DIRECT_DEPENDENCIES;
pub const DIRECT_DEPENDENCIES_STR: &str = built_info::DIRECT_DEPENDENCIES_STR;
pub const INDIRECT_DEPENDENCIES: &[(&str, &str)] = &built_info::INDIRECT_DEPENDENCIES;
pub const INDIRECT_DEPENDENCIES_STR: &str = built_info::INDIRECT_DEPENDENCIES_STR;

// git2
pub const GIT_VERSION: Option<&str> = built_info::GIT_VERSION;
pub const GIT_DIRTY: Option<bool> = built_info::GIT_DIRTY;
pub const GIT_HEAD_REF: Option<&str> = built_info::GIT_HEAD_REF;
pub const GIT_COMMIT_HASH: Option<&str> = built_info::GIT_COMMIT_HASH;
pub const GIT_COMMIT_HASH_SHORT: Option<&str> = built_info::GIT_COMMIT_HASH_SHORT;

// chrono
pub const BUILT_TIME_UTC: &str = built_info::BUILT_TIME_UTC;
