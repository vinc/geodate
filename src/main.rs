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
mod moon_transit;
mod sun_transit;

use geodate::*;
use sun_transit::*;
use earth_orbit::*;
use moon_phase::*;
use moon_transit::*;

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
            println!("{} {}", name, get_date(time, lon, use_solar_calendar));
        }
    } else {
        println!("{}", get_date(now, lon, use_solar_calendar));
    }
}
