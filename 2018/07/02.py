#!/usr/bin/env python3

import networkx

dg = networkx.DiGraph()
with open("input.txt") as i:
    dg.add_edges_from((x[5], x[36]) for x in i)

part_one = "".join(networkx.lexicographical_topological_sort(dg))

print(max(sum(map(lambda x: (ord(x) - 64) + 60, path)) for path in networkx.all_simple_paths(dg, source=part_one[0], target=part_one[-1])))

