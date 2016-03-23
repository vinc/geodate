use math::*;
use julian::*;

fn compute_jdme(i: usize, m: f64) -> f64 {
    let jdme_terms = vec![
        (2451900.05952, 365242.74049, -0.06223, -0.00823,  0.00032), // December Solstice
        (2451623.80984, 365242.37404,  0.05169, -0.00411, -0.00057), // March Equinoxe
        (2451716.56767, 365241.62603,  0.00325,  0.00888, -0.00030), // June Solstice
        (2451810.21715, 365242.01767, -0.11575,  0.00337,  0.00078)  // September Equinoxe
    ];

    let (a, b, c, d, e) = jdme_terms[i];

    a + b * m
      + c * m.powi(2)
      + d * m.powi(3)
      + e * m.powi(4)
}

fn compute_periodic_terms(t: f64) -> f64 {
    let terms = vec![
        (485.0, 324.96,   1934.136),
        (203.0, 337.23,  32964.467),
        (199.0, 342.08,     20.186),
        (182.0,  27.85, 445267.112),
        (156.0,  73.14,  45036.886),
        (136.0, 171.52,  22518.443),
        ( 77.0, 222.54,  65928.934),
        ( 74.0, 296.72,   3034.906),
        ( 70.0, 243.58,   9037.513),
        ( 58.0, 119.81,  33718.147),
        ( 52.0, 297.17,    150.678),
        ( 50.0,  21.02,   2281.226),
        ( 45.0, 247.54,  29929.562),
        ( 44.0, 325.15,  31555.956),
        ( 29.0,  60.93,   4443.417),
        ( 18.0, 155.12,  67555.328),
        ( 17.0, 288.79,   4562.452),
        ( 16.0, 198.04,  62894.029),
        ( 14.0, 199.76,  31436.921),
        ( 12.0,  95.39,  14577.848),
        ( 12.0, 287.11,  31931.756),
        ( 12.0, 320.81,  34777.259),
        (  9.0, 227.73,   1222.114),
        (  8.0,  15.45,  16859.074)
    ];

    terms.iter().fold(0.0, |s, &(a, b, c)| {
        s + a * cos_deg(b + c * t)
    })
}

fn sun_ephemeris(i: usize, timestamp: i64) -> i64 {
    let jd = unix_to_julian(timestamp);

    let y = jde_to_julian_year(jd).floor();

    // Convert AD year to millenia, from 2000 AD
    let m = (y - 2000.0) / 1000.0;

    let jdme = compute_jdme(i, m);

    // Julian century
    let t = (jdme - J2000) / 36525.0;

    let w = 35999.373 * t - 2.47;

    let l = 1.0 + 0.0334 * cos_deg(w) + 0.0007 * cos_deg(2.0 * w);

    let s = compute_periodic_terms(t);

    julian_to_unix(jdme + (0.00001 * s) / l)
}

pub fn get_december_solstice(timestamp: i64) -> i64 {
    sun_ephemeris(0, timestamp)
}

/*
pub fn get_march_equinoxe(timestamp: i64) -> i64 {
    sun_ephemeris(1, timestamp)
}

pub fn get_june_solstice(timestamp: i64) -> i64 {
    sun_ephemeris(2, timestamp)
}

pub fn get_september_equinoxe(timestamp: i64) -> i64 {
    sun_ephemeris(3, timestamp)
}
*/
