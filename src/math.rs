use core::f64::consts::PI;
use core::ops::Div;
#[cfg(not(feature = "std"))]
use num_traits::Float;

// FIXME: Use builtin rust to_radians()
pub fn rad(num: f64) -> f64 {
    num * PI / 180.0
}

// FIXME: Use builtin rust to_degrees()
pub fn deg(num: f64) -> f64 {
    num * 180.0 / PI
}

pub fn cos_deg(num: f64) -> f64 {
    rad(num).cos()
}

pub fn sin_deg(num: f64) -> f64 {
    rad(num).sin()
}

pub fn tan_deg(num: f64) -> f64 {
    rad(num).tan()
}

pub fn acos_deg(num: f64) -> f64 {
    deg(num.acos())
}

pub fn asin_deg(num: f64) -> f64 {
    deg(num.asin())
}

pub fn atan2_deg(x: f64, y: f64) -> f64 {
    deg(x.atan2(y))
}

pub fn dec_deg(d: f64, m: f64, s: f64) -> f64 {
    d + m / 60.0 + s / 3600.0
}

pub fn modulo(x: f64, y: f64) -> f64 {
    (y + x % y) % y
}

pub fn interpolate(y1: f64, y2: f64, y3: f64, n: f64) -> f64 {
    // From "Astronomical Algorithms" by Jean Meeus
    // Formula 3.3
    let a = y2 - y1; // First difference
    let b = y3 - y2; // First difference
    let c = b - a;   // Second difference

    y2 + (n.div(2.0)) * (a + b + n * c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cos_deg_test() {
        assert_eq!(-1.0, cos_deg(180.0));
    }

    #[test]
    fn acos_deg_test() {
        assert_eq!(180.0, acos_deg(-1.0));
    }

    #[test]
    fn dec_deg_test() {
        assert_eq!(1.3958333333333333, dec_deg(1.0, 23.0, 45.0));
    }
}
