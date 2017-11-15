Geodate
=======

A command line tool displaying the time in a local geocentric date format.


Synopsis
--------

Geodate displays the current local time in a geocentric date format using a
more natural lunisolar calendar with metric time.

Be prepared to forget all about hours and minutes and start using centidays
instead!

You will also learn to get more in touch with the natural environment with this
lunisolar calendar. For example the full moon will always be around the middle
of every month, easy, just look up in the sky to know the time.

A detailed explanation of the date format is available
[online](https://vinc.cc/essays/geocalendar.html).


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

Run this tool with a latitude and a longitude and you will get
a geocentric expression of the current local time:

    $ geodate -46.90 168.12
    45:06:02:52:92

Add a timestamp to get the date of a particular event (for example at sunrise
on the day of the summer solstice at Stonehenge):

    $ geodate 51.178844 -1.826189 1403322675
    44:05:24:15:42

Geodate can also be run in ephemeris mode with the `--ephem` flag:

    $ geodate --ephem 51.178844 -1.826189 1403322675
    Moonrise:            44:05:24:01:57
    Current:             44:05:24:15:42
    Sunrise:             44:05:24:15:46
    Solstice:            44:05:24:44:61
    Moonset:             44:05:24:58:85
    Sunset:              44:05:24:84:52

Finally you can always add a `--machine` flag to get a unix timestamp
instead of the default human format.


Algorithms
----------

Geodate implements a lot of algorithms described in the book Astronomical
Algorithms by Jean Meeus to calculate the precise time of any sunrise,
solstice, and new moon required to create a lunisolar calendar.

Additional astronomical events such as moonrise or equinox are also calculated
in ephemeris mode.


License
-------

Copyright (c) 2016-2017 Vincent Ollivier. Released under the MIT License.
