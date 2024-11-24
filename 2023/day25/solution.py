#!/usr/bin/env python3

# to be totally fair, this solution was taken from hyper-neutrino.
# all I could come up with without understanding graph theory is
# a brute-force approach removing each set of 3 edges

import networkx

g = networkx.Graph()

for line in open(0):
    node, nodes = line.split(":")
    nodes = list(map(lambda l: l.strip(), nodes.split()))
    for other in nodes:
        g.add_edge(node, other)
        g.add_edge(other, node)

g.remove_edges_from(networkx.minimum_edge_cut(g))
a, b = networkx.connected_components(g)

print(len(a) * len(b))
