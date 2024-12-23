#!/usr/bin/env python

import sys

if len(sys.argv) <= 1:
    print('Using default input file')
    fileName = 'input.txt'
else:
    fileName = sys.argv[1]

with open(fileName) as f:
    connection_list = [line.strip().split('-') for line in f.readlines()]

connections = {}
def add_connection(a, b):
    if a in connections:
        connections[a].append(b)
    else:
        connections[a] = [b]

for a, b in connection_list:
    add_connection(a,b)
    add_connection(b,a)

sets_of_3 = set()
for a,b in connection_list:
    if not a.startswith('t') and not b.startswith('t'): continue

    for c in connections[a]:
        if c in connections[b]:
            cur = [a,b,c]
            cur.sort()
            sets_of_3.add(tuple(cur))

print(len(sets_of_3))
