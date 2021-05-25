# mc-legacy-formatting changelog

Notable `mc-legacy-formatting` changes, tracked in the [keep a changelog](https://keepachangelog.com/en/1.0.0/) format with the addition of the `Internal` change type.

## [Unreleased]

### Internal

* `SpanIter` no longer keeps track of a `finished` flag as it was unnecessary
* Updated dependencies

## [0.3.1] - 2020-11-19

### Changed

* Fixed `Cargo.toml` so that the README will display properly on crates.io
  * This got broken by the move to workspaces in the repo; see https://github.com/rust-lang/cargo/issues/5911

## [0.3.0] - 2020-11-19

### Added

* Server list ping example demonstrating a real-world usecase for the crate
* Intra-doc links for most symbols in the generated documentation

### Changed

* `Span::StrikethroughWhitespace` now contains the `text` string slice it's wrapping instead of the number of whitespace characters
  * I realized while writing code on top of this library that omitting the string slice made it impossible to build a wrapper API over this crate that is also based on string slices

### Internal

* Created `test-helper` utility crate to make creating test cases faster
* Changed CI workflow to avoid running jobs twice for pull requests
* Changed audit workflow to only run on PRs when necessary
* Added job to check MSRV in CI (which is currently 1.48 due to usage of intra-doc links)

## [0.2.0] - 2020-10-20

### Added

* `SpanExt`, an extension trait that makes it easy to construct a `SpanIter` ([#1](https://github.com/Cldfire/mc-legacy-formatting/pull/1))
* `Span::wrap_colored`, a helper method to make it easy to wrap a `Span` in a `PrintSpanColored` ([#1](https://github.com/Cldfire/mc-legacy-formatting/pull/1))

### Changed

* `Span` now implements `Copy`

## [0.1.2] - 2020-10-19

### Fixed

Fixed a bug that was causing a `Span::WhitespaceStrikethrough` to be incorrectly parsed in some cases.

## [0.1.1] - 2020-10-19

### Fixed

Fixed some issues with the `Cargo.toml` metadata after the initial publish.

## [0.1.0] - 2020-10-19

Initial release.

[Unreleased]: https://github.com/Cldfire/mc-legacy-formatting/compare/0.3.1...HEAD
[0.3.1]: https://github.com/Cldfire/mc-legacy-formatting/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/Cldfire/mc-legacy-formatting/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/Cldfire/mc-legacy-formatting/compare/0.1.2...0.2.0
[0.1.2]: https://github.com/Cldfire/mc-legacy-formatting/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/Cldfire/mc-legacy-formatting/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/Cldfire/mc-legacy-formatting/releases/tag/0.1.0
