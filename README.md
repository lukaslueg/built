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
