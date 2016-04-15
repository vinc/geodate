GeoDate
=======

A command line tool displaying the time in a local geocentric date format.


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
