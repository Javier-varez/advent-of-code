#!/usr/bin/env python3

text = open(0).read()
# part 1
res = sum(map(lambda x: 1 if x == '(' else -1 if x == ')' else 0, text))
print(res)

# part 2
cur = 0
for i, c in enumerate(map(lambda x: 1 if x == '(' else -1 if x == ')' else 0, text)):
    cur = cur + c
    if cur == -1:
        print(i+1)
