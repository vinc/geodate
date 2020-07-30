Geodate
=======

A command line tool displaying the time in a geocentric date format.


Synopsis
--------

Geodate displays the current local time in a geocentric date format using a
natural lunisolar calendar with metric time based on decimal fractions of the
mean solar day.

Be prepared to forget all about hours and minutes and start using centidays
instead! For example noon is the middle of the day or 50 centidays after
midnight.

You will also learn to be more in touch with the natural environment with this
lunisolar calendar. For example the full moon will always be around the middle
of every month, easy, just look up in the sky to know the time.

A detailed explanation of the date format is available
[online](https://vinc.cc/units/geodate).


Installation
------------

First you need to install Rust:

    $ curl https://sh.rustup.rs -sSf | sh

Then you can install the latest stable version with cargo:

    $ cargo install geodate

Or the development version by fetching the git repository:

    $ git clone git://github.com/vinc/geodate.git
    $ cd geodate
    $ cargo build --release
    $ sudo cp target/release/geodate /usr/local/bin/


Usage
-----

Run this tool with a latitude and a longitude and you will get a geocentric
expression of the current local time:

    $ geodate -46.8995 168.1269
    01:15:06:02:52:92

Add a timestamp to get the date of a particular event (for example at sunrise
on the day of the summer solstice at Stonehenge):

    $ geodate 51.1789 -1.8262 1403322675
    01:14:05:24:15:42

Geodate can also be run in ephemeris mode with the `--ephem` flag:

    $ geodate --ephem 51.1789 -1.8262 1403322675
    Moonrise:            01:14:05:24:01:57
    Current:             01:14:05:24:15:42
    Sunrise:             01:14:05:24:15:46
    Solstice:            01:14:05:24:44:61
    Moonset:             01:14:05:24:58:86
    Sunset:              01:14:05:24:84:53

Finally you can always add a `--machine` flag to get a unix timestamp
instead of the default human format, and `--unix` to change epoch and
begin to count the years from 1970 like computers do instead of 1900
which is more in line with what humans are used to.


Algorithms
----------

Geodate implements a lot of algorithms described in the book Astronomical
Algorithms by Jean Meeus to calculate the precise time of any sunrise,
solstice, and new moon required to create a lunisolar calendar.

Additional astronomical events such as moonrise or equinox are also calculated
in ephemeris mode.


License
-------

Copyright (c) 2016-2020 Vincent Ollivier. Released under the MIT License.
