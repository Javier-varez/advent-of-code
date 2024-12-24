#!/usr/bin/env python

import sys

if len(sys.argv) <= 1:
    print('Using default input file')
    fileName = 'input.txt'
else:
    fileName = sys.argv[1]

with open(fileName) as f:
    data = f.read()

initial_value_str, connection_str = data.split('\n\n')

outputs = {}
for line in initial_value_str.splitlines():
    variable, value = [x.strip() for x in line.split(':')]
    outputs[variable] = int(value)

connections = {}
for instr in connection_str.splitlines():
    instr, target = [x.strip() for x in instr.split('->')]
    a, instr, b = instr.split(' ')
    connections[target] = (a, instr, b);

def solve(var):
    if var in outputs: return outputs[var]
    a, instr, b = connections[var]
    a = solve(a)
    b = solve(b)
    if instr == 'AND':
        return a & b
    if instr == 'OR':
        return a | b
    if instr == 'XOR':
        return a ^ b
    raise f"Invalid instruction {instr}"

n = 0
for var in connections:
    if not var.startswith('z'): continue
    val = solve(var)
    idx = int(var[1:])
    n |= val << idx;

print(n)
