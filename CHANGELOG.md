# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).


## [Unreleased]

[Unreleased]: https://github.com/fastobo/fastobo/compare/v0.2.1...HEAD

### Changed
- Bumped `fastobo` to `v0.6.0`

### Fixed
- Allow ISO 8601 datetimes with fractional seconds.


## [0.2.1] - 2019-07-15

[0.2.1]: https://github.com/fastobo/fastobo/compare/v0.2.0...v0.2.1

### Changed
- Bumped `fastobo` to `v0.5.0`.
- Used `err-derive` instead of `failure` for error chain management


## [0.2.0] - 2019-06-13

[0.2.0]: https://github.com/fastobo/fastobo/compare/v0.1.3...v0.2.0

### Added
- Mandatory validation of clause cardinality.


## [0.1.3] - 2019-06-04

[0.1.3]: https://github.com/fastobo/fastobo/compare/v0.1.2...v0.1.3

### Fixed
- Bumped `fastobo` version to prevent a bug when parsing Xref lists


## [0.1.2] - 2019-05-24

[0.1.2]: https://github.com/fastobo/fastobo/compare/v0.1.1...v0.1.2

### Fixed
- Bumped `fastobo` version to prevent a panic when parsing OBO
  documents containing instance frames.


## [0.1.1] - 2019-05-16

[0.1.1]: https://github.com/fastobo/fastobo/compare/v0.1.0...v0.1.1

### Added
- Continuous deployment of Mac OSX pre-built binaries.

### Changed
- Parsing and validation have been separated into different steps.
- Reported errors are now grouped by frame location if applicable.

### Doc
- Fixed example command in `README.md`.


## [0.1.0] - 2019-05-11

[0.1.0]: https://github.com/fastobo/fastobo/compare/239f642...v0.1.0

Initial release.
