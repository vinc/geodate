use std;

const PI: f64 = std::f64::consts::PI;

pub fn rad(num: f64) -> f64 {
    num * PI / 180.0
}

pub fn deg(num: f64) -> f64 {
    num * 180.0 / PI
}

pub fn cos_deg(num: f64) -> f64 {
    rad(num).cos()
}

pub fn sin_deg(num: f64) -> f64 {
    rad(num).sin()
}

pub fn acos_deg(num: f64) -> f64 {
    deg(num.acos())
}

pub fn asin_deg(num: f64) -> f64 {
    deg(num.asin())
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
}
