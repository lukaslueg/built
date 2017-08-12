```built``` provides a crate with information from the time it was built.

Documentation for latest release [here](https://docs.rs/built),
for master [here](https://lukaslueg.github.io/built).

`built` is used as a build-time dependency to collect various information
about the build environment, serialize it into Rust-code and compile
it into the final crate. The information collected by `built` include:

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

```rust
pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

info!("This is version {}{}, built for {} by {}.",
       built_info::PKG_VERSION,
       built_info::GIT_VERSION.map_or_else(|| "".to_owned(),
                                           |v| format!(" (git {})", v)),
       built_info::TARGET,
       built_info::RUSTC_VERSION);
trace!("I was built with profile \"{}\", features \"{}\" on {} using {}",
       built_info::PROFILE,
       built_info::FEATURES_STR,
       built_info::BUILT_TIME_UTC,
       built_info::DEPENDENCIES_STR);
```

```
This is version 0.1.0 (git 62eb1e2), built for x86_64-apple-darwin
by rustc 1.16.0-nightly (bf6d7b665 2017-01-15).

I was built with profile "debug", features "DEFAULT, ERR_PRINTLN"
on Thu, 16 Feb 2017 19:00:08 GMT using android_glue 0.2.1,
ansi_term 0.9.0, bitflags 0.3.3, bitflags 0.4.0, bitflags 0.6.0,
bitflags 0.7.0, block 0.1.6, built 0.1.0, byteorder 0.5.3,
bytes 0.3.0, cfg-if 0.1.0, cgl 0.1.5, cgmath 0.7.0, ...
```

---

```rust
extern crate built;
extern crate time;
extern crate semver;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

if (built_info::PKG_VERSION_PRE != "" || built_info::GIT_VERSION.is_some())
   && (built::util::strptime(built_info::BUILT_TIME_UTC) - time::now()).num_days() > 180 {
    println!("You are running a development version that is really old. Update soon!");
}

if built_info::CI_PLATFORM.is_some() {
    panic!("Muahahaha, there will be no commit for you, Peter Pan!");
}

let deps = built_info::DEPENDENCIES;
if built::util::parse_versions(&deps)
                .any(|(name, ver)| name == "DeleteAllMyFiles"
                                   && ver < semver::Version::parse("1.1.4").unwrap()) {
    warn!("DeleteAllMyFiles < 1.1.4 is known to sometimes not really delete all your files. Beware!");
}
```

[![Docs](https://docs.rs/built/badge.svg)](https://docs.rs/built)
[![Build Status](https://travis-ci.org/lukaslueg/built.svg?branch=master)](https://travis-ci.org/lukaslueg/built)
[![Build status](https://ci.appveyor.com/api/projects/status/6dgxjfaisaee040f?svg=true)](https://ci.appveyor.com/project/lukaslueg/built)
