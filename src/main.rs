#[macro_use]
extern crate lazy_static;
extern crate time;

mod data;
mod julian;
mod math;
mod seasons;

use data::*;
use julian::*;
use math::*;
use seasons::*;

use std::env;

/*
fn print_debug_time(timestamp: i64) {
    println!(
        "DEBUG: {} ==> {}",
        timestamp, 
        time::at(time::Timespec::new(timestamp, 0)).strftime("%c").unwrap()
    );
}
*/

fn get_midnight(timestamp: i64, longitude: f64) -> i64 {
    julian_to_unix(julian_transit(timestamp, longitude) - 0.5)
}

fn julian_transit(timestamp: i64, longitude: f64) -> f64 {
    let jd = unix_to_julian(timestamp);

    // Julian Cycle
    let n = (jd - J2000 + longitude / 360.0 + 0.5).floor();

    // Approximate Solar Noon
    let noon = J2000 + n - longitude / 360.0;

    // Solar Mean Anomaly
    let anomaly = (357.5291 + 0.98560028 * (noon - J2000)) % 360.0;

    // Equation of the Center
    let center = 1.9148 * sin_deg(1.0 * anomaly)
               + 0.0200 * sin_deg(2.0 * anomaly)
               + 0.0003 * sin_deg(3.0 * anomaly);

    // Ecliptic Longitude
    let ecliptic_longitude = (anomaly + center + 102.9372 + 180.0) % 360.0;

    // Solar Transit
    let transit = noon
                + 0.0053 * sin_deg(anomaly)
                - 0.0069 * sin_deg(2.0 * ecliptic_longitude);

    transit
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: detri <latitude> <longitude> [<timestamp>]");
        return;
    }
    let now = if args.len() == 4 {
        args[3].parse::<i64>().unwrap()
    } else {
        time::get_time().sec
    };
    // let lat = args[1].parse::<f64>().unwrap();
    let lon = args[2].parse::<f64>().unwrap();
    let use_solar_calendar = false;

    let mut tom = now + 86400;
    let mut mid = get_midnight(now, lon);

    if mid > now {
      tom = now;
      mid = get_midnight(now - 86400, lon);
    }

    let n = if use_solar_calendar { 4 } else { 1 }; // Nb of events per year
    let mut seasonal_events = (1 * n .. 50 * n).map(|i| {
        // FIXME: Avoid bugs by picking a date around the middle of the year
        let new_year_timestamp = ((i / n) as f64 * 86400.0 * 365.25) as i64;
        let mid_year_timestamp = new_year_timestamp - 180 * 86400;

        if use_solar_calendar {
            // FIXME: Don't use that low level function
            let event_code = i % 4;
            get_sun_ephemeris(event_code, mid_year_timestamp)
        } else {
            get_december_solstice(mid_year_timestamp)
        }
    });


    let mut new_moons = NEW_MOONS.iter();
    let mut new_moon = new_moons.next().unwrap();

    let mut next_seasonal_event = seasonal_events.next().unwrap();

    let mut d = 0;
    let mut m = 0;
    let mut y = 0;
    let mut t = get_midnight(0, lon);

    if t < 0 {
      t += 86400;
    }

    while t < mid - 2000 { // Mean solar day approximation
        d += 1;
        t += 86400;
        if use_solar_calendar {
            if next_seasonal_event < (t + 86400) { // New month
                next_seasonal_event = seasonal_events.next().unwrap();
                d = 0;
                m += 1;
                if m == 4 { // New year
                    m = 0;
                    y += 1;
                }
            }
        } else {
            if *new_moon < (t + 86400) { // New yonth
                new_moon = new_moons.next().unwrap();
                d = 0;
                m += 1;
                if next_seasonal_event < (t + 86400) { // New year
                    next_seasonal_event = seasonal_events.next().unwrap();
                    m = 0;
                    y += 1;
                }
            }
        }
    }

    let e = (10000 * (now - mid)) / (get_midnight(tom, lon) - mid);
    let c = e / 100;
    let b = e % 100;

    println!("{:02}:{:02}:{:02}:{:02}:{:02}", y, m, d, c, b);
}
