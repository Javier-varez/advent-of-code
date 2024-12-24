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

def solve_rec(var, visited):
    if var in outputs: return outputs[var]

    if var in visited: raise Exception("Circular dependency")
    visited.add(var)

    a, instr, b = connections[var]
    a = solve_rec(a, visited)
    b = solve_rec(b, visited)

    visited.remove(var)

    if instr == 'AND':
        return a & b
    if instr == 'OR':
        return a | b
    if instr == 'XOR':
        return a ^ b
    raise Exception(f"Invalid instruction {instr}")

def solve(var):
    return solve_rec(var, set())

n = 0
for var in connections:
    if not var.startswith('z'): continue
    val = solve(var)
    idx = int(var[1:])
    n |= val << idx;

print(n)

def reset_ins():
    for var in outputs:
        outputs[var] = 0

def check_truth_table(xName,yName,prevXName,prevYName,zName):
    reset_ins()
    if solve(zName) != 0:
        return False
    reset_ins()
    outputs[xName] = 1
    if solve(zName) != 1:
        return False
    reset_ins()
    outputs[yName] = 1
    if solve(zName) != 1:
        return False
    reset_ins()
    outputs[xName] = 1
    outputs[yName] = 1
    if solve(zName) != 0:
        return False

    if not prevXName or not prevYName: return True

    reset_ins()
    outputs[prevXName] = 1
    outputs[prevYName] = 1
    if solve(zName) != 1:
        return False

    reset_ins()
    outputs[prevXName] = 1
    outputs[prevYName] = 1
    outputs[xName] = 1
    if solve(zName) != 0:
        return False

    reset_ins()
    outputs[prevXName] = 1
    outputs[prevYName] = 1
    outputs[yName] = 1
    if solve(zName) != 0:
        return False

    reset_ins()
    outputs[prevXName] = 1
    outputs[prevYName] = 1
    outputs[xName] = 1
    outputs[yName] = 1
    if solve(zName) != 1:
        return False

    return True

verified_rules = set()
def register_verified_rules_for(zName):
    if zName in outputs:
        verified_rules.add(zName)
        return

    a, _, b = connections[zName]
    if a not in verified_rules:
        verified_rules.add(a)
        register_verified_rules_for(a)
    if b not in verified_rules:
        verified_rules.add(b)
        register_verified_rules_for(b)

def collect_unverified_rules(zName, unverified_rules):
    if zName in outputs:
        return

    unverified_rules.add(zName)
    a, _, b = connections[zName]
    if a not in verified_rules:
        collect_unverified_rules(a, unverified_rules)
    if b not in verified_rules:
        collect_unverified_rules(b, unverified_rules)

def check_error():
    verified_rules = set()
    for z in range(0, 45):
        xName = f'x{z:02}'
        yName = f'y{z:02}'
        zName = f'z{z:02}'

        prevXName = None
        prevYName = None
        if z != 0:
            prevXName = f'x{z-1:02}'
            prevYName = f'y{z-1:02}'

        if not check_truth_table(xName, yName, prevXName, prevYName,zName):
            unverified_rules = set()
            collect_unverified_rules(zName, unverified_rules)
            return z, unverified_rules
        else:
            register_verified_rules_for(zName)

    return None, None

swapped_nodes = set()
while True:
    errIdx, unverified_rules = check_error()
    if errIdx is None:
        print('solved')
        print(f'swapped_nodes {",".join(sorted(swapped_nodes))}')
        break

    print(f'found first error at {errIdx}, unverified_rules{unverified_rules}')

    fixed = False
    for swap_a in unverified_rules:
        for swap_b in connections:
            # Ignore rules we know are correct
            if swap_a in verified_rules or swap_b in verified_rules or swap_b == swap_a: continue
            saved_a = connections[swap_a]
            saved_b = connections[swap_b]
            connections[swap_a] = saved_b
            connections[swap_b] = saved_a

            try:
                newIdx, _ = check_error()
                if newIdx is None or newIdx > errIdx:
                    swapped_nodes.add(swap_a)
                    swapped_nodes.add(swap_b)
                    print(f'found correct swap at {swap_a}, {swap_b}')
                    fixed = True
                    break
            except:
                pass

            connections[swap_a] = saved_a
            connections[swap_b] = saved_b

        if fixed:
            break
    assert fixed
