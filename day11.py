import re


def parse(data):
    monkeys = {}

    monkey_commands = [line.strip() for line in data.split('\n\n') if line]

    monkey_re = re.compile(r"\S+\s*(\d+):(.*)")
    key_value = re.compile(r"([^:]+):(.*)")
    list_re = re.compile(r"\s*,\s*")
    operation_re = re.compile(r".*=\s*(\S+)\s*(\+|\*)\s*(\S+)")
    divisible_re = re.compile(r".*?(\d+)")

    for command in monkey_commands:
        # This could be made more robust by not hardcoding the lines and actually
        # parsing the keys, but who cares

        # Monkey id
        lines = command.splitlines()
        monkey_match = monkey_re.match(lines[0])
        assert monkey_match is not None
        monkey_num = int(monkey_match[1])
        monkeys[monkey_num] = {}
        monkey = monkeys[monkey_num]

        # Monkey items
        items = lines[1:]
        starting_items = key_value.match(items[0])
        assert starting_items is not None
        start = list_re.split(starting_items[2].strip())
        monkey['items'] = [int(i) for i in start]

        # Monkey Operation
        operation = key_value.match(items[1])
        assert operation is not None
        operation = operation_re.match(operation[2])
        assert operation is not None
        try:
            lhs = int(operation[1])
        except ValueError:
            lhs = operation[1]
        try:
            rhs = int(operation[3])
        except ValueError:
            rhs = operation[3]
        monkey['operation'] = (lhs, operation[2], rhs)

        # Monkey Diviser
        divisible = key_value.match(items[2])
        assert divisible is not None
        divisible = divisible_re.match(divisible[2])
        assert divisible is not None
        monkey['divisible'] = int(divisible[1])

        # Monkey Target if divisible
        true_option = key_value.match(items[3])
        assert true_option is not None
        true_option = divisible_re.match(true_option[2])
        assert true_option is not None
        monkey['true'] = int(true_option[1])

        # Monkey Target if not divisible
        false_option = key_value.match(items[4])
        assert false_option is not None
        false_option = divisible_re.match(false_option[2])
        assert false_option is not None
        monkey['false'] = int(false_option[1])

    return monkeys


def calc_worry(monkeys: dict, monkey: int, manageable: bool, lcm: int):
    monkeyd = monkeys[monkey]
    old = monkeyd['items'][0]

    modulus = monkeyd['divisible']

    lhs, op, rhs = monkeyd['operation']
    if isinstance(lhs, str):
        lhs = old
    if isinstance(rhs, str):
        rhs = old
    if op == '+':
        new = lhs + rhs
    elif op == '*':
        new = lhs * rhs
    else:
        raise RuntimeError

    if manageable:
        # Warning division doesn't always work with modulus distribution
        new //= 3
    else:
        # Because modulus is distributive: we can reduce the worry without ruining
        # any future calculations, but the modulus n must always be the same
        # throughout the full distribution. We calculate the lcm of all the possible
        # divisors so that it can work with the many divisors of each monkey.
        new = new % lcm

    if new % modulus == 0:
        monkeys[monkeyd['true']]['items'].append(new)
    else:
        monkeys[monkeyd['false']]['items'].append(new)
    monkeyd['items'].pop(0)


def run_round(monkeys: dict, inspections: dict, manageable: bool):
    vals = [monkey['divisible'] for monkey in monkeys.values()]
    lcm = 1
    for val in vals:
        lcm = lcm * val

    for monkey in sorted(list(monkeys.keys())):
        while monkeys[monkey]['items']:
            calc_worry(monkeys, monkey, manageable, lcm)
            inspections[monkey] += 1


def part1(data: str):
    monkeys = parse(data)

    inspections = dict((k, 0) for k in monkeys)
    for _ in range(20):
        run_round(monkeys, inspections, True)
    vals = sorted(list(inspections.values()))[-2:]
    return vals[0] * vals[1]


def part2(data: str):
    monkeys = parse(data)

    inspections = dict((k, 0) for k in monkeys)
    for _ in range(10000):
        run_round(monkeys, inspections, False)
    vals = sorted(list(inspections.values()))[-2:]
    return vals[0] * vals[1]
