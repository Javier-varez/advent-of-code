#!/usr/bin/env python

import heapq

with open("./realinput.txt", 'r') as f:
    coords = [list(map(lambda v: int(v), l.split(','))) for l in f.readlines()]

def construct_map(dimension, pieces):
    m = [['.' for j in range(0, dimension + 1)] for i in range(0, dimension + 1)]
    for x,y in pieces:
        m[y][x] = '#'
    return m

# dimension = 6
# num_pieces = 12
dimension = 70
num_pieces = 1024

start = (0, 0)
end = (dimension, dimension)
map = construct_map(dimension, coords[:num_pieces])

pq = []
heapq.heappush(pq, (0, start[0], start[1]))
min_cost = {}

while pq:
    cost,x,y = heapq.heappop(pq)
    if cost > min_cost.get((x,y), float("inf")):
        continue

    min_cost[(x,y)] = cost
    if (x,y) == end:
        break;

    for (nx,ny) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]:
        if nx < 0 or ny < 0 or nx > dimension or ny > dimension:
            continue

        if map[ny][nx] == '#':
            continue

        cur_cost = cost + 1
        if not (cur_cost, nx, ny) in pq:
            heapq.heappush(pq, (cur_cost, nx, ny))

print(f'{min_cost[end]}')
