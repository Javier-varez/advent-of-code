#!/usr/bin/env python


with open('realinput.txt', 'r') as f:
    lines = f.readlines()

towels = [towel.strip() for towel in lines[0].split(', ')]

patterns = [pattern.strip() for pattern in lines[2:]]

memory = {}

def is_possible(pattern):
    input = tuple(pattern)
    if input in memory:
        return memory[input]

    if pattern == '':
        return True

    for towel in towels:
        parts = pattern.split(towel)
        if len(parts) == 1 and parts[0] == pattern:
            continue

        ok = True
        for part in parts:
            ok = ok and is_possible(part)

        if ok:
            memory[input] = ok
            return ok

    memory[input] = False
    return False

possible_patterns = [is_possible(pattern) for pattern in patterns]
print(len([valid for valid in possible_patterns if valid]))
