use std::{env, path};

fn main() {
    let dst =
        path::Path::new(&env::var("OUT_DIR").expect("OUT_DIR not set")).join("built_at_build.rs");
    built::write_built_file_with_opts(
        #[cfg(any(feature = "cargo-lock", feature = "git2"))]
        Some(
            env::var("CARGO_MANIFEST_DIR")
                .expect("CARGO_MANIFEST_DIR")
                .as_ref(),
        ),
        &dst,
    )
    .unwrap();

    println!("cargo::rerun-if-env-changed=BUILT_AT_BUILD")
}
