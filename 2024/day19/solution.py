#!/usr/bin/env python


with open('realinput.txt', 'r') as f:
    lines = f.readlines()

towels = [towel.strip() for towel in lines[0].split(', ')]

patterns = [pattern.strip() for pattern in lines[2:]]

def join(left, right):
    newlist = list(left)
    newlist.extend(right)
    return tuple(newlist)

memory = {}
def possible_arrangements(pattern):
    input = (tuple(pattern))
    if input in memory:
        return memory[input]

    if pattern == '':
        return 1

    count = 0
    for i, towel in enumerate(towels):
        if not pattern.startswith(towel):
            continue

        sub = pattern[len(towel):]
        count += possible_arrangements(sub)

    memory[input] = count
    return count

print(sum(possible_arrangements(p) for p in patterns))
