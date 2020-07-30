#[macro_export]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr, $e:expr) => ({
        let (a, b, e) = (&$a, &$b, &$e);
        assert!((*a - *b).abs() <= *e, "{} is not within {} of {}", *a, *e, *b);
    })
}

#[cfg(test)]
pub fn parse_time(iso: &str) -> i64 {
    time::strptime(iso, "%FT%T%z").unwrap().to_timespec().sec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_time_test() {
        assert_eq!(0, parse_time("1970-01-01T00:00:00+00:00"));
        //assert_eq!(0, parse_time("1970-01-01T01:00:00+01:00"));

        assert_eq!(-2208988800, parse_time("1900-01-01T00:00:00+00:00"));
    }
}
