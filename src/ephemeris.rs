use sun_transit::*;
use earth_orbit::*;
use moon_phase::*;
use moon_transit::*;

use alloc::collections::BTreeMap;
use alloc::string::ToString;
use alloc::string::String;

/// Get the ephemeris of a geodate
pub fn ephemeris(timestamp: i64, longitude: f64, latitude: f64) -> BTreeMap<i64, String> {
    let mut events = BTreeMap::new();

    let day_begin_at = midnight(timestamp, longitude);
    let day_end_at = midnight(day_begin_at + 86400 + 10000, longitude);

    events.insert(timestamp, "Current".to_string());

    let es = vec![
        ("Equinox", next_march_equinox(day_begin_at)),
        ("Equinox", next_september_equinox(day_begin_at)),
        ("Solstice", next_december_solstice(day_begin_at)),
        ("Solstice", next_june_solstice(day_begin_at))
    ];
    for (name, e) in es {
        if e < day_end_at {
            events.insert(e, name.to_string());
        }
    }

    let n = lunation_number(day_begin_at); // FIXME: Potential bug here
    let es = vec![
        ("New Moon", new_moon(n)),
        ("First Quarter Moon", first_quarter_moon(n + 0.25)),
        ("Full Moon", full_moon(n + 0.50)),
        ("Last Quarter Moon", last_quarter_moon(n + 0.75))
    ];
    for (name, e) in es {
        if day_begin_at < e && e < day_end_at {
            events.insert(e, name.to_string());
        }
    }

    if let Some(event) = moonrise(timestamp, longitude, latitude) {
        if event < day_begin_at {
            if let Some(event) = moonrise(timestamp + 86400, longitude, latitude) {
                if day_begin_at <= event && event <= day_end_at {
                    events.insert(event, "Moonrise+1".to_string());
                } else {
                    //events.insert(event, "Moonrise +1"); // FIXME?
                }
            }
        } else if event > day_end_at {
            if let Some(event) = moonrise(timestamp - 86400, longitude, latitude) {
                if day_begin_at <= event && event <= day_end_at {
                    events.insert(event, "Moonrise-1".to_string());
                } else {
                    //events.insert(event, "Moonrise -1"); // FIXME?
                }
            }
        } else {
            events.insert(event, "Moonrise".to_string());
        }
    }

    if let Some(event) = moonset(timestamp, longitude, latitude) {
        if event < day_begin_at {
            if let Some(event) = moonset(timestamp + 86400, longitude, latitude) {
                if day_begin_at <= event && event <= day_end_at {
                    events.insert(event, "Moonset".to_string());
                }
            }
        } else if event > day_end_at {
            if let Some(event) = moonset(timestamp - 86400, longitude, latitude) {
                if day_begin_at <= event && event <= day_end_at {
                    events.insert(event, "Moonset".to_string());
                }
            }
        } else {
            events.insert(event, "Moonset".to_string());
        }
    }

    if let Some(event) = sunrise(timestamp, longitude, latitude) {
        events.insert(event, "Sunrise".to_string());
    }

    if let Some(event) = sunset(timestamp, longitude, latitude) {
        events.insert(event, "Sunset".to_string());
    }

    events
}
