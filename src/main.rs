#[macro_use]
extern crate lazy_static;
extern crate time;

#[macro_use]
mod utils;

mod delta_time;
mod earth_orbit;
mod geodate;
mod julian;
mod math;
mod moon_phase;
mod sun_transit;

use geodate::*;
use sun_transit::*;

use std::collections::BTreeMap;
use std::env;

fn main() {
    let mut use_solar_calendar = false;
    let mut print_ephemeris = false;

    let args: Vec<_> = env::args().filter(|arg| {
        if arg == "--solar" {
            use_solar_calendar = true
        }
        if arg == "--ephem" {
            print_ephemeris = true
        }

        !arg.starts_with("--")
    }).collect();

    if args.len() < 3 {
        println!("Usage: geodate [--solar] <latitude> <longitude> [<timestamp>]");
        return;
    }

    let now = if args.len() == 4 {
        args[3].parse::<i64>().unwrap()
    } else {
        time::get_time().sec
    };

    let lat = args[1].parse::<f64>().unwrap();
    let lon = args[2].parse::<f64>().unwrap();

    if print_ephemeris {
        let mut events = BTreeMap::new();

        events.insert(now, "Current: ");

        if let Some(sunrise) = get_sunrise(now, lon, lat) {
            events.insert(sunrise, "Sunrise: ");
        }

        if let Some(sunset) = get_sunset(now, lon, lat) {
            events.insert(sunset, "Sunset:  ");
        }

        for (&time, name) in &events {
            println!("{} {}", name, get_date(time, lon, use_solar_calendar));
        }
    } else {
        println!("{}", get_date(now, lon, use_solar_calendar));
    }
}
