extern crate time;
extern crate getopts;
extern crate geodate;

use getopts::Options;

use geodate::geodate::*;
use geodate::ephemeris::*;

use std::env;

fn encode_float(x: f64) -> String {
    format!("0{}", x)
}

fn decode_float(x: &str) -> f64 {
    x[1..].parse::<f64>().unwrap()
}

fn main() {
    let args: Vec<String> = env::args().map(|arg|
        // Encode float arguments that can be negative to avoid getopts panic
        // from unrecognized options.
        if let Some(x) = arg.parse::<f64>().ok() {
            encode_float(x)
        } else {
            arg
        }
    ).collect();

    let mut opts = Options::new();
    opts.optflag("h", "help",    "print help");
    opts.optflag("v", "version", "print version");
    opts.optflag("e", "ephem",   "print ephemeris");
    opts.optflag("s", "solar",   "use solar calendar");
    opts.optflag("u", "unix",    "use unix epoch");
    opts.optflag("m", "machine", "use machine format");
    opts.optopt("f",  "format",  "use custom format", "<str>");

    let matches = match opts.parse(&args) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") || matches.free.len() < 3 {
        let brief = "Usage: geodate [options] <latitude> <longitude> [<timestamp>]";
        print!("{}", opts.usage(brief));
        return;
    }

    if matches.opt_present("v") {
        println!("geodate {}", String::from("v") + env!("CARGO_PKG_VERSION"));
        return;
    }

    let mut format = String::from("%h:%y:%m:%d:%c:%b");

    if matches.opt_present("m") {
        format = String::from("%x");
    }

    if matches.opt_present("s") {
        format = format.replace("%m", "%s");
    }

    if matches.opt_present("u") {
        format = format.replace("%y", "%u");
    }

    if matches.opt_present("f") {
        format = matches.opt_str("f").unwrap();
    }

    let lat = decode_float(&matches.free[1]);
    let lon = decode_float(&matches.free[2]);

    // Convert geodate string back into unix timestamp
    if matches.free.len() == 4 && matches.free[3].contains(":") {
        println!("{}", reverse_date(format, matches.free[3].clone(), lon));
        return;
    }

    let now = if matches.free.len() == 4 {
        decode_float(&matches.free[3]) as i64
    } else {
        time::get_time().sec
    };

    if matches.opt_present("e") {
        let events = get_ephemeris(now, lon, lat);

        for (&time, name) in &events {
            let date = get_formatted_date(&format, time, lon);
            println!("{} {}", name, date);
        }
    } else {
        let date = get_formatted_date(&format, now, lon);
        println!("{}", date);
    }
}

// Extract year from a geodate string
fn date_year(date: String) -> i64 {
    let parts: Vec<_> = date.split(":").collect();

    let y = match parts.len() {
        6 => format!("{}{}", parts[0], parts[1]),
        5 => format!("{}", parts[0]),
        _ => panic!("wrong date format")
    };

    y.parse::<i64>().unwrap()
}

// Transform a geodate string into an integer for comparison
fn date_index(date: String) -> i64 {
    let year = date_year(date.clone());
    let mut index = date.replace(":", "").parse::<i64>().unwrap();
    if index < 0 { // Special case for negative years
        index = (year + 0) * 100_000_000 - (index % 100_000_000);
    }
    index
}

// Reverse a geodate into a timestamp
fn reverse_date(format: String, date: String, longitude: f64) -> i64 {
    let y = date_year(date.clone());
    let n = date_index(date.clone());

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
        let i = date_index(get_formatted_date(&format, mid, longitude));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_year_test() {
        assert_eq!(date_year(    "00:00:00:00:00".into()),     0);
        assert_eq!(date_year(    "02:00:00:00:00".into()),     2);
        assert_eq!(date_year(    "42:00:00:00:00".into()),    42);

        assert_eq!(date_year(   "-00:00:00:00:00".into()),     0);
        assert_eq!(date_year(   "-02:00:00:00:00".into()),    -2);
        assert_eq!(date_year(   "-42:00:00:00:00".into()),   -42);

        assert_eq!(date_year( "00:00:00:00:00:00".into()),     0);
        assert_eq!(date_year( "00:02:00:00:00:00".into()),     2);
        assert_eq!(date_year( "00:42:00:00:00:00".into()),    42);
        assert_eq!(date_year( "03:37:00:00:00:00".into()),   337);
        assert_eq!(date_year( "13:37:00:00:00:00".into()),  1337);

        assert_eq!(date_year("-00:00:00:00:00:00".into()),     0);
        assert_eq!(date_year("-00:02:00:00:00:00".into()),    -2);
        assert_eq!(date_year("-00:42:00:00:00:00".into()),   -42);
        assert_eq!(date_year("-03:37:00:00:00:00".into()),  -337);
        assert_eq!(date_year("-13:37:00:00:00:00".into()), -1337);
    }

    #[test]
    fn date_index_test() {
        assert_eq!(date_index( "00:00:00:00:00:00".into()),             0);
        assert_eq!(date_index( "00:02:00:00:00:00".into()),     200000000);
        assert_eq!(date_index("-00:02:00:00:00:00".into()),    -200000000);
        assert_eq!(date_index("-00:02:05:00:00:00".into()),    -195000000);
        assert_eq!(date_index("-00:02:10:00:00:00".into()),    -190000000);
        assert_eq!(date_index("-00:01:00:00:00:00".into()),    -100000000);
        assert_eq!(date_index("-00:01:10:00:00:00".into()),     -90000000);
        assert_eq!(date_index("-00:01:11:28:99:99".into()),     -88710001);
    }
}
