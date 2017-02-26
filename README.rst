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

which will output something like

.. code-block:: bash

    12-|                  ###            ###
       |                  ###            ###
       |                  ###         ######
       |                  ###      #########
       |                  ###      #########
       |                  ######   #########
       |                  ######   #########
     8-|                  ######   #########
       |                  ######   ############   ###
       |                  ######   ############   ###
       |                  #####################   ###
       |                  #####################   ###
       |                  #####################   ###
     4-|         ###      ###########################
       |         ###      ###########################
       |         ####################################
       |         ####################################
       |   #############################################   ###   ###
       |######################################################   ###
       |######################################################   ###
      0+------------------------------------------------------------
              |            |            |            |            |
             -5            0            5           10           15

For now it only provides simple histogram plotting
but there are plans to also add some line graph, bar charts and other options.
