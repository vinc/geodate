# Changelog

The format of this file is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).


## [Unreleased]

## [0.5.0] - 2024-09-29

### Added
- Add JDME for the years -1000 to +1000 (#58)
- Add no_std support (#36)

### Fixed
- Fix negative dates (#62)
- Fix off-by-one error some days at midnight

### Changed
- Update GitHub Actions (#60)
- Update time dependency (#40)
- Migrate from TravisCI to GitHub Actions (#30)

### Bumped
- Bump num-traits from 0.2.17 to 0.2.18 (#70)
- Bump time from 0.3.30 to 0.3.31 (#67)
- Bump time from 0.3.20 to 0.3.30 (#64)
- Bump num-traits from 0.2.15 to 0.2.17 (#65)
- Bump time from 0.3.9 to 0.3.20 (#53)
- Bump num-traits from 0.2.14 to 0.2.15 (#42)
- Bump time from 0.1.43 to 0.3.9 (#41)

## [0.4.0] - 2020-07-30

### Added
- Add function to get the ephemeris of a geodate
- Add function to reverse a geodate into a timestamp

### Removed
- Remove time dependency (#22)

## [0.3.0] - 2019-05-10

### Added
- Add support of geodates older than unix epoch
- Add option to convert geodate strings back into unix timestamps
- Add machine flag to CLI to output unix timestamp
- Add format option to CLI for custom date format

### Changed
- Change default epoch from unix to gregorian

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
