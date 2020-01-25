```built``` provides a crate with information from the time it was built.

[![Crates.io Version](https://img.shields.io/crates/v/built.svg)](https://crates.io/crates/built)
[![Docs](https://docs.rs/built/badge.svg)](https://docs.rs/built)
[![Build status](https://ci.appveyor.com/api/projects/status/6dgxjfaisaee040f?svg=true)](https://ci.appveyor.com/project/lukaslueg/built)
[![Build Status](https://travis-ci.org/lukaslueg/built.svg?branch=master)](https://travis-ci.org/lukaslueg/built)

`built` is used as a build-time dependency to collect various information
about the build environment and serialize it into the final crate.
The information collected by `built` include:

* Various metadata like version, authors, homepage etc. as set by `Cargo.toml`
* The tag or commit id if the crate was being compiled from within a git repo.
* The values of various `cfg!`, like `target_os` and `target_arch`.
* The features the crate was compiled with.
* The various dependencies, dependencies of dependencies and their versions
   cargo ultimately chose to compile.
* The presence of a CI-platform like `Travis CI` and `AppVeyor`.
* The used compiler and it's version; the used documentation generator and
   it's version.

---

```rust,ignore
// In build.rs

fn main() {
    built::write_built_file().expect("Failed to acquire build-time information")
}
```

```rust,ignore
// In lib.rs or main.rs

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

println!(
    "This is version {}{}, built for {} by {}.",
    built_info::PKG_VERSION,
    built_info::GIT_VERSION.map_or_else(|| "".to_owned(), |v| format!(" (git {})", v)),
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
```

> This is version 0.1.0 (git 0.1-62-gcfdfb93), built for x86_64-unknown-linux-gnu by rustc 1.40.0 (73528e339 2019-12-16).
> It seems I've not been built on a continuous integration platform, and I'm currently not executing on one!
