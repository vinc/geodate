use std;

const PI: f64 = std::f64::consts::PI;

pub fn cos_deg(num: f64) -> f64 {
    (num * PI / 180.0).cos()
}

pub fn sin_deg(num: f64) -> f64 {
    (num * PI / 180.0).sin()
}
