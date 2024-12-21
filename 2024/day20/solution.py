#!/usr/bin/env python

import sys
import heapq

if len(sys.argv) <= 1:
    print('please give me an input file')
    sys.exit(1)

with open(sys.argv[1]) as f:
    map = [list(line.strip()) for line in f.readlines()]
    height = len(map)
    width = len(map[0])
    for r, line in enumerate(map):
        for c, byte in enumerate(line):
            if byte == 'S':
                start = (r,c)
            if byte == 'E':
                end = (r,c)

def solveMinPath(map):
    pq = []
    sr, sc = start
    er, ec = end

    cost_map = {}

    heapq.heappush(pq, (0, sr, sc))

    while pq:
        cost, r,c = heapq.heappop(pq)
        if cost >= cost_map.get((r,c), float('inf')):
            continue

        cost_map[(r,c)] = cost

        if (r,c) == end:
            break

        for nr, nc in [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)]:
            if nr < 0 or nc < 0 or nr >= len(map) or nc >= len(map[nr]) or map[nr][nc] == '#':
                continue
            heapq.heappush(pq, (cost + 1, nr, nc))

    return cost_map

cost_map = solveMinPath(map)

baseline = cost_map[end]

print(f'height = {height}, width = {width}')
print(f'cost = {baseline}')

save_amounts = {}
more_than_100 = 0

for r in range(0, height):
    for c in range(0, width):
        if map[r][c] == '#':
            continue

        # try to cheat from here to all surrounding positions within 2 picoseconds
        for (nr,nc) in [(r + 1, c + 1), (r + 2, c), (r, c + 2), (r - 1, c + 1)]:
            if nr < 0 or nc < 0 or nr >= len(map) or nc >= len(map[nr]) or map[nr][nc] == '#':
                continue

            # valid cheat
            cost_a = cost_map[(r,c)]
            cost_b = cost_map[(nr,nc)]

            saved = abs(cost_a - cost_b) - 2
            if saved in save_amounts:
                save_amounts[saved] += 1
            else:
                save_amounts[saved] = 1

            if saved >= 100:
                more_than_100 += 1

for amount, times in save_amounts.items():
    print(f'there are {times} cheats that save {amount} ps')

print(f'more than 100: {more_than_100}')
