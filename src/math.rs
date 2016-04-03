use std;

const PI: f64 = std::f64::consts::PI;

pub fn rad(num: f64) -> f64 {
    num * PI / 180.0
}

pub fn cos_deg(num: f64) -> f64 {
    rad(num).cos()
}

pub fn sin_deg(num: f64) -> f64 {
    rad(num).sin()
}
