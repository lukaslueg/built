//! Builds the testbox with built-support.

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
}
