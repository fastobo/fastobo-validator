# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).


## [Unreleased]

[Unreleased]: https://github.com/fastobo/fastobo/compare/v0.2.4...HEAD


## [v0.2.4] - 2020-02-13

[v0.2.4]: https://github.com/fastobo/fastobo/compare/v0.2.3...v0.2.4

### Changed
- Bumped `fastobo` dependency from `v0.7.0` to `v0.8.2`.
- Removed `err-derive` dependency.


## [v0.2.3] - 2019-08-07

[v0.2.3]: https://github.com/fastobo/fastobo/compare/v0.2.2...v0.2.3

### Changed
- Bumped `fastobo` to `v0.7.0`


## [v0.2.2] - 2019-07-15

[v0.2.2]: https://github.com/fastobo/fastobo/compare/v0.2.1...v0.2.2

### Changed
- Bumped `fastobo` to `v0.6.0`

### Fixed
- Allow ISO 8601 datetimes with fractional seconds.


## [v0.2.1] - 2019-07-15

[v0.2.1]: https://github.com/fastobo/fastobo/compare/v0.2.0...v0.2.1

### Changed
- Bumped `fastobo` to `v0.5.0`.
- Used `err-derive` instead of `failure` for error chain management


## [v0.2.0] - 2019-06-13

[v0.2.0]: https://github.com/fastobo/fastobo/compare/v0.1.3...v0.2.0

### Added
- Mandatory validation of clause cardinality.


## [v0.1.3] - 2019-06-04

[v0.1.3]: https://github.com/fastobo/fastobo/compare/v0.1.2...v0.1.3

### Fixed
- Bumped `fastobo` version to prevent a bug when parsing Xref lists


## [v0.1.2] - 2019-05-24

[v0.1.2]: https://github.com/fastobo/fastobo/compare/v0.1.1...v0.1.2

### Fixed
- Bumped `fastobo` version to prevent a panic when parsing OBO
  documents containing instance frames.


## [v0.1.1] - 2019-05-16

[v0.1.1]: https://github.com/fastobo/fastobo/compare/v0.1.0...v0.1.1

### Added
- Continuous deployment of Mac OSX pre-built binaries.

### Changed
- Parsing and validation have been separated into different steps.
- Reported errors are now grouped by frame location if applicable.

### Doc
- Fixed example command in `README.md`.


## [v0.1.0] - 2019-05-11

[v0.1.0]: https://github.com/fastobo/fastobo/compare/239f642...v0.1.0

Initial release.
