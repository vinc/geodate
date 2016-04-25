#[macro_use]
extern crate lazy_static;
extern crate time;

mod date;
mod julian;
mod math;
mod moon;
mod earth_orbit;

use date::*;

use std::env;

fn main() {
    let mut use_solar_calendar = false;

    let args: Vec<_> = env::args().filter(|arg| {
        if arg == "--solar" {
            use_solar_calendar = true
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

    // let lat = args[1].parse::<f64>().unwrap();
    let lon = args[2].parse::<f64>().unwrap();

    println!("{}", get_date(now, lon, use_solar_calendar));
}
