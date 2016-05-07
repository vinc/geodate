pub const J2000: f64 = 2451_545.0; // TODO: Add 0.0009 to this value?

pub fn unix_to_julian(timestamp: i64) -> f64 {
    (timestamp as f64 / 86400.0) + 2440587.5
}

pub fn julian_to_unix(jd: f64) -> i64 {
    ((jd - 2440587.5) * 86400.0).round() as i64
}

// Returns the Julian year for a given Julian ephemeris day
pub fn jde_to_julian_year(jde: f64) -> f64 {
    2000.0 + (jde - J2000) / 365.25
}

pub fn jde_to_julian_century(jde: f64) -> f64 {
    (jde - J2000) / 36525.0
}

pub fn jde_to_julian_millenia(jde: f64) -> f64 {
    (jde - J2000) / 365250.0
}

pub fn unix_to_year(timestamp: i64) -> f64 {
    1970.0 + (timestamp as f64) / 86400.0 / 365.25
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jde_to_julian_millenia_test() {
        assert_approx_eq!(-0.007_218_343_600, jde_to_julian_millenia(2448908.5), 0.000000000001);
    }
}
