#!/usr/bin/env python3

def calc_ribbon(line):
    a,b,c = map(int, line.split("x"))
    v = a * b * c
    s = [a, b, c]
    s.sort()
    return v + 2 * (s[0] + s[1])

total_area = sum(list(map(calc_ribbon, open(0))))
print(total_area)
