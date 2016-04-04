extern crate time;

use std::ops::Rem;
use math::*;
use julian::*;

// From "Astronomical Algorithms"
// By Jean Meeus
pub fn get_new_moon(lunation_number: i64) -> i64 {
    //let y = 1970.0 + (timestamp as f64) / 86400.0 / 365.25;
    //let k = ((y - 2000.0) * 12.3685).floor();
    let k = lunation_number as f64;
    let t = k / 1236.85;

    // k + 0.25 => first quarter
    // k + 0.50 => full moon
    // k + 0.75 => last quarter

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

    let cor = 0.0
            - 0.40720         * sin_deg(m)
            + 0.17241 * e     * sin_deg(s)
            + 0.01608         * sin_deg(2.0 * m)
            + 0.01039         * sin_deg(2.0 * f)
            + 0.00739 * e     * sin_deg(m - s)
            - 0.00514 * e     * sin_deg(m + s)
            + 0.00208 * e * e * sin_deg(2.0 * s)
            - 0.00111         * sin_deg(m - 2.0 * f)
            - 0.00057         * sin_deg(m + 2.0 * f)
            + 0.00056 * e     * sin_deg(2.0 * m + s)
            - 0.00042         * sin_deg(3.0 * m)
            + 0.00042 * e     * sin_deg(s + 2.0 * f)
            + 0.00038 * e     * sin_deg(s - 2.0 * f)
            - 0.00024 * e     * sin_deg(2.0 * m - s)
            - 0.00017         * sin_deg(o)
            - 0.00007         * sin_deg(m + 2.0 * s)
            + 0.00004         * sin_deg(2.0 * m - 2.0 * f)
            + 0.00004         * sin_deg(3.0 * s)
            + 0.00003         * sin_deg(m + s - 2.0 * f)
            + 0.00003         * sin_deg(2.0 * m + 2.0 * f)
            - 0.00003         * sin_deg(m + s + 2.0 * f)
            + 0.00003         * sin_deg(m - s + 2.0 * f)
            - 0.00002         * sin_deg(m - s - 2.0 * f)
            - 0.00002         * sin_deg(3.0 * m + s)
            + 0.00002         * sin_deg(4.0 * m);

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

    //println!("DEBUG: k   = {:>13.5}", k);
    //println!("DEBUG: t   = {:>13.5}", t);
    //println!("DEBUG: e   = {:>13.5}", e);
    //println!("DEBUG: s  = {:>13.5}", s);
    //println!("DEBUG: m  = {:>13.5}", m);
    //println!("DEBUG: f   = {:>13.5}", f);
    //println!("DEBUG: o   = {:>13.5}", o);
    //println!("DEBUG: jde = {:>13.5}", jde);
    //println!("DEBUG: cor = {:>13.5}", cor);
    //println!("DEBUG: add = {:>13.5}", add);

    let jde = jde + cor + add;

    //println!("DEBUG: jde = {:>13.5}", jde);
  
    //println!("DEBUG: k={}, jde={}", k, jde);
    
    let tt = julian_to_unix(jde);
    let dt = delta_time(unix_to_year(tt)).floor() as i64;
    //println!("DEBUG: tt  = {:>13.5}", tt);
    //println!("DEBUG: yy  = {:>13.5}", unix_to_year(tt));
    //println!("DEBUG: dt  = {:>13.5}", dt);

    tt - dt
}
