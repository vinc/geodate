# Changelog

## Unreleased

- Fix off-by-one error some days at midnight

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
