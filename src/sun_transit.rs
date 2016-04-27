use julian::*;
use delta_time::*;
use math::*;

#[derive(PartialEq)]
enum Event { Rising, Transit, Setting }

fn event_equation(event: Event, timestamp: i64, longitude: f64, latitude: f64, altitude: f64) -> f64 {
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

    // Sun Transit
    let transit = noon + 0.0053 * sin_deg(anomaly)
                       - 0.0069 * sin_deg(2.0 * ecliptic_longitude);

    // Declinaison of the Sun
    let d = asin_deg(sin_deg(ecliptic_longitude) * sin_deg(23.44));

    // Hour Angle
    let a = -2.076 * altitude.sqrt() / 60.0;
    let w = acos_deg((sin_deg(a - 0.83) - sin_deg(latitude) * sin_deg(d)) /
                     (cos_deg(latitude) * cos_deg(d)));

    match event {
        Event::Rising  => transit - w / 360.0,
        Event::Setting => transit + w / 360.0,
        Event::Transit => transit
    }
}

#[allow(dead_code)]
pub fn get_noon(timestamp: i64, longitude: f64) -> i64 {
    get_midday(timestamp, longitude)
}

#[allow(dead_code)]
pub fn get_midday(timestamp: i64, longitude: f64) -> i64 {
    let jde = event_equation(Event::Transit, timestamp, longitude, 0.0, 0.0);

    terrestrial_to_universal_time(julian_to_unix(jde))
}

pub fn get_midnight(timestamp: i64, longitude: f64) -> i64 {
    let jde = event_equation(Event::Transit, timestamp, longitude, 0.0, 0.0);

    terrestrial_to_universal_time(julian_to_unix(jde - 0.5))
}

pub fn get_sunrise(timestamp: i64, longitude: f64, latitude: f64) -> i64 {
    let jde = event_equation(Event::Rising, timestamp, longitude, latitude, 0.0);

    terrestrial_to_universal_time(julian_to_unix(jde))
}

pub fn get_sunset(timestamp: i64, longitude: f64, latitude: f64) -> i64 {
    let jde = event_equation(Event::Setting, timestamp, longitude, latitude, 0.0);

    terrestrial_to_universal_time(julian_to_unix(jde))
}

#[cfg(test)]
mod tests {
    extern crate time;

    use super::*;

    fn parse_time(iso: &str) -> i64 {
        time::strptime(iso, "%FT%T%z").unwrap().to_timespec().sec
    }

    #[test]
    fn parse_time_test() {
        assert_eq!(0, parse_time("1970-01-01T00:00:00+00:00"));
        assert_eq!(0, parse_time("1970-01-01T01:00:00+01:00"));
    }

    #[test]
    fn get_noon_test() {
        // http://www.esrl.noaa.gov/gmd/grad/solcalc/
        let times = vec![
            ("2010-06-21T12:01:46+00:00", "2010-06-21T12:00:00+00:00", 45.0, 0.0),
            ("2010-09-23T11:52:25+00:00", "2010-09-23T12:00:00+00:00", 45.0, 0.0),
            ("2010-12-21T11:58:03+00:00", "2010-12-21T12:00:00+00:00", 45.0, 0.0)
        ];

        for (t0, t1, lat, lon) in times {
            assert_eq!(parse_time(t0), get_noon(parse_time(t1), lon));
        }
    }

    #[test]
    fn get_sunrise_test() {
        // TODO: Test at latitudes > 70
        // http://www.esrl.noaa.gov/gmd/grad/solcalc/
        let times = vec![
            ("2010-06-21T04:13:15+00:00", "2010-06-21T12:00:00+00:00", 45.0, 0.0),
            ("2010-09-23T05:48:17+00:00", "2010-09-23T12:00:00+00:00", 45.0, 0.0),
            ("2010-12-21T07:35:09+00:00", "2010-12-21T12:00:00+00:00", 45.0, 0.0)

        ];

        for (t0, t1, lat, lon) in times {
            assert_eq!(parse_time(t0), get_sunrise(parse_time(t1), lon, lat));
        }
    }

    #[test]
    fn get_sunset_test() {
        // TODO: Test at latitudes > 70
        // http://www.esrl.noaa.gov/gmd/grad/solcalc/
        let times = vec![
            ("2010-06-21T19:50:16+00:00", "2010-06-21T12:00:00+00:00", 45.0, 0.0),
            ("2010-09-23T17:56:34+00:00", "2010-09-23T12:00:00+00:00", 45.0, 0.0),
            ("2010-12-21T16:20:58+00:00", "2010-12-21T12:00:00+00:00", 45.0, 0.0)
        ];

        for (t0, t1, lat, lon) in times {
            assert_eq!(parse_time(t0), get_sunset(parse_time(t1), lon, lat));
        }
    }
}
