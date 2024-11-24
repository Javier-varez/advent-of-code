#!/usr/bin/env python3

import sympy

input = [tuple(map(lambda coords: tuple(map(int, coords.split(","))), line.split("@"))) for line in open(0)]

prx, pry, prz, srx, sry, srz = sympy.symbols("prx pry prz srx sry srz")

# (pos[0] - p_rock[0]) / (s_rock[0] - speed[0]) = (pos[1] - p_rock[1]) / (s_rock[1] - speed[1])

eqs = []
for i, ((px, py, pz), (sx, sy, sz)) in enumerate(input):
    eqs.append((px - prx) * (sry - sy) + (py - pry) * (sx - srx))
    eqs.append((px - prx) * (srz - sz) + (pz - prz) * (sx - srx))

sol = sympy.solve(eqs)
sol = sol[0]
print(sol)
print(sol[prx] + sol[pry] + sol[prz])
