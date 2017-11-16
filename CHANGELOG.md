# Changelog

The format of this file is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).


## [Unreleased]

### Added
- Add support of geodates older than unix epoch
- Add option to convert geodate strings back into unix timestamps
- Add machine flag to CLI to output unix timestamp

### Fixed
- Fix geodates around unix epoch
- Fix panic with geodates at the end of the first year after unix epoch


## [0.2.1] - 2017-03-20

### Added
- Add changelog
- Add version flag to CLI

### Fixed
- Fix incorrect date output


## [0.2.0] - 2017-02-20

### Added
- Add documentation

### Changed
- Move code from CLI to library


## [0.1.1] - 2017-01-18

- Publish crate


## [0.1.0] - 2015-01-19

- Start project
