Detri
=====

A command line tool displaying a lunisolar calendar


Installation
------------

To install Detri from source:

    $ git clone git://github.com/vinc/detri.git
    $ cd detri
    $ cargo build --release
    $ sudo cp target/release/detri /usr/local/bin/


Usage
-----

If you were watching the sunrise on the day of the winter solstice at
Stonehenge and runing this tool at the same time (what a crazy idea,
look at the sun, it's beautiful!), you would see this output:

    $ detri 51.178844 -1.826189
    45:12:11:33:54


License
-------

Copyright (C) 2016 Vincent Ollivier. Released under GNU GPL License v3.
