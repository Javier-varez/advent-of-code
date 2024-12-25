#!/usr/bin/env python

import sys

if len(sys.argv) <= 1:
    print('Using default input file')
    fileName = 'input.txt'
else:
    fileName = sys.argv[1]

def parse(key_or_lock):
    cols = [-1,-1,-1,-1,-1]
    for row in key_or_lock.splitlines():
        for i, c in enumerate(row):
            cols[i] += 1 if c == '#' else 0
    return cols

with open(fileName) as f:
    key_lock_data = f.read().split('\n\n')

locks = [parse(key_or_lock) for key_or_lock in key_lock_data if key_or_lock.splitlines()[0] == '#####']
keys = [parse(key_or_lock) for key_or_lock in key_lock_data if key_or_lock.splitlines()[-1] == '#####']

def valid(p):
    k, l = p
    return (k + l) <= 5

count = 0
for key in keys:
    for lock in locks:
        count += all(map(valid, zip(key, lock)))
print(count)
