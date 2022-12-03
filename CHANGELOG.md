# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [unreleased]

## [0.5.2] - 2022-12-03
### Changed
- Removed unused transitive dependency on `time`
- Bump `cargo-lock` to 8.0
- Bump `git2` to 0.15
- Fix unescaped quotes in literals

### Added
- Added GitHub Actions to the list of detected CI platforms

## [0.5.1] - 2021-05-27
### Changed
- Bump `cargo-lock` to 7.0
- Bump `semver` to 1.0

## [0.5.0] - 2021-05-01
### Fixed
- Fix possibly wrong `CFG_` values in cross-compilation scenarios
- Fix handling of backspaces in doc-attributes

### Changed
- Switch deprecated `tempdir` to `tempfile`
- Add `#allow(dead_code)` to generated code
- Bump `cargo-lock` to 6.0
- Bump `semver` to 0.11
- Publicly re-export dependencies

## [0.4.4] - 2020-12-01
### Added
- Added `PKG_LICENSE` and `PKG_REPOSITORY`

## [0.4.3] - 2020-08-19
### Fixed
- Fix handling of unescaped special characters

## [0.4.2] - 2020-05-27
### Added
- Add `GIT_DIRTY`
- Add `GIT_COMMIT_HASH` and `GIT_HEAD_REF`

### Changed
- Bump `semver` to 0.10

[unreleased]: https://github.com/lukaslueg/built/compare/0.5.1...master
[0.5.1]: https://github.com/lukaslueg/built/compare/0.5.0...0.5.1
[0.5.0]: https://github.com/lukaslueg/built/compare/0.4.4...0.5.0
[0.4.4]: https://github.com/lukaslueg/built/compare/0.4.3...0.4.4
[0.4.3]: https://github.com/lukaslueg/built/compare/0.4.2...0.4.3
[0.4.2]: https://github.com/lukaslueg/built/compare/0.4.1...0.4.2
