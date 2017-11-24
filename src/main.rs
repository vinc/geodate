extern crate time;
extern crate geodate;

use geodate::geodate::*;
use geodate::sun_transit::*;
use geodate::earth_orbit::*;
use geodate::moon_phase::*;
use geodate::moon_transit::*;

use std::collections::BTreeMap;
use std::env;

fn main() {
    let mut print_ephemeris    = false;
    let mut print_version      = false;
    let mut print_help         = false;

    let mut format = String::from("%h:%y:%m:%d:%c:%b");

    let args: Vec<_> = env::args().filter(|arg| {
        match arg.as_ref() {
            "--machine" => { format = String::from("%u"); },
            "--solar"   => { format = format.replace("%m", "%s"); },
            "--unix"    => { format = format.replace("%y", "%u"); },
            "--ephem"   => { print_ephemeris = true; },
            "--version" => { print_version = true; },
            "--help"    => { print_help = true; },
            _           => { }
        }

        !arg.starts_with("--")
    }).collect();

    if print_version {
        println!("geodate {}", String::from("v") + env!("CARGO_PKG_VERSION"));
        return;
    }

    if print_help || args.len() < 3 {
        println!("Usage: geodate [options] <latitude> <longitude> [<timestamp>]");
        println!("");
        println!("Options:");
        println!("   --machine   use machine format");
        println!("   --solar     use solar calendar");
        println!("   --unix      use unix epoch");
        println!("   --ephem     print ephemeris");
        println!("   --version   print version");
        println!("   --help      print help");
        return;
    }

    let lat = args[1].parse::<f64>().unwrap();
    let lon = args[2].parse::<f64>().unwrap();

    // Convert geodate string back into unix timestamp
    if args.len() == 4 && args[3].contains(":") {
        let y = date_year(args[3].clone());
        let n = date_index(args[3].clone());

        // Approximate timestamps of bounds
        let mut min = (y - 2) * 365 * 86400;
        let mut max = (y + 2) * 365 * 86400;
        let epoch = if format.contains("%y") {
            // 1900 - 1970
            min -= 70 * 365 * 86400;
            max -= 70 * 365 * 86400;

            -2208988580 // 1900-01-01T00:03:40+0000
        } else {
            518780 // 1970-01-07T00:06:20+0000
        };
        if min < epoch && epoch < max {
            if args[3].starts_with("-") {
                max = epoch - 9;
            } else {
                min = epoch;
            }
        }

        loop {
            let mid = (min + max) / 2;
            let i = date_index(get_formatted_date(&format, mid, lon));
            if i == n || mid == min || mid == max {
                println!("{}", mid);
                return;
            }
            if i < n {
                min = mid;
            } else {
                max = mid;
            }
        }
    }

    let now = if args.len() == 4 {
        args[3].parse::<i64>().unwrap()
    } else {
        time::get_time().sec
    };

    if print_ephemeris {
        let mut events = BTreeMap::new();

        let day_begin_at = get_midnight(now, lon);
        let day_end_at = get_midnight(day_begin_at + 86400 + 10000, lon);

        events.insert(now, "Current:            ");
        
        let es = vec![
            ("Equinox:            ", get_next_march_equinox(day_begin_at)),
            ("Equinox:            ", get_next_september_equinox(day_begin_at)),
            ("Solstice:           ", get_next_december_solstice(day_begin_at)),
            ("Solstice:           ", get_next_june_solstice(day_begin_at))
        ];
        for (name, e) in es {
            if e < day_end_at {
                events.insert(e, name);
            }
        }

        let n = get_lunation_number(day_begin_at); // FIXME: Potential bug here
        let es = vec![
            ("New Moon:           ", get_new_moon(n)),
            ("First Quarter Moon: ", get_first_quarter_moon(n + 0.25)),
            ("Full Moon:          ", get_full_moon(n + 0.50)),
            ("Last Quarter Moon:  ", get_last_quarter_moon(n + 0.75))
        ];
        for (name, e) in es {
            if day_begin_at < e && e < day_end_at {
                events.insert(e, name);
            }
        }

        let mut moonrise = get_moonrise(now, lon, lat);
        if moonrise < day_begin_at {
            moonrise = get_moonrise(now + 86400, lon, lat);
        }
        if moonrise < day_end_at {
            events.insert(moonrise, "Moonrise:           ");
        }

        let mut moonset = get_moonset(now, lon, lat);
        if moonset < day_begin_at {
            moonset = get_moonset(now + 86400, lon, lat);
        }
        if moonset < day_end_at {
            events.insert(moonset,  "Moonset:            ");
        }

        if let Some(sunrise) = get_sunrise(now, lon, lat) {
            events.insert(sunrise, "Sunrise:            ");
        }

        if let Some(sunset) = get_sunset(now, lon, lat) {
            events.insert(sunset, "Sunset:             ");
        }

        for (&time, name) in &events {
            let date = get_formatted_date(&format, time, lon);
            println!("{} {}", name, date);
        }
    } else {
        let date = get_formatted_date(&format, now, lon);
        println!("{}", date);
    }
}

// Extract year from a geodate string
fn date_year(date: String) -> i64 {
    let parts: Vec<_> = date.split(":").collect();

    let y = match parts.len() {
        6 => format!("{}{}", parts[0], parts[1]),
        5 => format!("{}", parts[0]),
        _ => panic!("wrong date format")
    };

    y.parse::<i64>().unwrap()
}

// Transform a geodate string into an integer for comparison
fn date_index(date: String) -> i64 {
    let year = date_year(date.clone());
    let mut index = date.replace(":", "").parse::<i64>().unwrap();
    if index < 0 { // Special case for negative years
        index = (year + 0) * 100_000_000 - (index % 100_000_000);
    }
    index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_year_test() {
        assert_eq!(date_year(    "00:00:00:00:00".into()),     0);
        assert_eq!(date_year(    "02:00:00:00:00".into()),     2);
        assert_eq!(date_year(    "42:00:00:00:00".into()),    42);

        assert_eq!(date_year(   "-00:00:00:00:00".into()),     0);
        assert_eq!(date_year(   "-02:00:00:00:00".into()),    -2);
        assert_eq!(date_year(   "-42:00:00:00:00".into()),   -42);

        assert_eq!(date_year( "00:00:00:00:00:00".into()),     0);
        assert_eq!(date_year( "00:02:00:00:00:00".into()),     2);
        assert_eq!(date_year( "00:42:00:00:00:00".into()),    42);
        assert_eq!(date_year( "03:37:00:00:00:00".into()),   337);
        assert_eq!(date_year( "13:37:00:00:00:00".into()),  1337);

        assert_eq!(date_year("-00:00:00:00:00:00".into()),     0);
        assert_eq!(date_year("-00:02:00:00:00:00".into()),    -2);
        assert_eq!(date_year("-00:42:00:00:00:00".into()),   -42);
        assert_eq!(date_year("-03:37:00:00:00:00".into()),  -337);
        assert_eq!(date_year("-13:37:00:00:00:00".into()), -1337);
    }

    #[test]
    fn date_index_test() {
        assert_eq!(date_index( "00:00:00:00:00:00".into()),             0);
        assert_eq!(date_index( "00:02:00:00:00:00".into()),     200000000);
        assert_eq!(date_index("-00:02:00:00:00:00".into()),    -200000000);
        assert_eq!(date_index("-00:02:05:00:00:00".into()),    -195000000);
        assert_eq!(date_index("-00:02:10:00:00:00".into()),    -190000000);
        assert_eq!(date_index("-00:01:00:00:00:00".into()),    -100000000);
        assert_eq!(date_index("-00:01:10:00:00:00".into()),     -90000000);
        assert_eq!(date_index("-00:01:11:28:99:99".into()),     -88710001);
    }
}
