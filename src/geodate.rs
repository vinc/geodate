use earth_orbit::*;
use moon_phase::*;
use sun_transit::*;

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

        match (i % k) + 4 - k {
            0 => get_march_equinox(mid_year_timestamp),     // only if use_solar_calendar
            1 => get_june_solstice(mid_year_timestamp),     // only if use_solar_calendar
            2 => get_september_equinox(mid_year_timestamp), // only if use_solar_calendar
            3 => get_december_solstice(mid_year_timestamp),
            _ => unreachable!()
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

#[allow(dead_code)]
pub fn get_lunisolar_date(timestamp: i64, longitude: f64) -> String {
    get_date(timestamp, longitude, false)
}

#[allow(dead_code)]
pub fn get_solar_date(timestamp: i64, longitude: f64) -> String {
    get_date(timestamp, longitude, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_solar_date_test() {
        // Stonehenge coordinates: 51.178844, -1.826189
        assert_eq!("44:02:00:15:42", get_solar_date(1403322675, -1.826189));
    }

    #[test]
    fn get_lunisolar_date_test() {
        assert_eq!("14:03:03:71:61", get_lunisolar_date(449947500, -2.7653));
    }
}
