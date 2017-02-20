//! Geodate
//!
//! Geodate computes a representation of the current local time in a geocentric
//! date format using a more natural lunisolar calendar with metric time.
//!
//! # Examples
//!
//! ```rust
//! use geodate::geodate;
//!
//! let timestamp = 1403322675;
//! let longitude = -1.826189;
//!
//! println!("{}", geodate::get_lunisolar_date(timestamp, longitude));
//! // 44:05:24:15:42
//! ```
//!
//! This library also exposes some useful functions implementing algorithms
//! from the reference book Astronomical Algorithms by Jean Meeus to calculate
//! the precise time of any sunrise, solstice, and new moon required to create
//! a lunisolar calendar.
//!
//! ```rust
//! use geodate::earth_orbit;
//! use geodate::sun_transit;
//!
//! let timestamp = 1403322675;
//! let longitude = -1.826189;
//! let latitude  = 51.178844;
//!
//! let solstice = earth_orbit::get_previous_december_solstice(timestamp);
//! println!("timestamp of previous december solstice: {}", solstice);
//!
//! if let Some(sunrise) = sun_transit::get_sunrise(timestamp, longitude, latitude) {
//!     println!("timestamp of sunrise: {}", sunrise);
//! }
//! ```
//!
//! Note: some functions available in pair, like `get_*_december_solstice()`
//! returns the `previous` and `next` events for the given time, while others,
//! like `get_sunrise()`, give the event associated with the current implicit
//! time period (day, month).

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod utils;

mod julian;
mod math;

pub mod delta_time;

/// Computes solstices and equinoxes times
pub mod earth_orbit;

/// Constructs string representations of the time in a geodate format
pub mod geodate;

/// Computes phases of the Moon and lunation numbers
pub mod moon_phase;

/// Computes moonrise and moonset times
pub mod moon_transit;

/// Computes sunrise, sunset, midnight, and midday times
pub mod sun_transit;



