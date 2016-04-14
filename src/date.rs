use julian::*;
use math::*;
use moon::*;
use seasons::*;

//use time;

/*
fn print_debug_time(timestamp: i64) {
    println!(
        "DEBUG: {} ==> {}",
        timestamp,
        time::at(time::Timespec::new(timestamp, 0)).strftime("%c").unwrap()
    );
}
*/

fn get_midnight(timestamp: i64, longitude: f64) -> i64 {
    julian_to_unix(julian_transit(timestamp, longitude) - 0.5)
}

fn julian_transit(timestamp: i64, longitude: f64) -> f64 {
    let jd = unix_to_julian(timestamp);

    // Julian Cycle
    let n = (jd - J2000 + longitude / 360.0 + 0.5).floor();

    // Approximate Solar Noon
    let noon = J2000 + n - longitude / 360.0;

    // Solar Mean Anomaly
    let anomaly = (357.5291 + 0.98560028 * (noon - J2000)) % 360.0;

    // Equation of the Center
    let center = 1.9148 * sin_deg(1.0 * anomaly)
               + 0.0200 * sin_deg(2.0 * anomaly)
               + 0.0003 * sin_deg(3.0 * anomaly);

    // Ecliptic Longitude
    let ecliptic_longitude = (anomaly + center + 102.9372 + 180.0) % 360.0;

    // Solar Transit
    let transit = noon
                + 0.0053 * sin_deg(anomaly)
                - 0.0069 * sin_deg(2.0 * ecliptic_longitude);

    transit
}

/*
pub fn get_lunisolar_date(timestamp: i64, longitude: f64) -> String {
    get_date(timestamp, longitude, false)
}
pub fn get_solar_date(timestamp: i64, longitude: f64) -> String {
    get_date(timestamp, longitude, true)
}
*/

pub fn get_date(timestamp: i64, longitude: f64, use_solar_calendar: bool) -> String {
    let now = timestamp;
    let lon = longitude;

    let mut tom = now + 86400;
    let mut mid = get_midnight(now, lon);

    if mid > now {
      tom = now;
      mid = get_midnight(now - 86400, lon);
    }

    let n = 2 + (now / 86400 / 365) as usize;
    let k = if use_solar_calendar { 4 } else { 1 }; // Nb of events per year
    let mut seasonal_events = (1 * k .. n * k).map(|i| {
        // FIXME: Avoid bugs by picking a date around the middle of the year
        let new_year_timestamp = ((i / k) as f64 * 86400.0 * 365.25) as i64;
        let mid_year_timestamp = new_year_timestamp - 180 * 86400;

        if use_solar_calendar {
            // FIXME: Don't use that low level function
            let event_code = i % k;
            get_sun_ephemeris(event_code, mid_year_timestamp)
        } else {
            get_december_solstice(mid_year_timestamp)
        }
    });
    let mut next_seasonal_event = seasonal_events.next().unwrap();

    let m = n * 13;
    let mut new_moons = (0..m).map(|i| {
        // Lunations since the first new moon of January 2000
        let lunation_number = (i as f64) - 371.0;

        get_new_moon(lunation_number)
    });
    let mut new_moon = new_moons.next().unwrap();

    let mut d = 0;
    let mut m = 0;
    let mut y = 0;
    let mut t = get_midnight(0, lon);

    if t < 0 {
      t += 86400;
    }

    while t < mid - 2000 { // Mean solar day approximation
        d += 1;
        t += 86400;
        if use_solar_calendar {
            if next_seasonal_event < (t + 86400) { // New month
                next_seasonal_event = seasonal_events.next().unwrap();
                d = 0;
                m += 1;
                if m == 4 { // New year
                    m = 0;
                    y += 1;
                }
            }
        } else {
            if new_moon < (t + 86400) { // New month
                new_moon = new_moons.next().unwrap();
                d = 0;
                m += 1;
                if next_seasonal_event < (t + 86400) { // New year
                    next_seasonal_event = seasonal_events.next().unwrap();
                    m = 0;
                    y += 1;
                }
            }
        }
    }

    let e = (10000 * (now - mid)) / (get_midnight(tom, lon) - mid);
    let c = e / 100;
    let b = e % 100;

    format!("{:02}:{:02}:{:02}:{:02}:{:02}", y, m, d, c, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_solar_date_test() {
        // Stonehenge coordinates: 51.178844, -1.826189
        assert_eq!("44:02:00:15:34", get_solar_date(1403322675, -1.826189));
    }

    #[test]
    fn get_lunisolar_date_test() {
        assert_eq!("14:03:03:71:54", get_lunisolar_date(449947500, -2.7653));
    }
}
