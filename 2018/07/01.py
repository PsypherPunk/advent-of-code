#!/usr/bin/env python3

import networkx

dg = networkx.DiGraph()
with open("input.txt") as i:
    dg.add_edges_from((x[5], x[36]) for x in i)

print("".join(networkx.lexicographical_topological_sort(dg)))

