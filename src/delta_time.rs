use julian::*;

pub fn delta_time(year: f64) -> f64 {
    // From "Polynomial Expressions for Delta T"
    // By Fred Espenak, GSFC Planetary Systems Laboratory
    // FIXME: Avoid discontinuity around 2013
    if year >= 2013.0 {
        let t = year - 2015.0;
        return 67.62 + 0.3645 * t + 0.0039755 * t.powi(2)
    }

    // From "Delta T: Polynomial Approximation of Time Period 1620-2013"
    // By M. Khalid, Mariam Sultana, and Faheem Zaidi (2014)
    let terms = vec![
        (3.670, 76.541, -253.532,  695.901, -1256.982,   627.152),
        (3.120, 10.872,  -40.744,  236.890,  -351.537,    36.612),
        (2.495, 13.480,   13.075,    8.635,    -3.307,  -128.294),
        (1.925, 12.584,    1.929,   60.896, -1432.216,  3129.071),
        (1.525,  6.364,   11.004,  407.776, -4168.394,  7561.686),
        (1.220, -5.058,   -1.701,  -46.403,  -866.171,  5917.585),
        (0.880, 13.392,  128.592, -279.165, -1282.050,  4039.490),
        (0.455, 30.782,   34.348,   46.452,  1295.550, -3210.913),
        (0.115, 55.281,   91.248,   87.202, -3092.565,  8255.422)
    ];
    let (k, a0, a1, a2, a3, a4) = match year.round() as i64 {
        1620 ..= 1672 => terms[0],
        1673 ..= 1729 => terms[1],
        1730 ..= 1797 => terms[2],
        1798 ..= 1843 => terms[3],
        1844 ..= 1877 => terms[4],
        1878 ..= 1904 => terms[5],
        1905 ..= 1945 => terms[6],
        1946 ..= 1989 => terms[7],
        1990 ..= 2013 => terms[8],
        _             => panic!("year too far back in time")
    };
    let u = k + (year - 2000.0) / 100.0;

    a0 + a1 * u.powi(1)
       + a2 * u.powi(2)
       + a3 * u.powi(3)
       + a4 * u.powi(4)
}

pub fn terrestrial_to_universal_time(timestamp: i64) -> i64 {
    timestamp - delta_time(unix_to_year(timestamp)).floor() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_approx_eq {
        ($a:expr, $b:expr, $e:expr) => ({
            let (a, b, e) = (&$a, &$b, &$e);
            assert!((*a - *b).abs() <= *e, "{} is not within {} of {}", *a, *e, *b);
        })
    }

    #[test]
    fn delta_time_test() {
        // ftp://maia.usno.navy.mil/ser7/deltat.data
        assert_approx_eq!(44.4841, delta_time(unix_to_year( 126230400)), 0.5); // 1974-01-01
        assert_approx_eq!(54.0856, delta_time(unix_to_year( 449625600)), 0.5); // 1984-04-01
        assert_approx_eq!(57.3073, delta_time(unix_to_year( 652147200)), 0.5); // 1990-09-01
        assert_approx_eq!(63.8285, delta_time(unix_to_year( 946684800)), 0.5); // 2000-01-01
        assert_approx_eq!(66.0699, delta_time(unix_to_year(1262304000)), 0.5); // 2010-01-01
        assert_approx_eq!(68.1024, delta_time(unix_to_year(1451606400)), 0.5); // 2016-01-01
        assert_approx_eq!(68.1024, delta_time(unix_to_year(1451606400)), 0.5); // 2016-01-01

        // ftp://maia.usno.navy.mil/ser7/deltat.preds
        assert_approx_eq!(70.0000, delta_time(unix_to_year(1577836800)), 1.0); // 2020-01-01
        assert_approx_eq!(72.0000, delta_time(unix_to_year(1704067200)), 1.0); // 2024-01-01
    }
}
