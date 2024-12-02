#!/usr/bin/env python3

SIZE = 1000

def handle_instruction(state, insn, start, end):
    start_row, start_col = start
    end_row, end_col = end
    for i in range(start_row, end_row + 1):
        for j in range(start_col, end_col + 1):
            if insn == 'turn on':
                updated = state[i][j] + 1
            elif insn == 'turn off':
                updated = state[i][j] - 1
                if updated < 0:
                    updated = 0
            elif insn == 'toggle':
                updated = state[i][j] + 2
            else:
                raise "Oh no! unexpected instruction!"
            state[i][j] = updated

def parse_coord(string):
    row, col = map(lambda x: int(x.strip()), string.split(","))
    return row, col

state = [[0 for _ in range(SIZE)] for _ in range(SIZE)]

for line in open(0):
    if line.startswith("turn on"):
        prefix = "turn on"
    elif line.startswith("turn off"):
        prefix = "turn off"
    elif line.startswith("toggle"):
        prefix = "toggle"
    else:
        raise "Oh no! unexpected instruction!"

    start, end = map(parse_coord, line[len(prefix):].split("through"))
    handle_instruction(state, prefix, start, end)

count = sum(map(lambda x: sum(x), state))
print(count)
