#!/usr/bin/env python

import sys

if len(sys.argv) <= 1:
    print('Using default input file')
    fileName = 'input.txt'
else:
    fileName = sys.argv[1]

def mix(s, n):
    return s ^ n

def prune(n):
    return n % 16777216

def next(n):
    n = mix(n, n * 64)
    n = prune(n)
    n = mix(n, int(n / 32))
    n = prune(n)
    n = mix(n, n * 2048)
    n = prune(n)
    return n

def nth(n, t):
    for i in range(0,t):
        n = next(n)
    return n

with open(fileName) as f:
    result = sum([nth(int(n.strip()), 2000) for n in f.readlines()])
    print(result)
