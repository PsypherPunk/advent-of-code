======
Python
======

It feels like cheating but this is only a handful of lines in Python, thanks
to the `networkx`_ module:

.. _networkx: https://networkx.github.io/

.. code:: python

    import networkx


    graph = networkx.DiGraph()

    with open("input.txt") as i:
        for orbit in i.readlines():
            graph.add_edge(*(obj.strip() for obj in orbit.split(")")))

    print(networkx.transitive_closure(graph).size())
    print(networkx.shortest_path_length(graph.to_undirected(), "YOU", "SAN") - 2)

But we're not doing Python this year…
