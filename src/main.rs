#[macro_use]
extern crate lazy_static;
extern crate time;

mod data;

use data::*;

use std::io::prelude::*;
use std::fs::File;
use std::env;

pub const J2000: f64 = 2451545.0009;
pub const PI: f64 = std::f64::consts::PI;

fn find_midnight(timestamp: i64, longitude: f64) -> i64 {
    julian_to_unix(julian_transit(timestamp, longitude) - 0.5)
}

fn sin_deg(num: f64) -> f64 {
    (num * PI / 180.0).sin()
}

fn unix_to_julian(timestamp: i64) -> f64 {
    (timestamp as f64 / 86400.0) + 2440587.5
}

fn julian_to_unix(date: f64) -> i64 {
    ((date - 2440587.5) * 86400.0) as i64
}

fn julian_transit(timestamp: i64, longitude: f64) -> f64 {
    let date = unix_to_julian(timestamp);

    // Julian Cycle
    let n = (date - J2000 + longitude / 360.0 + 0.5).floor();

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
        println!("Usage: detri <latitude> <longitude>");
        return;
    }
    // let lat = args[1].parse::<f64>().unwrap();
    let lon = args[2].parse::<f64>().unwrap();

    let now = time::get_time().sec;
    let mut tom = now + 86400;
    let mut mid = find_midnight(now, lon);

    if mid > now {
      tom = now;
      mid = find_midnight(now - 86400, lon);
    }

    let mut solstices = SOLSTICES.iter();
    let mut new_moons = NEW_MOONS.iter();
    let mut solstice = solstices.next().unwrap();
    let mut new_moon = new_moons.next().unwrap();

    let mut d = 0;
    let mut m = 0;
    let mut y = 0;
    let mut n = 0;
    let mut t = find_midnight(0, lon);

    if t < 0 {
      t += 86400;
    }

    while t < mid - 2000 { // Mean solar day approximation
        d += 1;
        t += 86400;
        if *solstice < (t + 86400) {
            solstice = solstices.next().unwrap();
            n += 1;
        }
        if *new_moon < (t + 86400) {
            new_moon = new_moons.next().unwrap();
            d = 0;
            m += 1;
            if n == 2 {
                n = 0;
                m = 0;
                y += 1;
            }
        }
    }

    let e = (10000 * (now - mid)) / (find_midnight(tom, lon) - mid);
    let c = e / 100;
    let b = e % 100;

    println!("{:02}:{:02}:{:02}:{:02}:{:02}", y, m, d, c, b);
}
