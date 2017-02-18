```built``` provides your crate with information from the time it was built.

`built` is used as a build-time dependency to collect various information
about the build environment, serialize it into Rust-code and compile
it into the final crate. The information collected by `built` include:

 * Various metadata like version, authors, homepage etc. as set by `Cargo.toml`
 * The tag or commit id if the crate was being compiled from within a git repo.
 * The features the crate was compiled with.
 * The various dependencies, dependencies of dependencies and their versions
   cargo ultimately chose to compile.
 * The presence of a CI-platform like `Travis CI` and `AppVeyor`.
 * The used compiler and it's version; the used documentation generator and
   it's version.

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

let deps = built::DEPENDENCIES;
if built::util::parse_versions(&v)
                .any(|(name, ver)| name == "DeleteAllMyFiles"
                                   && ver < semver::Version::parse("1.1.4").unwrap())) {
    warn!("DeleteAllMyFiles < 1.1.4 is known to sometimes not really delete all your files. Beware!");
}
```

[![Build Status](https://travis-ci.org/lukaslueg/built.svg?branch=master)](https://travis-ci.org/lukaslueg/built)
