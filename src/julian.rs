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

// From "Polynomial Expressions for Delta T"
// By Fred Espenak, GSFC Planetary Systems Laboratory
pub fn delta_time(year: f64) -> f64 {
    let y = match year {
        1961.0...1986.0 => 1975.0,
        1986.0...2005.0 => 2000.0,
        2005.0...2050.0 => 2000.0,
        _               => 0.0 // FIXME
    };

    let t = year - y;

    match year {
        1961.0...1986.0 =>
            45.45 + 1.067 * t
            - t.powi(2) / 260.0
            - t.powi(3) / 718.0,

        1986.0...2005.0 =>
              63.86 
            +  0.3345        * t 
            -  0.060374      * t.powi(2)
            +  0.0017275     * t.powi(3) 
            +  0.000651814   * t.powi(4)
            +  0.00002373599 * t.powi(5),

        2005.0...2050.0 =>
            62.92 + 0.32217 * t + 0.005589 * t.powi(2),

        _ => 0.0
    }
}
