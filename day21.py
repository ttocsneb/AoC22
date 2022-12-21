import sympy
import re


def parse(data: str):
    parsed = {}

    lines = [line.strip() for line in data.splitlines() if line]

    regex = re.compile(r"(\w+)\s+(\+|-|/|\*)\s+(\w+)")

    for line in lines:
        split = line.split(':')

        m = regex.match(split[1].strip())
        if m is not None:
            parsed[split[0]] = (m[1], m[2], m[3])
        else:
            parsed[split[0]] = int(split[1].strip())

    return parsed


def part1(data: str):
    parsed = parse(data)

    to_calc = ['root']

    while to_calc:
        next_calc = to_calc.pop()
        a = parsed[next_calc][0]
        b = parsed[next_calc][2]
        a_valid = isinstance(parsed[a], tuple)
        b_valid = isinstance(parsed[b], tuple)
        if a_valid or b_valid:
            to_calc.append(next_calc)
            if a_valid:
                to_calc.append(a)
            if b_valid:
                to_calc.append(b)
            continue

        expr = f'{parsed[a]}{parsed[next_calc][1]}{parsed[b]}'
        val = eval(expr)
        parsed[next_calc] = val

    return int(parsed['root'])


def part2(data: str):
    parsed = parse(data)

    root = parsed['root']

    humn = sympy.Symbol('humn')

    def solve(monkey: str):
        if monkey == 'humn':
            return humn
        eq = parsed[monkey]
        if not isinstance(eq, tuple):
            return eq

        a = solve(eq[0])
        op = eq[1]
        b = solve(eq[2])

        if op == '/':
            return a / b
        if op == '*':
            return a * b
        if op == '+':
            return a + b
        if op == '-':
            return a - b

        raise NotImplementedError("Operation not supported")

    a = solve(root[0])
    b = solve(root[2])

    print(f'{a} == {b}')

    return int(sympy.solve(a - b, humn)[0])
