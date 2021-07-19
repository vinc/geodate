extern crate getopts;
extern crate geodate;

use getopts::Options;

use std::env;
use std::time::SystemTime;

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
        let date = &matches.free[3];
        println!("{}", geodate::reverse::get_timestamp(&format, date, lon));
        return;
    }

    let now = if matches.free.len() == 4 {
        decode_float(&matches.free[3]) as i64
    } else {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(time) => time.as_secs() as i64,
            Err(_) => 0
        }
    };

    if matches.opt_present("e") {
        let events = geodate::ephemeris::get_ephemeris(now, lon, lat);

        for (&time, name) in &events {
            let date = geodate::get_formatted_date(&format, time, lon);
            println!("{:20} {}", format!("{}:", name), date);
        }
    } else {
        let date = geodate::get_formatted_date(&format, now, lon);
        println!("{}", date);
    }
}
