use earth_orbit::*;
use moon_phase::*;
use sun_transit::*;
use utils::*;

/// Constructs a string representing the time in a geodate format
pub fn get_date(timestamp: i64, longitude: f64, use_solar_calendar: bool) -> String {
    let now = timestamp;
    let lon = longitude;

    let epochs = [
        parse_time("1970-01-01T00:00:00+0000"), // Unix epoch
        parse_time("1957-01-01T00:00:00+0000"), // The following dates all
        parse_time("1938-01-01T00:00:00+0000"), // occurs on a new moon day,
        parse_time("1900-01-01T00:00:00+0000"), // but we cannot go further
        parse_time("1699-01-01T00:00:00+0000"), // back than 1620 with the
        parse_time("1623-01-01T00:00:00+0000")  // current delta time formula.
    ];

    let mut first_new_moon = 0;
    let mut epoch = 0;
    for &e in &epochs {
        // Pick the nearest epoch to shorten calculations
        first_new_moon = get_next_new_moon(e);
        epoch = get_midnight(first_new_moon, lon);
        if epoch < now {
            break;
        }
    }
    if now < epoch {
        panic!("too far back in time");
    }

    let mut new_year = get_next_december_solstice(epoch);
    let mut new_month = if use_solar_calendar {
        get_next_march_equinox(epoch)
    } else {
        get_next_new_moon(first_new_moon)
    };

    let mut midnight = get_midnight(now, lon);
    if midnight > now {
        midnight -= 86400;
    } else if midnight < now - 86400 {
        midnight += 86400;
    }

    let mut d = 0;
    let mut m = 0;
    let mut y = 0;
    let mut t = epoch;
    while t < midnight - 2000 { // Mean solar day approximation
        d += 1;
        t += 86400;
        if new_month < t + 86400 {
            new_month = if use_solar_calendar {
                match m {
                    0 => get_next_june_solstice(new_month),
                    1 => get_next_september_equinox(new_month),
                    2 => get_next_december_solstice(new_month),
                    3 => get_next_march_equinox(new_month),
                    _ => unreachable!()
                }
            } else {
                get_next_new_moon(new_month)
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

    let e = (10000 * (now - midnight)) / 86400;
    let c = e / 100;
    let b = e % 100;

    if epoch < epochs[0] {
        y += (epoch as f64 / 86400.0 / 365.25).round() as i64;
        format!("{:03}:{:02}:{:02}:{:02}:{:02}", y, m, d, c, b)
    } else {
        format!("{:02}:{:02}:{:02}:{:02}:{:02}", y, m, d, c, b)
    }
}

/// Constructs a string representing the time in a geodate format with a lunisolar calendar
pub fn get_lunisolar_date(timestamp: i64, longitude: f64) -> String {
    get_date(timestamp, longitude, false)
}

/// Constructs a string representing the time in a geodate format with a solar calendar
pub fn get_solar_date(timestamp: i64, longitude: f64) -> String {
    get_date(timestamp, longitude, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_solar_date_test() {
        // Stonehenge coordinates: 51.178844, -1.826189
        assert_eq!("44:02:00:15:42", get_solar_date(1403322675, -1.826189));
    }

    #[test]
    fn get_lunisolar_date_test() {
        assert_eq!("00:00:00:00:00", get_lunisolar_date(parse_time("1970-01-07T00:06:15+0000"), 0.0));
        assert_eq!("00:11:29:99:99", get_lunisolar_date(parse_time("1970-12-28T00:01:20+0000"), 0.0));
        assert_eq!("01:00:00:00:00", get_lunisolar_date(parse_time("1970-12-28T00:01:30+0000"), 0.0));

        assert_eq!("06:00:00:00:00", get_lunisolar_date(parse_time("1976-01-01T00:03:20+0000"), 0.0));

        assert_eq!("14:03:03:71:59", get_lunisolar_date(449947500, -2.7653));

        assert_eq!("43:11:28:99:98", get_lunisolar_date(parse_time("2014-01-01T00:03:20+0000"), 0.0));
        assert_eq!("43:11:28:99:99", get_lunisolar_date(parse_time("2014-01-01T00:03:30+0000"), 0.0));
        assert_eq!("44:00:00:00:00", get_lunisolar_date(parse_time("2014-01-01T00:03:40+0000"), 0.0));

        assert_eq!("63:00:00:00:00", get_lunisolar_date(parse_time("2033-01-01T00:03:45+0000"), 0.0));
        assert_eq!("101:00:00:00:00", get_lunisolar_date(parse_time("2071-01-01T00:03:30+0000"), 0.0));

        // Check bugs fixed by version 0.2.1
        assert_eq!("46:02:10:49:46", get_lunisolar_date(parse_time("2016-03-19T12:00:00+0000"), 0.0));
        assert_eq!("46:02:11:80:04", get_lunisolar_date(parse_time("2016-03-20T08:00:00+0000"), 170.0));
        assert_eq!("30:04:28:99:99", get_lunisolar_date(parse_time("2000-06-01T17:57:50+0000"), 90.0));
        assert_eq!("30:05:00:00:00", get_lunisolar_date(parse_time("2000-06-01T17:58:00+0000"), 90.0));

        // Check bugs fixed by version 0.3.0
        assert_eq!("00:11:29:99:99", get_lunisolar_date(parse_time("1970-12-28T00:08:40+00:00"), -1.8262));
        assert_eq!("01:00:00:00:00", get_lunisolar_date(parse_time("1970-12-28T00:08:50+00:00"), -1.8262)); // v0.2.1 panics
        assert_eq!("01:00:01:49:35", get_lunisolar_date(parse_time("1970-12-29T12:00:00+00:00"), -1.8262)); // v0.2.1 panics
        assert_eq!("01:00:02:49:32", get_lunisolar_date(parse_time("1970-12-30T12:00:00+00:00"), -1.8262)); // v0.2.1 panics
        assert_eq!("01:00:03:99:28", get_lunisolar_date(parse_time("1970-12-31T23:59:59+00:00"), -1.8262)); // v0.2.1 panics
        assert_eq!("01:00:03:99:29", get_lunisolar_date(parse_time("1971-01-01T00:00:00+00:00"), -1.8262));

        // Negative time
        assert_eq!("-01:11:22:99:75", get_lunisolar_date(0, 0.0)); // Unix Epoch
        assert_eq!("-13:00:00:00:00", get_lunisolar_date(parse_time("1957-01-01T00:03:40+0000"), 0.0));
        assert_eq!("-70:00:00:00:00", get_lunisolar_date(parse_time("1900-01-01T00:03:40+0000"), 0.0));
        assert_eq!("-271:00:00:00:00", get_lunisolar_date(parse_time("1699-01-01T00:04:35+0000"), 0.0));
        assert_eq!("-328:00:00:00:00", get_lunisolar_date(parse_time("1642-01-01T00:04:40+0000"), 0.0));
        assert_eq!("-347:00:00:00:00", get_lunisolar_date(parse_time("1623-01-01T00:04:30+0000"), 0.0));

        // Bug
        assert_eq!("-30:11:28:99:99", get_lunisolar_date(parse_time("1940-12-28T00:01:39+00:00"), 0.0)); // Unix Epoch
        assert_eq!("-29:00:00:00:00", get_lunisolar_date(parse_time("1940-12-28T00:01:40+00:00"), 0.0)); // Unix Epoch
    }
}
