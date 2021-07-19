//! Geodate
//!
//! Geodate computes geocentric expressions of points in time using
//! a natural lunisolar calendar with metric time based on decimal fractions
//! of the mean solar day.
//!
//! # Examples
//!
//! ```rust
//! use geodate::geodate;
//!
//! let timestamp = 1403322675;
//! let longitude = -1.826189;
//!
//! assert_eq!("01:14:05:24:15:42", geodate::get_date(timestamp, longitude));
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
//! assert_eq!(1387645873, solstice);
//!
//! if let Some(sunrise) = sun_transit::get_sunrise(timestamp, longitude, latitude) {
//!     assert_eq!(1403322705, sunrise);
//! }
//! ```
//!
//! Note: some functions available in pair, for example `get_*_december_solstice()`
//! return the `previous` and `next` events for the given time, while others,
//! like `get_sunrise()`, give the event associated with the current implicit
//! time period (day, month).

#![no_std]
#[cfg(feature = "std")]
extern crate std;

#[macro_use]
extern crate alloc;
extern crate num_traits;

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

/// Computes ephemeris
pub mod ephemeris;

/// Reverse a geodate into a timestamp
pub mod reverse;
