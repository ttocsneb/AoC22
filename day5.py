import re


def parse(data: str):
    split = data.split('\n\n')
    struct = split[0]

    lines = list(reversed(struct.splitlines()))
    first = lines[0]

    # Find the number of columns
    num_items = len([i for i in first.split() if i])
    stacks = [[] for _ in range(num_items)]

    # Parse the grid
    for line in lines[1:]:
        for i, index in enumerate(range(1, num_items * 4, 4)):
            value = line[index]
            if value != ' ':
                stacks[i].append(value)

    # Parse the commands
    group = re.compile(r"\w+ (\d+) \w+ (\d+) \w+ (\d+)")
    commands = []
    for command in split[1].splitlines():
        match = re.match(group, command)
        if match:
            commands.append((int(match[1]), int(match[2]), int(match[3])))

    return stacks, commands


def part1(data: str):
    stacks, commands = parse(data)

    for (n, f, t) in commands:
        for _ in range(n):
            val = stacks[f - 1].pop()
            stacks[t - 1].append(val)

    buf = ''
    for stack in stacks:
        buf += stack[-1]
    return buf


def part2(data: str):
    stacks, commands = parse(data)

    for (n, f, t) in commands:
        new_stack = []
        for _ in range(n):
            new_stack.append(stacks[f - 1].pop())
        for val in reversed(new_stack):
            stacks[t - 1].append(val)

    buf = ''
    for stack in stacks:
        buf += stack[-1]
    return buf
