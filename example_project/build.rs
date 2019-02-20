extern crate built;

use std::env;
use std::path;

fn main() {
    let mut opts = built::Options::default();
    opts.set_dependencies(true)
        .set_rerun_on_git_change(true)
        .set_git_dirty_suffix(".dirty".to_string());

    let src = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dst = path::Path::new(&env::var("OUT_DIR").unwrap()).join("built.rs");
    built::write_built_file_with_opts(&opts, &src, &dst)
        .expect("Failed to acquire build-time information");
}
