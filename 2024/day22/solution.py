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

def generate_prices(n, num_prices):
    prices = []
    prices.append(n % 10)
    for i in range(0, num_prices):
        n = next(n)
        prices.append(n % 10)
    return prices

def calc_delta(price_list):
    last = price_list[0]
    delta = []
    for price in price_list[1:]:
        delta.append(price - last)
        last = price
    return delta

with open(fileName) as f:
    initial_secrets = [int(n.strip()) for n in f.readlines()]

prices = [generate_prices(seed, 2000) for seed in initial_secrets]
price_deltas = [calc_delta(price_list) for price_list in prices]

buyer_maps = []
sequences = {}
for buyer_prices, buyer_deltas in zip(prices, price_deltas):
    cur_map = {}
    for i in range(0, 2000-4):
        seq = tuple(buyer_deltas[i:i+4])
        n = sequences.get(seq, 0)
        sequences[seq] = n+1
        if seq not in cur_map:
            cur_map[seq] = buyer_prices[i + 4]
    buyer_maps.append(cur_map)

sequences = [(count, seq) for seq, count in sequences.items()]
sequences.sort()
sequences.reverse()

current = 0
for count, seq in sequences:
    best_case = count * 9
    if current > best_case:
        break

    total = 0
    for buyer_map in buyer_maps:
        if seq in buyer_map:
            total += buyer_map[seq]

    if total > current:
        current = total

print(f'best is {current}')
