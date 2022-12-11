
def parse(data: str):
    output = []

    lines = [line.strip() for line in data.splitlines() if line]

    for line in lines:
        split = line.split(' ')
        cmd = split[0]
        if cmd != "noop":
            output.append((split[0], int(split[1])))
        else:
            output.append((split[0], 1))

    return output


def run(commands: list[tuple[str, int]]):
    cycle = 0
    register = 1
    for cmd, value in commands:
        if cmd == 'addx':
            cycle += 1
            yield cycle, register
            cycle += 1
            yield cycle, register
            register += value
        elif cmd == 'noop':
            cycle += 1
            yield cycle, register


def part1(data: str):
    commands = parse(data)

    strengths = []
    for cycle, register in run(commands):
        if (cycle + 20) % 40 == 0:
            strengths.append((cycle, register, cycle * register))

    return sum(i[2] for i in strengths)


def render(data: list[str]) -> str:
    buffer = []
    for i in range(6):
        buffer.append(''.join(data[i * 40: (i + 1) * 40]))

    return '\n'.join(buffer)


def part2(data: str):
    commands = parse(data)

    output = [' '] * (40 * 6)
    gpu = 0
    for _, register in run(commands):
        if register - 1 <= ((gpu % 40)) <= register + 1:
            output[gpu] = '#'
        gpu += 1
        if gpu >= len(output):
            gpu = 0

    return render(output)
