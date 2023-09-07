```built``` provides a crate with information from the time it was built.

[![Crates.io Version](https://img.shields.io/crates/v/built.svg)](https://crates.io/crates/built)
[![Docs](https://docs.rs/built/badge.svg)](https://docs.rs/built)
[![Clippy, Format & Test](https://github.com/lukaslueg/built/actions/workflows/check.yml/badge.svg)](https://github.com/lukaslueg/built/actions/workflows/check.yml)

`built` is used as a build-time dependency to collect various information
about the build-environment, serialize this information into Rust-code and
provide that to the crate. The information collected by `built` include:

* Various metadata like version, authors, homepage etc. as set by `Cargo.toml`
* The tag or commit id if the crate was being compiled from within a Git repository.
* The values of `CARGO_CFG_*` build script environment variables, like `CARGO_CFG_TARGET_OS` and `CARGO_CFG_TARGET_ARCH`.
* The features the crate was compiled with.
* The various dependencies, dependencies of dependencies and their versions Cargo ultimately chose to compile.
* The presence of a CI-platform like `Github Actions`, `Travis CI` and `AppVeyor`.
* The compiler and it's version; the documentation-generator and it's version.

See [the example](https://github.com/lukaslueg/built/tree/master/example_project) or the [docs](https://docs.rs/built) for more information.

---

```rust,ignore
// In build.rs

fn main() {
    built::write_built_file().expect("Failed to acquire build-time information")
}
```

```rust,ignore
// In lib.rs or main.rs

// Include the generated-file as a seperate module
pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

println!(
    "This is version {}, built for {} by {}.",
    built_info::PKG_VERSION,
    built_info::TARGET,
    built_info::RUSTC_VERSION
);

match built_info::CI_PLATFORM {
    None => print!("It seems I've not been built on a continuous integration platform,"),
    Some(ci) => print!("I've been built on CI-platform {},", ci),
}

if built::util::detect_ci().is_some() {
    println!(" but I'm currently executing on one!");
} else {
    println!(" and I'm currently not executing on one!");
}

//....
```

> This is version 0.1.0, built for x86_64-unknown-linux-gnu by rustc 1.43.1 (8d69840ab 2020-05-04).
>
> I was built from git `0.4.1-10-gca2af4f`, commit ca2af4f11bb8f4f6421c4cccf428bf4862573daf; the working directory was "dirty". The branch was `refs/heads/master`.
>
> I was built for a x86_64-CPU, which is a little-endian architecture. I was compiled to run on linux (a unix-breed) and my runtime should be gnu.
>
> It seems I've not been built on a continuous integration platform, and I'm currently not executing on one!
>
> I was built with profile "debug", features "" on 2020-05-27 20:14:38 +02:00 (1 seconds ago) using autocfg 1.0.0, bitflags 1.2.1, built 0.4.1, cargo-lock 4.0.1, cc 1.0.54, cfg-if 0.1.10, chrono 0.4.11, example_project 0.1.0, git2 0.13.6, idna 0.2.0, jobserver 0.1.21, libc 0.2.71, libgit2-sys 0.12.6+1.0.0, libz-sys 1.0.25, log 0.4.8, matches 0.1.8, num-integer 0.1.42, num-traits 0.2.11, percent-encoding 2.1.0, pkg-config 0.3.17, proc-macro2 1.0.17, quote 1.0.6, semver 0.10.0, semver 0.9.0, semver-parser 0.7.0, serde 1.0.110, serde_derive 1.0.110, smallvec 1.4.0, syn 1.0.25, time 0.1.43, toml 0.5.6, unicode-bidi 0.3.4, unicode-normalization 0.1.12, unicode-xid 0.2.0, url 2.1.1, vcpkg 0.2.8, winapi 0.3.8, winapi-i686-pc-windows-gnu 0.4.0, winapi-x86_64-pc-windows-gnu 0.4.0
