use earth_orbit::*;
use moon_phase::*;
use sun_transit::*;

use alloc::string::String;
#[cfg(not(feature = "std"))]
use num_traits::Float;

#[derive(Clone, Copy)]
pub enum Epoch {
    Gregorian,
    Unix
}

#[derive(Clone, Copy)]
pub enum Calendar {
    Lunisolar,
    Solar
}

static ZEROS: [i64; 6] = [
               0, // 1970-01-01 | Unix epoch

      -410227200, // 1957-01-01 | The following dates all
     -1009843200, // 1938-01-01 | occurs on a new moon day,
     -2208988800, // 1900-01-01 | but we cannot go further
     -8551872000, // 1699-01-01 | back than 1620 with the
    -10950249600  // 1623-01-01 | current delta time formula.
];

/// Get a string representation of a geodate
///
/// Format:
/// - %h   Century (hectoyear)
/// - %y   Solar year starting in 1900, Gregorian friendly epoch
/// - %m   Lunar month (for a lunisolar calendar)
/// - %d   Solar day
/// - %c   Centiday
/// - %b   Dimiday
///
/// %u Solar year starting in 1970, Unix friendly epoch
/// %s Seasonal month (for a solar calendar)
///
/// %x Unix timestamp
pub fn get_formatted_date(format: &str, timestamp: i64, longitude: f64) -> String {
    let mut res = String::from(format);
    let now = timestamp;
    let lon = longitude;

    if format.contains("%x") {
        res = res.replace("%x", &format!("{}", now));

        if !format.contains("%") {
            return res;
        }
    }

    let epoch = if format.contains("%u") {
        Epoch::Unix
    } else {
        Epoch::Gregorian
    };

    let calendar = if format.contains("%s") {
        Calendar::Solar
    } else {
        Calendar::Lunisolar
    };

    let mut first_new_moon = 0;
    let mut zero = 0;
    for &e in &ZEROS {
        // Pick the nearest zero to shorten calculations
        first_new_moon = get_next_new_moon(e);
        zero = get_midnight(first_new_moon, lon);
        if zero < now {
            break;
        }
    }
    if now < zero {
        panic!("too far back in time");
    }

    let mut new_year = get_next_december_solstice(zero);
    let mut new_month = match calendar {
        Calendar::Solar     => get_next_march_equinox(zero),
        Calendar::Lunisolar => get_next_new_moon(first_new_moon)
    };

    let mut midnight = get_midnight(now, lon);
    if midnight > now {
        midnight -= 86400;
    } else if midnight <= now - 86400 {
        midnight += 86400;
    }

    let mut d = 0;
    let mut m = 0;
    let mut y = 0;
    let mut t = zero;
    while t < midnight - 2000 { // Mean solar day approximation
        d += 1;
        t += 86400;
        if new_month < t + 86400 {
            new_month = match calendar {
                Calendar::Solar => {
                    match m {
                        0 => get_next_june_solstice(new_month),
                        1 => get_next_september_equinox(new_month),
                        2 => get_next_december_solstice(new_month),
                        3 => get_next_march_equinox(new_month),
                        _ => unreachable!()
                    }
                },
                Calendar::Lunisolar => {
                    get_next_new_moon(new_month)
                }
            };
            d = 0;
            m += 1;
            if new_year < t + 86400 {
                new_year = get_next_december_solstice(new_year);
                m = 0;
                y += 1;
            }
        }
    }

    let epoch_zero = match epoch {
        Epoch::Unix      => ZEROS[0],
        Epoch::Gregorian => ZEROS[3]
    };

    y += ((zero - epoch_zero) as f64 / 86400.0 / 365.25).round() as i64;
    if zero < epoch_zero {
        y = y.abs();
        res.insert(0, '-');
    }

    if res.contains("%h") {
        let h = y / 100;
        res = res.replace("%h", &format!("{:02}", h));
    }
    y = y % 100;

    res = res.replace("%u", &format!("{:02}", y));
    res = res.replace("%y", &format!("{:02}", y));

    res = res.replace("%m", &format!("{:02}", m));
    res = res.replace("%s", &format!("{:02}", m));
    res = res.replace("%d", &format!("{:02}", d));

    let e = (10000 * (now - midnight)) / 86400;
    let c = e / 100;
    let b = e % 100;
    res = res.replace("%c", &format!("{:02}", c));
    res = res.replace("%b", &format!("{:02}", b));

    res
}

/// Get date with the default formatting
pub fn get_date(timestamp: i64, longitude: f64) -> String {
    get_formatted_date("%h:%y:%m:%d:%c:%b", timestamp, longitude)
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::*;

    #[test]
    fn get_date_test() {
        assert_eq!("01:14:05:24:15:42", get_date(1403322675, -1.826189));
    }

    #[test]
    fn get_solar_date_test() {
        let format = "%u:%s:%d:%c:%b";
        // Stonehenge coordinates: 51.178844, -1.826189
        assert_eq!("44:02:00:15:42", get_formatted_date(format, 1403322675, -1.826189));
    }

    #[test]
    fn get_lunisolar_date_test() {
        let format = "%u:%m:%d:%c:%b";
        assert_eq!("00:00:00:00:00", get_formatted_date(format, parse_time("1970-01-07T00:06:15+00:00"), 0.0));
        assert_eq!("00:11:29:99:99", get_formatted_date(format, parse_time("1970-12-28T00:01:20+00:00"), 0.0));
        assert_eq!("01:00:00:00:00", get_formatted_date(format, parse_time("1970-12-28T00:01:30+00:00"), 0.0));
        assert_eq!("06:00:00:00:00", get_formatted_date(format, parse_time("1976-01-01T00:03:20+00:00"), 0.0));
        assert_eq!("14:03:03:71:59", get_formatted_date(format, 449947500, -2.7653));
        assert_eq!("43:11:28:99:98", get_formatted_date(format, parse_time("2014-01-01T00:03:20+00:00"), 0.0));
        assert_eq!("43:11:28:99:99", get_formatted_date(format, parse_time("2014-01-01T00:03:30+00:00"), 0.0));
        assert_eq!("44:00:00:00:00", get_formatted_date(format, parse_time("2014-01-01T00:03:40+00:00"), 0.0));

        let format = "%h:%u:%m:%d:%c:%b";
        assert_eq!("00:63:00:00:00:00", get_formatted_date(format, parse_time("2033-01-01T00:03:45+00:00"), 0.0));
        assert_eq!("01:01:00:00:00:00", get_formatted_date(format, parse_time("2071-01-01T00:03:30+00:00"), 0.0));
        assert_eq!("01:50:00:00:00:00", get_formatted_date(format, parse_time("2120-01-01T00:03:00+00:00"), 0.0));
        assert_eq!("02:15:00:00:00:00", get_formatted_date(format, parse_time("2185-01-01T00:03:30+00:00"), 0.0));
        assert_eq!("03:40:00:00:00:00", get_formatted_date(format, parse_time("2310-01-01T00:02:30+00:00"), 0.0));
        assert_eq!("05:30:00:00:00:00", get_formatted_date(format, parse_time("2500-01-01T00:02:30+00:00"), 0.0));

        let format = "%u:%m:%d:%c:%b";
        assert_eq!("63:00:00:00:00", get_formatted_date(format, parse_time("2033-01-01T00:03:45+00:00"), 0.0));
        assert_eq!("01:00:00:00:00", get_formatted_date(format, parse_time("2071-01-01T00:03:30+00:00"), 0.0));
        assert_eq!("50:00:00:00:00", get_formatted_date(format, parse_time("2120-01-01T00:03:00+00:00"), 0.0));
        assert_eq!("15:00:00:00:00", get_formatted_date(format, parse_time("2185-01-01T00:03:30+00:00"), 0.0));
        assert_eq!("40:00:00:00:00", get_formatted_date(format, parse_time("2310-01-01T00:02:30+00:00"), 0.0));
        assert_eq!("30:00:00:00:00", get_formatted_date(format, parse_time("2500-01-01T00:02:30+00:00"), 0.0));

        // Check bugs fixed by version 0.2.1
        assert_eq!("46:02:10:49:46", get_formatted_date(format, parse_time("2016-03-19T12:00:00+00:00"), 0.0));
        assert_eq!("46:02:11:80:04", get_formatted_date(format, parse_time("2016-03-20T08:00:00+00:00"), 170.0));
        assert_eq!("30:04:28:99:99", get_formatted_date(format, parse_time("2000-06-01T17:57:50+00:00"), 90.0));
        assert_eq!("30:05:00:00:00", get_formatted_date(format, parse_time("2000-06-01T17:58:00+00:00"), 90.0));

        // Check bugs fixed by version 0.3.0
        assert_eq!("00:11:29:99:99", get_formatted_date(format, parse_time("1970-12-28T00:08:40+00:00"), -1.8262));
        assert_eq!("01:00:00:00:00", get_formatted_date(format, parse_time("1970-12-28T00:08:50+00:00"), -1.8262)); // v0.2.1 panics
        assert_eq!("01:00:01:49:35", get_formatted_date(format, parse_time("1970-12-29T12:00:00+00:00"), -1.8262)); // v0.2.1 panics
        assert_eq!("01:00:02:49:32", get_formatted_date(format, parse_time("1970-12-30T12:00:00+00:00"), -1.8262)); // v0.2.1 panics
        assert_eq!("01:00:03:99:28", get_formatted_date(format, parse_time("1970-12-31T23:59:59+00:00"), -1.8262)); // v0.2.1 panics
        assert_eq!("01:00:03:99:29", get_formatted_date(format, parse_time("1971-01-01T00:00:00+00:00"), -1.8262));

        // Negative time
        let format = "%h:%u:%m:%d:%c:%b";
        assert_eq!("-00:01:11:22:99:75", get_formatted_date(format, 0, 0.0)); // Unix Epoch
        assert_eq!("-00:13:00:00:00:00", get_formatted_date(format, parse_time("1957-01-01T00:03:40+00:00"), 0.0));
        assert_eq!("-00:70:00:00:00:00", get_formatted_date(format, parse_time("1900-01-01T00:03:40+00:00"), 0.0));
        assert_eq!("-02:71:00:00:00:00", get_formatted_date(format, parse_time("1699-01-01T00:04:35+00:00"), 0.0));
        assert_eq!("-03:28:00:00:00:00", get_formatted_date(format, parse_time("1642-01-01T00:04:40+00:00"), 0.0));
        assert_eq!("-03:47:00:00:00:00", get_formatted_date(format, parse_time("1623-01-01T00:04:30+00:00"), 0.0));

        let format = "%u:%m:%d:%c:%b";
        assert_eq!("-01:11:22:99:75", get_formatted_date(format, 0, 0.0)); // Unix Epoch
        assert_eq!("-13:00:00:00:00", get_formatted_date(format, parse_time("1957-01-01T00:03:40+00:00"), 0.0));
        assert_eq!("-70:00:00:00:00", get_formatted_date(format, parse_time("1900-01-01T00:03:40+00:00"), 0.0));
        assert_eq!("-71:00:00:00:00", get_formatted_date(format, parse_time("1699-01-01T00:04:35+00:00"), 0.0));
        assert_eq!("-28:00:00:00:00", get_formatted_date(format, parse_time("1642-01-01T00:04:40+00:00"), 0.0));
        assert_eq!("-47:00:00:00:00", get_formatted_date(format, parse_time("1623-01-01T00:04:30+00:00"), 0.0));

        // Bug
        assert_eq!("-30:11:28:99:99", get_formatted_date(format, parse_time("1940-12-28T00:01:39+00:00"), 0.0)); // Unix Epoch
        assert_eq!("-29:00:00:00:00", get_formatted_date(format, parse_time("1940-12-28T00:01:40+00:00"), 0.0)); // Unix Epoch

        // Bug with "50:08:28:100:00" at solar midnight
        assert_eq!("50:08:27:99:99", get_formatted_date(format, parse_time("2020-09-15T23:55:01+00:00"), 0.0));
        assert_eq!("50:08:28:00:00", get_formatted_date(format, parse_time("2020-09-15T23:55:02+00:00"), 0.0));
        assert_eq!("50:08:28:00:00", get_formatted_date(format, parse_time("2020-09-15T23:55:03+00:00"), 0.0));
    }
}
