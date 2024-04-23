# Changelog

## Unreleased

- Add JDME for the years -1000 to +1000 (#58)
- Add no_std support (#36)
- Create dependabot.yml
- Fix negative dates (#62)
- Fix off-by-one error some days at midnight
- Migrate from TravisCI to GitHub Actions (#30)
- Update GitHub Actions (#60)
- Update time dependency (#40)
- Bump num-traits from 0.2.14 to 0.2.15 (#42)
- Bump num-traits from 0.2.15 to 0.2.17 (#65)
- Bump time from 0.1.43 to 0.3.9 (#41)
- Bump time from 0.3.9 to 0.3.20 (#53)
- Bump time from 0.3.20 to 0.3.30 (#64)
- Bump time from 0.3.30 to 0.3.31 (#67)

## 0.4.0 (2020-07-30)

- Add function to get the ephemeris of a geodate
- Add function to reverse a geodate into a timestamp
- Remove time dependency (#22)

## 0.3.0 (2019-05-10)

- Add support of geodates older than unix epoch
- Add option to convert geodate strings back into unix timestamps
- Add machine flag to CLI to output unix timestamp
- Add format option to CLI for custom date format
- Change default epoch from unix to gregorian
- Fix geodates around unix epoch
- Fix panic with geodates at the end of the first year after unix epoch

## 0.2.1 (2017-03-20)

- Add changelog
- Add version flag to CLI
- Fix incorrect date output

## 0.2.0 (2017-02-20)

- Add documentation
- Move code from CLI to library

## 0.1.1 (2017-01-18)

- Publish crate

## 0.1.0 (2015-01-19)

- Start project
