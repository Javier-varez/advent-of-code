#!/usr/bin/env python3

def is_nice(string):
    num_dup_seq = sum(map(lambda c: 1 if string.count(f"{c[0]}{c[1]}") > 1 else 0, zip(list(string), list(string)[1:])))
    num_dup_with_letter_in_between = sum(map(lambda c: 1 if c[0] == c[1] else 0, zip(list(string), list(string)[2:])))
    return num_dup_with_letter_in_between >= 1 and num_dup_seq >= 1

print(sum(map(lambda line: 1 if is_nice(line) else 0, open(0))))
