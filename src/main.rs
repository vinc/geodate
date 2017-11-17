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
        let mut min = (y - 2) * 365 * 86400;
        let mut max = (y + 2) * 365 * 86400;
        loop {
            let mid = (min + max) / 2;
            let i = date_index(get_formatted_date(&format, mid, lon));
            if i == n {
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
    date.split(":").next().unwrap().parse::<i64>().unwrap()
}

// Transform a geodate string into an integer for comparison
fn date_index(date: String) -> i64 {
    let year = date_year(date.clone());
    let mut index = date.replace(":", "").parse::<i64>().unwrap();
    if index < 0 { // Special case for negative years
        index = (year + 1) * 100_000_000 - (index % 100_000_000);
    }
    index
}
