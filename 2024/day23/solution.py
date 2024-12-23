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

all_interconnected = set()
def search(cur, req):
    k = tuple(sorted(list(req)))
    if k in all_interconnected: return
    all_interconnected.add(k)

    for n in connections[cur]:
        # Check if it is already in the required set, in that case we don't need to add it again
        if n in req: continue
        # Check if it is not connected to every other node in req
        if not all(n in connections[o] for o in req): continue
        copy = req.copy()
        copy.add(n)
        search(n, copy)

for n in connections:
    s = set()
    s.add(n)
    search(n, s)

print(",".join(sorted(sorted(all_interconnected, key=len, reverse=True)[0])))
