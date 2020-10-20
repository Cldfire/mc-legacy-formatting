# mc-legacy-formatting changelog

Notable `mc-legacy-formatting` changes, tracked in the [keep a changelog](https://keepachangelog.com/en/1.0.0/) format with the addition of the `Internal` change type.

## [Unreleased]

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

[Unreleased]: https://github.com/Cldfire/mc-legacy-formatting/compare/0.2.0...HEAD
[0.2.0]: https://github.com/Cldfire/mc-legacy-formatting/compare/0.1.2...0.2.0
[0.1.2]: https://github.com/Cldfire/mc-legacy-formatting/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/Cldfire/mc-legacy-formatting/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/Cldfire/mc-legacy-formatting/releases/tag/0.1.0
