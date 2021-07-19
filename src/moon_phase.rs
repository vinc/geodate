use math::*;
use julian::*;
use delta_time::*;

use core::ops::Rem;
use num_traits::Float;

#[repr(usize)]
#[derive(Clone, Copy)]
enum MoonPhase {
    NewMoon,
    FirstQuarterMoon,
    FullMoon,
    LastQuarterMoon
}

// From "Astronomical Algorithms"
// By Jean Meeus
fn get_time_of(phase: MoonPhase, lunation_number: f64) -> i64 {
    /*
    // TODO: use `lunation_number: i64`
    let k = match phase {
        MoonPhase::NewMoon          => (lunation_number as f64) + 0.00;
        MoonPhase::FirstQuarterMoon => (lunation_number as f64) + 0.25;
        MoonPhase::FullMoon         => (lunation_number as f64) + 0.50;
        MoonPhase::LastQuarterMoon  => (lunation_number as f64) + 0.75;
    };
    */

    let k = lunation_number;
    let t = k / 1236.85;

    let e = 1.0 - 0.002_516 * t - 0.000_007_4 * t.powi(2);

    // Sun's mean anomaly at time JDE
    let s =  2.5534
           + 29.105_356_7  * k
           -  0.000_001_4  * t.powi(2)
           -  0.000_000_11 * t.powi(3);

    // Moon's mean anomaly
    let m = 201.5643
           + 385.816_935_28  * k
           +   0.010_758_2   * t.powi(2)
           +   0.000_012_38  * t.powi(3)
           -   0.000_000_058 * t.powi(4);

    // Moon's argument of latitude
    let f = 160.7108
          + 390.670_502_84  * k
          -   0.001_611_8   * t.powi(2)
          -   0.000_002_27  * t.powi(3)
          +   0.000_000_011 * t.powi(4);

    // Longitude of the ascending node of the lunar orbit
    let o = 124.7746
          - 1.563_755_88 * k
          + 0.002_0672   * t.powi(2)
          + 0.000_002_15 * t.powi(3);

    let e = (e.rem(360.0) + 360.0).rem(360.0);
    let s = (s.rem(360.0) + 360.0).rem(360.0);
    let m = (m.rem(360.0) + 360.0).rem(360.0);
    let f = (f.rem(360.0) + 360.0).rem(360.0);
    let o = (o.rem(360.0) + 360.0).rem(360.0);

    let jde = 2_451_550.097_660
            +        29.530_588_861     * k
            +         0.000_154_370     * t.powi(2)
            -         0.000_000_150     * t.powi(3)
            +         0.000_000_000_730 * t.powi(4);

    // Correction to be added to JDE

    // [New Moon, First Quarter, Full Moon, Last Quarter]
    let num_cors = vec![
        [-0.40720, -0.62801, -0.40614, -0.62801],
        [ 0.17241,  0.17172,  0.17302,  0.17172],
        [ 0.01608, -0.01183,  0.01614, -0.01183],
        [ 0.01039,  0.00862,  0.01043,  0.00862],
        [ 0.00739,  0.00804,  0.00734,  0.00804],
        [-0.00514,  0.00454, -0.00515,  0.00454],
        [ 0.00208,  0.00204,  0.00209,  0.00204],
        [-0.00111, -0.00180, -0.00111, -0.00180],
        [-0.00057, -0.00070, -0.00057, -0.00070],
        [ 0.00056, -0.00040,  0.00056, -0.00040],
        [-0.00042, -0.00034, -0.00042, -0.00034],
        [ 0.00042,  0.00032,  0.00042,  0.00032],
        [ 0.00038,  0.00032,  0.00038,  0.00032],
        [-0.00024, -0.00028, -0.00024, -0.00028],
        [-0.00017,  0.00027, -0.00017,  0.00027],
        [-0.00007, -0.00017, -0.00007, -0.00017],
        [ 0.00004, -0.00005,  0.00004, -0.00005],
        [ 0.00004,  0.00004,  0.00004,  0.00004],
        [ 0.00003, -0.00004,  0.00003, -0.00004],
        [ 0.00003,  0.00004,  0.00003,  0.00004],
        [-0.00003,  0.00003, -0.00003,  0.00003],
        [ 0.00003,  0.00003,  0.00003,  0.00003],
        [-0.00002,  0.00002, -0.00002,  0.00002],
        [-0.00002,  0.00002, -0.00002,  0.00002],
        [ 0.00002, -0.00002,  0.00002, -0.00002]
    ];

    // Multiply each previous terms by E to a given power
    // [new moon, first quarter, full moon, last quarter]
    let pow_cors = vec![
        [0, 0, 0, 0],
        [1, 1, 1, 1],
        [0, 1, 0, 1],
        [0, 0, 0, 0],
        [1, 0, 1, 0],
        [1, 1, 1, 1],
        [2, 2, 2, 2],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [1, 0, 1, 0],
        [0, 1, 0, 1],
        [1, 1, 1, 1],
        [1, 1, 1, 1],
        [1, 2, 1, 2],
        [0, 1, 0, 1],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];

    // Sum the following terms multiplied a number of times
    // given in the next table, and multiply the sinus of the
    // result by the previously obtained number.
    let terms = [s, m, f, o];

    // [new and full moon, first and last quarter]
    let mul_cors = vec![
        [[ 0.0,  1.0,  0.0,  0.0], [ 0.0,  1.0,  0.0,  0.0]],
        [[ 1.0,  0.0,  0.0,  0.0], [ 1.0,  0.0,  0.0,  0.0]],
        [[ 0.0,  2.0,  0.0,  0.0], [ 1.0,  1.0,  0.0,  0.0]],
        [[ 0.0,  0.0,  2.0,  0.0], [ 0.0,  2.0,  0.0,  0.0]],
        [[-1.0,  1.0,  0.0,  0.0], [ 0.0,  0.0,  2.0,  0.0]],
        [[ 1.0,  1.0,  0.0,  0.0], [-1.0,  1.0,  0.0,  0.0]],
        [[ 2.0,  0.0,  0.0,  0.0], [ 2.0,  0.0,  0.0,  0.0]],
        [[ 0.0,  1.0, -2.0,  0.0], [ 0.0,  1.0, -2.0,  0.0]],
        [[ 0.0,  1.0,  2.0,  0.0], [ 0.0,  1.0,  2.0,  0.0]],
        [[ 1.0,  2.0,  0.0,  0.0], [ 0.0,  3.0,  0.0,  0.0]],
        [[ 0.0,  3.0,  0.0,  0.0], [-1.0,  2.0,  0.0,  0.0]],
        [[ 1.0,  0.0,  2.0,  0.0], [ 1.0,  0.0,  2.0,  0.0]],
        [[ 1.0,  0.0, -2.0,  0.0], [ 1.0,  0.0, -2.0,  0.0]],
        [[-1.0,  2.0,  0.0,  0.0], [ 2.0,  1.0,  0.0,  0.0]],
        [[ 0.0,  0.0,  0.0,  1.0], [ 1.0,  2.0,  0.0,  0.0]],
        [[ 2.0,  1.0,  0.0,  0.0], [ 0.0,  0.0,  0.0,  1.0]],
        [[ 0.0,  2.0, -2.0,  0.0], [-1.0,  1.0, -2.0,  0.0]],
        [[ 3.0,  0.0,  0.0,  0.0], [ 0.0,  2.0,  2.0,  0.0]],
        [[ 1.0,  1.0, -2.0,  0.0], [ 1.0,  1.0,  2.0,  0.0]],
        [[ 0.0,  2.0,  2.0,  0.0], [-2.0,  1.0,  0.0,  0.0]],
        [[ 1.0,  1.0,  2.0,  0.0], [ 1.0,  1.0, -2.0,  0.0]],
        [[-1.0,  1.0,  2.0,  0.0], [ 3.0,  0.0,  0.0,  0.0]],
        [[-1.0,  1.0, -2.0,  0.0], [ 0.0,  2.0, -2.0,  0.0]],
        [[ 1.0,  3.0,  0.0,  0.0], [-1.0,  1.0,  2.0,  0.0]],
        [[ 0.0,  4.0,  0.0,  0.0], [ 1.0,  3.0,  0.0,  0.0]]
    ];

    let j = phase as usize;
    let cor = (0..25).fold(0.0, |acc, i| {
        let sin_cor = (0..4).fold(0.0, |sa, si| {
            sa + mul_cors[i][j % 2][si] * terms[si]
        });

        acc + num_cors[i][j] * e.powi(pow_cors[i][j]) * sin_deg(sin_cor)
    });

    // Additional corrections for quarters
    let w = 0.00306
          - 0.00038 * e * cos_deg(s)
          + 0.00026 *     cos_deg(m)
          - 0.00002 *     cos_deg(m - s)
          + 0.00002 *     cos_deg(m + s)
          + 0.00002 *     cos_deg(2.0 * f);

    let cor = match phase {
        MoonPhase::FirstQuarterMoon => cor + w,
        MoonPhase::LastQuarterMoon  => cor - w,
        _                           => cor
    };

    // Additional corrections for all phases
    let add = 0.0
            + 0.000_325 * sin_deg(299.77 +  0.107_408 * k - 0.009_173 * t.powi(2))
            + 0.000_165 * sin_deg(251.88 +  0.016_321 * k)
            + 0.000_164 * sin_deg(251.83 + 26.651_886 * k)
            + 0.000_126 * sin_deg(349.42 + 36.412_478 * k)
            + 0.000_110 * sin_deg( 84.66 + 18.206_239 * k)
            + 0.000_062 * sin_deg(141.74 + 53.303_771 * k)
            + 0.000_060 * sin_deg(207.14 +  2.453_732 * k)
            + 0.000_056 * sin_deg(154.84 +  7.306_860 * k)
            + 0.000_047 * sin_deg( 34.52 + 27.261_239 * k)
            + 0.000_042 * sin_deg(207.19 +  0.121_824 * k)
            + 0.000_040 * sin_deg(291.34 +  1.844_379 * k)
            + 0.000_037 * sin_deg(161.72 + 24.198_154 * k)
            + 0.000_035 * sin_deg(239.56 + 25.513_099 * k)
            + 0.000_023 * sin_deg(331.55 +  3.592_518 * k);

    let jde = jde + cor + add;

    terrestrial_to_universal_time(julian_to_unix(jde))
}

pub fn get_new_moon(lunation_number: f64) -> i64 {
    get_time_of(MoonPhase::NewMoon, lunation_number)
}

pub fn get_first_quarter_moon(lunation_number: f64) -> i64 {
    get_time_of(MoonPhase::FirstQuarterMoon, lunation_number)
}

pub fn get_full_moon(lunation_number: f64) -> i64 {
    get_time_of(MoonPhase::FullMoon, lunation_number)
}

pub fn get_last_quarter_moon(lunation_number: f64) -> i64 {
    get_time_of(MoonPhase::LastQuarterMoon, lunation_number)
}

/*
// TODO: get_lunation_number(timestamp: i64, numbering: LunationNumbering)
// TODO: get_meeus_lunation_number(timestamp: i64)
enum LunationNumbering {
    Islamic,
    Thai,
    Brown,
    Meeus
}
*/

/// Computes the Lunation Number since the first new moon of 2000
pub fn get_lunation_number(timestamp: i64) -> f64 {
    ((unix_to_year(timestamp) - 2000.0) * 12.3685).floor() // TODO: `as i64`
}

pub fn get_next_new_moon(timestamp: i64) -> i64 {
    let new_moon = get_new_moon(get_lunation_number(timestamp));
    if new_moon > timestamp {
        new_moon
    } else {
        get_new_moon(get_lunation_number(timestamp) + 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::*;

    #[test]
    fn get_lunation_number_test() {
        // Example 49.a from "Astronomical Algoritms"
        // New Moon: 1977-02-18 03:37:42 TD
        let t = terrestrial_to_universal_time(parse_time("1977-02-18T03:37:42+00:00"));
        assert_eq!(-283.0, get_lunation_number(t));

        // Later in the day
        let t = parse_time("1977-02-18T12:00:00+00:00");
        assert_eq!(-283.0, get_lunation_number(t));

        // Later in the month
        let t = parse_time("1977-02-30T12:00:00+00:00");
        assert_eq!(-283.0, get_lunation_number(t));

        // Earlier in the day
        let t = parse_time("1977-02-18T01:00:00+00:00");
        assert_eq!(-283.0, get_lunation_number(t));

        // A few days before
        let t = parse_time("1977-02-14T12:00:00+00:00");
        assert_eq!(-283.0, get_lunation_number(t)); // FIXME: should be -284

        // A week before
        let t = parse_time("1977-02-11T12:00:00+00:00");
        assert_eq!(-284.0, get_lunation_number(t));

        // Meeus Lunation 0
        let t = parse_time("2000-01-06T18:14:00+00:00");
        assert_eq!(0.0, get_lunation_number(t));

        // Brown Lunation 1
        let t = parse_time("1923-01-17T02:41:00+00:00");
        assert_eq!(-952.0, get_lunation_number(t));

        // Islamic Lunation 1
        let t = parse_time("0622-07-16T00:00:00+00:00");
        assert_eq!(-17037.0, get_lunation_number(t));

        // Thai Lunation 0
        let t = parse_time("0638-03-22T00:00:00+00:00");
        assert_eq!(-16843.0, get_lunation_number(t)); // FIXME: should be -16842
    }

    #[test]
    fn get_new_moon_test() {
        // Example 49.a from "Astronomical Algoritms"
        // New Moon: 1977-02-18 03:37:42 TD
        let lunation_number = -283.0;
        let t = terrestrial_to_universal_time(parse_time("1977-02-18T03:37:42+00:00"));
        assert_eq!(t, get_new_moon(lunation_number));

        // First new moon of 1970
        let t = parse_time("1970-01-07T20:35:27+0000");
        assert_eq!(t, get_new_moon(get_lunation_number(0) + 1.0));
    }

    #[test]
    fn get_last_quarter_moon_test() {
        // Example 49.b from "Astronomical Algoritms"
        // Last Quarter Moon: 2044-01-21 23:48:17 TD
        let lunation_number = 544.75;
        let t = terrestrial_to_universal_time(parse_time("2044-01-21T23:48:17+00:00"));
        assert_eq!(t, get_last_quarter_moon(lunation_number));
    }
}
