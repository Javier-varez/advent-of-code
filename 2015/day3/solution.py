#!/usr/bin/env python3

txt = open(0).read()

m = {
    '^': (-1, 0),
    'v': (1, 0),
    '<': (0, -1),
    '>': (0, 1),
}

visited = set()

def visit(iterable):
    starting_pos = (0, 0)
    pos = starting_pos
    visited.add(pos)

    for dx,dy in iterable:
        if dx == 0 and dy == 0:
            continue
        x,y = pos
        pos = (x+dx, y+dy)
        visited.add(pos)

instructions = list(map(lambda x: m[x] if x in m else (0,0), txt))
visit(instructions[::2])
visit(instructions[1::2])

print(len(visited))
