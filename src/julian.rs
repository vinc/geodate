pub const J2000: f64 = 2451545.0009;

pub fn unix_to_julian(timestamp: i64) -> f64 {
    (timestamp as f64 / 86400.0) + 2440587.5
}

pub fn julian_to_unix(jd: f64) -> i64 {
    ((jd - 2440587.5) * 86400.0) as i64
}

// Returns the Julian year for a given Julian ephemeris day
pub fn jde_to_julian_year(jde: f64) -> f64 {
    2000.0 + (jde - J2000) / 365.25
}

pub fn unix_to_year(timestamp: i64) -> f64 {
    1970.0 + (timestamp as f64) / 86400.0 / 365.25
}
