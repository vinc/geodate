use sun_transit::*;
use earth_orbit::*;
use moon_phase::*;
use moon_transit::*;

use std::collections::BTreeMap;

/// Get the ephemeris of a geodate
pub fn get_ephemeris(timestamp: i64, longitude: f64, latitude: f64) -> BTreeMap<i64, String> {
    let mut events = BTreeMap::new();

    let day_begin_at = get_midnight(timestamp, longitude);
    let day_end_at = get_midnight(day_begin_at + 86400 + 10000, longitude);

    events.insert(timestamp, "Current".to_string());

    let es = vec![
        ("Equinox", get_next_march_equinox(day_begin_at)),
        ("Equinox", get_next_september_equinox(day_begin_at)),
        ("Solstice", get_next_december_solstice(day_begin_at)),
        ("Solstice", get_next_june_solstice(day_begin_at))
    ];
    for (name, e) in es {
        if e < day_end_at {
            events.insert(e, name.to_string());
        }
    }

    let n = get_lunation_number(day_begin_at); // FIXME: Potential bug here
    let es = vec![
        ("New Moon", get_new_moon(n)),
        ("First Quarter Moon", get_first_quarter_moon(n + 0.25)),
        ("Full Moon", get_full_moon(n + 0.50)),
        ("Last Quarter Moon", get_last_quarter_moon(n + 0.75))
    ];
    for (name, e) in es {
        if day_begin_at < e && e < day_end_at {
            events.insert(e, name.to_string());
        }
    }

    if let Some(moonrise) = get_moonrise(timestamp, longitude, latitude) {
        if moonrise < day_begin_at {
            if let Some(moonrise) = get_moonrise(timestamp + 86400, longitude, latitude) {
                if day_begin_at <= moonrise && moonrise <= day_end_at {
                    events.insert(moonrise, "Moonrise+1".to_string());
                } else {
                    //events.insert(moonrise, "Moonrise +1");
                }
            }
        } else if moonrise > day_end_at {
            if let Some(moonrise) = get_moonrise(timestamp - 86400, longitude, latitude) {
                if day_begin_at <= moonrise && moonrise <= day_end_at {
                    events.insert(moonrise, "Moonrise-1".to_string());
                } else {
                    //events.insert(moonrise, "Moonrise -1");
                }
            }
        } else {
            events.insert(moonrise, "Moonrise".to_string());
        }
    }

    if let Some(moonset) = get_moonset(timestamp, longitude, latitude) {
        if moonset < day_begin_at {
            if let Some(moonset) = get_moonset(timestamp + 86400, longitude, latitude) {
                if day_begin_at <= moonset && moonset <= day_end_at {
                    events.insert(moonset, "Moonset".to_string());
                }
            }
        } else if moonset > day_end_at {
            if let Some(moonset) = get_moonset(timestamp - 86400, longitude, latitude) {
                if day_begin_at <= moonset && moonset <= day_end_at {
                    events.insert(moonset, "Moonset".to_string());
                }
            }
        } else {
            events.insert(moonset, "Moonset".to_string());
        }
    }

    if let Some(sunrise) = get_sunrise(timestamp, longitude, latitude) {
        events.insert(sunrise, "Sunrise".to_string());
    }

    if let Some(sunset) = get_sunset(timestamp, longitude, latitude) {
        events.insert(sunset, "Sunset".to_string());
    }

    events
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::*;

    #[test]
    fn get_ephemeris_test() {
        assert_eq!("00:00:00:00:00", get_ephemeris(parse_time("1970-01-07T00:06:15+0000"), 0.0, 0.0));
    }
}
