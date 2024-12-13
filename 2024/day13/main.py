#!/usr/bin/env python


def process(machine):
    lines = machine.splitlines()
    buttonA = [int(x.strip()[2:]) for x in lines[0].split(':')[1].split(',')]
    buttonB = [int(x.strip()[2:]) for x in lines[1].split(':')[1].split(',')]
    prize = [int(x.strip()[2:]) + 10000000000000 for x in lines[2].split(':')[1].split(',')]
    return buttonA, buttonB, prize

def load():
    with open('./realinput.txt', 'r') as f:
        input = f.read()
        return [process(machine) for machine in input.split('\n\n')]

totalToks = 0
for buttonA, buttonB, prize in load():
    print('machine:')
    print(f'buttonA: {buttonA}')
    print(f'buttonB: {buttonB}')
    print(f'prize: {prize}')

    num = buttonA[1] * prize[0] / buttonA[0] - prize[1]
    denom = (buttonA[1] * buttonB[0]) / buttonA[0] - buttonB[1]

    numB = int(round(num / denom))
    numA = int((prize[0] - buttonB[0] * numB) / buttonA[0])

    if (numA * buttonA[0] + numB * buttonB[0]) != prize[0]:
        continue

    if (numA * buttonA[1] + numB * buttonB[1]) != prize[1]:
        continue

    toks =numA*3+numB
    print(f'A = {numA}, B = {numB}, toks = {toks}')
    totalToks += toks

print(f'total toks = {totalToks}')
