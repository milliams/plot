Plot
====

A command-line statistics and plotting tool.

``plot`` works on input streams and so if you have a file called ``data.txt`` which looks like:

.. code-block:: bash

    $ head -n4 data.txt
    11.01821751894866
    -3.862915996857989
    4.293330805873133
    2.6587850003804734

then you can plot a histogram of that with

.. code-block:: bash

    $ cat data.txt | plot hist

which will output something like::

    12-|                                   ----
       |                                   |  |
       |                       -------     |  |
       |                       |  |  |     |  |
       |                       |  |  |     |  |
       |                       |  |  |     |  |
       |                       |  |  |     |  |
       |                       |  |  |  ---|  |---
       |                       |  |  |  |  |  |  |
       |                       |  |  |  |  |  |  |
     8-|                       |  |  |  |  |  |  |---
       |                       |  |  |  |  |  |  |  |
       |                       |  |  |  |  |  |  |  |
       |                       |  |  |  |  |  |  |  |
       |                       |  |  |  |  |  |  |  |
       |                    ---|  |  |  |  |  |  |  |---
       |                    |  |  |  |  |  |  |  |  |  |
       |                 ---|  |  |  |--|  |  |  |  |  |
       |                 |  |  |  |  |  |  |  |  |  |  |
       |                 |  |  |  |  |  |  |  |  |  |  |
     4-|                 |  |  |  |  |  |  |  |  |  |  |        ----
       |                 |  |  |  |  |  |  |  |  |  |  |        |  |
       |                 |  |  |  |  |  |  |  |  |  |  |------  |  |
       |                 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
       |                 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
       |---        ----  |  |  |  |  |  |  |  |  |  |  |  |  |--|  |
       |  |        |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
       |  |  ----  |  |--|  |  |  |  |  |  |  |  |  |  |  |  |  |  |
       |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
       |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
      0+------------------------------------------------------------
           |           |           |          |           |
          -10         -5           0          5          10

Likewise, if you have a data file (or any pipeable stream) which has a two-column format like:

.. code-block:: bash

    $ cat data2.txt
    -3 2.3
    -1.6 5.3
    0.3 0.7
    4.3 -1.4
    6.4 4.3
    8.5 3.7

then you can draw a scatter plot of data using

.. code-block:: bash

    $ cat data2.txt | plot scatter

which outputs::

      |        o
      |
      |
    4-|                                              o
      |                                                        o
      |
      |
      |
      |  o
    2-|
      |
      |
      |                 o
      |
    0-|
      |
      |
      |
      |                 o
      +------------------------------------------------------------
                       |                  |                  |
                       0                  4                  8
