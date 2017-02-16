#![deny(warnings, bad_style, future_incompatible, unused, missing_docs, unused_comparisons)]

//! A innocent looking crate that knows all about itself

/// Build-time information about `testbox`.
pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
