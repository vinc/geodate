use geodate::*;

use alloc::vec::Vec;

/// Reverse a geodate into a timestamp
pub fn get_timestamp(format: &str, date: &str, longitude: f64) -> i64 {
    let y = date_year(date);
    let n = date_index(date);

    // Approximate timestamps of bounds
    let mut min = (y - 2) * 365 * 86400;
    let mut max = (y + 2) * 365 * 86400;
    let epoch = if format.contains("%y") {
        // 1900 - 1970
        min -= 70 * 365 * 86400;
        max -= 70 * 365 * 86400;

        -2208988580 // 1900-01-01T00:03:40+0000
    } else {
        518780 // 1970-01-07T00:06:20+0000
    };
    if min < epoch && epoch < max {
        if date.starts_with("-") {
            max = epoch - 9;
        } else {
            min = epoch;
        }
    }

    loop {
        let mid = (min + max) / 2;
        let i = date_index(&get_formatted_date(&format, mid, longitude));
        if i == n || mid == min || mid == max {
            return mid;
        }
        if i < n {
            min = mid;
        } else {
            max = mid;
        }
    }
}

// Extract year from a geodate string
fn date_year(date: &str) -> i64 {
    let parts: Vec<_> = date.split(":").collect();

    let y = match parts.len() {
        6 => format!("{}{}", parts[0], parts[1]),
        5 => format!("{}", parts[0]),
        _ => panic!("wrong date format")
    };

    y.parse::<i64>().unwrap()
}

// Transform a geodate string into an integer for comparison
fn date_index(date: &str) -> i64 {
    let year = date_year(date.clone());
    let mut index = date.replace(":", "").parse::<i64>().unwrap();
    if index < 0 { // Special case for negative years
        index = (year + 0) * 100_000_000 - (index % 100_000_000);
    }
    index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_year_test() {
        assert_eq!(date_year(    "00:00:00:00:00"),     0);
        assert_eq!(date_year(    "02:00:00:00:00"),     2);
        assert_eq!(date_year(    "42:00:00:00:00"),    42);

        assert_eq!(date_year(   "-00:00:00:00:00"),     0);
        assert_eq!(date_year(   "-02:00:00:00:00"),    -2);
        assert_eq!(date_year(   "-42:00:00:00:00"),   -42);

        assert_eq!(date_year( "00:00:00:00:00:00"),     0);
        assert_eq!(date_year( "00:02:00:00:00:00"),     2);
        assert_eq!(date_year( "00:42:00:00:00:00"),    42);
        assert_eq!(date_year( "03:37:00:00:00:00"),   337);
        assert_eq!(date_year( "13:37:00:00:00:00"),  1337);

        assert_eq!(date_year("-00:00:00:00:00:00"),     0);
        assert_eq!(date_year("-00:02:00:00:00:00"),    -2);
        assert_eq!(date_year("-00:42:00:00:00:00"),   -42);
        assert_eq!(date_year("-03:37:00:00:00:00"),  -337);
        assert_eq!(date_year("-13:37:00:00:00:00"), -1337);
    }

    #[test]
    fn date_index_test() {
        assert_eq!(date_index( "00:00:00:00:00:00"),             0);
        assert_eq!(date_index( "00:02:00:00:00:00"),     200000000);
        assert_eq!(date_index("-00:02:00:00:00:00"),    -200000000);
        assert_eq!(date_index("-00:02:05:00:00:00"),    -195000000);
        assert_eq!(date_index("-00:02:10:00:00:00"),    -190000000);
        assert_eq!(date_index("-00:01:00:00:00:00"),    -100000000);
        assert_eq!(date_index("-00:01:10:00:00:00"),     -90000000);
        assert_eq!(date_index("-00:01:11:28:99:99"),     -88710001);
    }
}
