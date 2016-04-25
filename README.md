GeoDate
=======

A command line tool displaying the time in a new local geocentric date format.

Synopsis
========

GeoDate displays the current local time in a geocentric date format using a
more natural lunisolar calendar with metric time.

Be prepared to forget all about hours and minutes and start using centidays
instead!

You will also learn to get more in touch with the natural environment with this
lunisolar calendar. For example the full moon will always be around the
fourteenth day of every month, easy!

A detailed explanation of the date format is available
[online](http://files.vinc.cc/calendar.html).


Installation
------------

To install `geodate` from source:

    $ git clone git://github.com/vinc/geodate.git
    $ cd geodate
    $ cargo build --release
    $ sudo cp target/release/geodate /usr/local/bin/


Usage
-----

Run this tool with a longitude and a latitude as arguments and you will get
a local geocentric representation of the time:

    $ geodate -46.90 168.12
    45:06:02:52:92

Add a timestamp to have the date of a particular moment (here it's during the
sunrise on the day of the summer solstice at Stonehenge):

    $ geodate 51.178844 -1.826189 1403322675
    44:05:24:15:34


License
-------

Copyright (C) 2016 Vincent Ollivier. Released under GNU GPL License v3.
