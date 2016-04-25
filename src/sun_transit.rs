use julian::*;
use math::*;

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

    // Sun Transit
    noon + 0.0053 * sin_deg(anomaly)
         - 0.0069 * sin_deg(2.0 * ecliptic_longitude)
}

#[allow(dead_code)]
pub fn get_noon(timestamp: i64, longitude: f64) -> i64 {
    get_midday(timestamp, longitude)
}

#[allow(dead_code)]
pub fn get_midday(timestamp: i64, longitude: f64) -> i64 {
    julian_to_unix(julian_transit(timestamp, longitude))
}

pub fn get_midnight(timestamp: i64, longitude: f64) -> i64 {
    julian_to_unix(julian_transit(timestamp, longitude) - 0.5)
}
