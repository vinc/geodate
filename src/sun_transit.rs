use julian::*;
use math::*;

#[allow(dead_code)]
#[derive(PartialEq)]
enum Event {
    Midnight,
    Sunrise,
    Midday,
    Sunset
}

fn get_time_of(event: Event, timestamp: i64, longitude: f64, latitude: f64, altitude: f64) -> Option<i64> {
    // Julian day
    let jd = (unix_to_julian(timestamp) + longitude / 360.0 + 0.5).floor();

    // Julian century
    let t = jde_to_julian_century(jd);

    // Julian millenia
    let r = jde_to_julian_millenia(jd);

    // Solar mean anomaly
    let m = 357.529_11 + 35_999.050_29 * t + 0.000_1537 * t.powi(2);

    // Equation of the Center
    let c = sin_deg(1.0 * m) * (1.914_602 - 0.004_817 * t - 0.000_014 * t.powi(2))
          + sin_deg(2.0 * m) * (0.019_993 - 0.000_101 * t)
          + sin_deg(3.0 * m) * (0.000_289);

    let (nl, no) = nutation(t);

    // Mean obliquity of the eliptic
    // (ε0)
    let e0 = mean_obliquity_eliptic(t);

    // True obliquity of the eliptic
    // (ε)
    let ep = e0 + no;

    // Geometric mean longitude
    // (L0)
    let l0 = 280.466_4567 + 360_007.698_2779 * r
           + 0.030_320_28 * r.powi(2)
           + r.powi(3) / 49931.0
           - r.powi(4) / 15300.0
           - r.powi(5) / 2000_000.0;

    // True longitude
    let o = l0 + c;
    let o = modulo(o, 360.0);

    // True anomaly
    //let v = m + c;

    // Eccentricity of the Earth orbit
    //let e = 0.016_708_634 - 0.000_042_037 * t - 0.000_000_1267 * t.powi(2);

    // Apparent longitude
    let p = 125.04 - 1934.136 * t;
    let l = o - 0.00569 - 0.00478 * sin_deg(p);


    // Right ascension
    // (α)
    // let a = atan2_deg(cos_deg(ep) * sin_deg(o), cos_deg(o));

    // Apparent right ascension
    // (α)
    // NOTE: To compute the apparent right ascension, the true longitude
    // is replaced by the apparent longitude and a term is added to the
    // true obliquity.
    let ep = ep + 0.00256 * cos_deg(p);
    let a = atan2_deg(cos_deg(ep) * sin_deg(l), cos_deg(l));
    let a = modulo(a, 360.0);

    /*
    // Mean sideral time at Greenwich at 0h UT
    let oo = 100.460_618_37 + 36_000.770_053_608 * t
           + 0.000_387_933 * t.powi(2)
           - t.powi(3) / 38_710_000.0;

    let m0 = (a - longitude - oo) / 360.0;
    let m0 = modulo(m0, 1.0);
    */

    let l0 = modulo(l0, 360.0); // FIXME: Move that above?

    // Equation of time
    let eot = l0 - 0.005_7183 - a + nl * cos_deg(ep);

    let transit = (720.0 - 4.0 * (longitude + eot)) / 1440.0;
    let transit = jd.floor() - 0.5 + transit;

    // NOTE: We can use the following instead
    // let transit = jd.floor() - 0.5 + m0;

    // Ecliptic Longitude
    let ecliptic_longitude = (m + c + 102.9372 + 180.0) % 360.0;

    // Declinaison of the Sun
    let d = asin_deg(sin_deg(ecliptic_longitude) * sin_deg(23.44));

    // Hour Angle
    let alt = -2.076 * altitude.sqrt() / 60.0;
    let w = acos_deg((sin_deg(alt - 0.83) - sin_deg(latitude) * sin_deg(d)) /
                     (cos_deg(latitude) * cos_deg(d)));

    if (event == Event::Sunrise || event == Event::Sunset) && w.is_nan() {
        return None
    }

    let jd_event = match event {
        Event::Midnight => transit - 0.5,
        Event::Sunrise  => transit - w / 360.0,
        Event::Sunset   => transit + w / 360.0,
        Event::Midday   => transit
    };

    Some(julian_to_unix(jd_event))
}

pub fn nutation(julian_century: f64) -> (f64, f64) {
    // TODO: The accuracy of this calculation can be improved

    // (T)
    let t = julian_century;

    /*
    // Mean anomaly of the Sun
    // (M)
    let ms = 357.52_772
           + 35_999.050_340 * t
           - 0.000_1603 * t.powi(2)
           - t.powi(3) / 300_000.0;

    // Mean anomaly of the Moon
    // (M')
    let mm = 134.96_298
           + 477_198.867_398 * t
           + 0.008_6972 * t.powi(2)
           - t.powi(3) / 56_250.0;
    */

    // Longitude of the ascending node of the Moon's mean orbit on the ecliptic
    // (Ω)
    // NOTE: The terms in T² and T³ are dropped because we are using simplified
    //       expressions to find Δψ and Δε.
    let pm = 125.04452
           - 1934.136_261 * t;
           //+ 0.002_0708 * t.powi(2)
           //+ t.powi(3) / 450_000.0;

    //let ms = modulo(ms, 360.0);
    //let mm = modulo(mm, 360.0);
    let pm = modulo(pm, 360.0);

    // Mean longitude of the Sun
    // (L)
    let ls = 280.4665 + 36_000.7698 * t;

    // Mean longitude of the Moon
    // (L')
    let lm = 218.3165 + 481_267.8813 * t;

    // Nutation in longitude
    // Accurate to 0.5"
    // (Δψ)
    let nl = dec_deg(0.0, 0.0, -17.20) * sin_deg(pm)
           + dec_deg(0.0, 0.0,  -1.32) * sin_deg(2.0 * ls)
           + dec_deg(0.0, 0.0,  -0.23) * sin_deg(2.0 * lm)
           + dec_deg(0.0, 0.0,   0.21) * sin_deg(2.0 * pm);

    // Nutation in obliquity
    // Accurate to 0.1"
    // (Δε)
    let no = dec_deg(0.0, 0.0,  9.20) * cos_deg(pm)
           + dec_deg(0.0, 0.0,  0.57) * cos_deg(2.0 * ls)
           + dec_deg(0.0, 0.0,  0.10) * cos_deg(2.0 * lm)
           + dec_deg(0.0, 0.0, -0.09) * cos_deg(2.0 * pm);

    (nl, no)
}

pub fn mean_obliquity_eliptic(julian_century: f64) -> f64 {
    // TODO: The accuracy of this calculation can be improved
    let t = julian_century;

    dec_deg(23.0, 26.0, 21.448)
        - dec_deg(0.0, 0.0, 46.8150) * t
        - dec_deg(0.0, 0.0, 0.00059) * t.powi(2)
        + dec_deg(0.0, 0.0, 0.001_813) * t.powi(3)
}

#[allow(dead_code)]
pub fn get_noon(timestamp: i64, longitude: f64) -> i64 {
    get_midday(timestamp, longitude)
}

#[allow(dead_code)]
pub fn get_midday(timestamp: i64, longitude: f64) -> i64 {
    get_time_of(Event::Midday, timestamp, longitude, 0.0, 0.0).unwrap()
}

pub fn get_midnight(timestamp: i64, longitude: f64) -> i64 {
    get_time_of(Event::Midnight, timestamp, longitude, 0.0, 0.0).unwrap()
}

#[allow(dead_code)]
pub fn get_sunrise(timestamp: i64, longitude: f64, latitude: f64) -> Option<i64> {
    get_time_of(Event::Sunrise, timestamp, longitude, latitude, 0.0)
}

#[allow(dead_code)]
pub fn get_sunset(timestamp: i64, longitude: f64, latitude: f64) -> Option<i64> {
    get_time_of(Event::Sunset, timestamp, longitude, latitude, 0.0)
}

#[cfg(test)]
mod tests {
    extern crate time;

    use super::*;
    use math::*;
    use julian::*;

    // FIXME: Deduplicate this macro
    macro_rules! assert_approx_eq {
        ($a:expr, $b:expr, $e:expr) => ({
            let (a, b, e) = (&$a, &$b, &$e);
            assert!((*a - *b).abs() <= *e, "{} is not within {} of {}", *a, *e, *b);
        })
    }

    fn parse_time(iso: &str) -> i64 {
        time::strptime(iso, "%FT%T%z").unwrap().to_timespec().sec
    }

    #[test]
    fn nutation_test() {
        // Example 22.a from "Astronomical Algoritms"
        // FIXME: Should be JDE instead of JD which differ by ΔT

        let jd = unix_to_julian(parse_time("1987-04-10T00:00:00+00:00"));
        let t = jde_to_julian_century(jd);
        let (nl, no) = nutation(t);

        assert_eq!(2446_895.5, jd);
        assert_approx_eq!(-3.788, 3600.0 * nl, 0.1);
        assert_approx_eq!( 9.443, 3600.0 * no, 0.1);


        // Example 28.a from "Astronomical Algoritms"

        let jd = unix_to_julian(parse_time("1992-10-13T00:00:00+00:00"));
        let t = (jd - J2000) / 36525.0;
        let (nl, _) = nutation(t);

        assert_eq!(2448_908.5, jd);
        assert_approx_eq!(0.004_419, nl, 0.00005);
        assert_approx_eq!(15.908, 3600.0 * nl, 0.2);
    }

    #[test]
    fn mean_obliquity_eliptic_test() {
        // Example 22.a from "Astronomical Algoritms"
        // FIXME: Should be JDE instead of JD which differ by ΔT

        let jd = unix_to_julian(parse_time("1987-04-10T00:00:00+00:00"));
        let t = (jd - J2000) / 36525.0;
        let e0 = mean_obliquity_eliptic(t);

        assert_approx_eq!(dec_deg(23.0, 26.0, 27.407), e0, 0.1);


        // Example 28.a from "Astronomical Algoritms"

        let jd = unix_to_julian(parse_time("1992-10-13T00:00:00+00:00"));
        let t = (jd - J2000) / 36525.0;
        let (_, no) = nutation(t);
        let e0 = mean_obliquity_eliptic(t);
        let ep = e0 + no;

        assert_approx_eq!(23.440_1443, ep, 0.00001);
    }

    #[ignore] // TODO: Check if bug is resolved in `time` crate
    #[test]
    fn parse_time_test() {
        assert_eq!(0, parse_time("1970-01-01T00:00:00+00:00"));
        assert_eq!(0, parse_time("1970-01-01T01:00:00+01:00")); // FIXME: Library bug
    }

    #[test]
    fn get_noon_test() {
        get_noon(parse_time("1992-10-13T00:00:00+00:00"), 0.0);
        get_noon(parse_time("1992-10-13T00:00:00+00:00"), 174.0);

        // http://www.esrl.noaa.gov/gmd/grad/solcalc/
        let times = vec![
            ("2010-06-21T12:01:46+00:00", "2010-06-21T12:00:00+00:00", 45.0, 0.0),
            ("2010-09-23T11:52:25+00:00", "2010-09-23T12:00:00+00:00", 45.0, 0.0),
            ("2010-12-21T11:58:03+00:00", "2010-12-21T12:00:00+00:00", 45.0, 0.0)
        ];

        for (t0, t1, _, lon) in times {
            assert_approx_eq!(parse_time(t0), get_noon(parse_time(t1), lon), 1);
        }
    }

    #[test]
    fn get_sunrise_test() {
        assert_eq!(None, get_sunrise(parse_time("2010-12-21T12:00:00+00:00"), 0.0, 70.0));

        // TODO: Test at latitudes > 70
        // http://www.esrl.noaa.gov/gmd/grad/solcalc/
        let times = vec![
            ("2010-06-21T04:13:15+00:00", "2010-06-21T12:00:00+00:00", 45.0, 0.0),
            ("2010-09-23T05:48:17+00:00", "2010-09-23T12:00:00+00:00", 45.0, 0.0),
            ("2010-12-21T07:35:09+00:00", "2010-12-21T12:00:00+00:00", 45.0, 0.0),

            ("2010-09-23T05:42:18+00:00", "2010-09-23T12:00:00+00:00", 70.0, 0.0)

        ];

        for (t0, t1, lat, lon) in times {
            // TODO: Improve accuracy
            let accuracy = if lat > 60.0 { 100 } else { 20 };

            assert_approx_eq!(parse_time(t0), get_sunrise(parse_time(t1), lon, lat).unwrap(), accuracy);
        }
    }

    #[test]
    fn get_sunset_test() {
        assert_eq!(None, get_sunrise(parse_time("2010-12-21T12:00:00+00:00"), 0.0, 70.0));

        // TODO: Test at latitudes > 70
        // http://www.esrl.noaa.gov/gmd/grad/solcalc/
        let times = vec![
            ("2010-06-21T19:50:16+00:00", "2010-06-21T12:00:00+00:00", 45.0, 0.0),
            ("2010-09-23T17:56:34+00:00", "2010-09-23T12:00:00+00:00", 45.0, 0.0),
            ("2010-12-21T16:20:58+00:00", "2010-12-21T12:00:00+00:00", 45.0, 0.0),

            ("2010-09-23T18:02:51+00:00", "2010-09-23T12:00:00+00:00", 70.0, 0.0)
        ];

        for (t0, t1, lat, lon) in times {
            // TODO: Improve accuracy
            let accuracy = if lat > 60.0 { 100 } else { 20 };

            assert_approx_eq!(parse_time(t0), get_sunset(parse_time(t1), lon, lat).unwrap(), accuracy);
        }
    }
}
