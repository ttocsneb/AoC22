from typing import Literal


def parse(data: str):
    parsed = []

    lines = [line.strip() for line in data.split('\n\n') if line]

    for packets in lines:
        lines = packets.splitlines()
        # I don't like this, but it's the easiest way to parse
        parsed.append((
            eval(lines[0]),
            eval(lines[1]),
        ))

    return parsed


def compare(left: list, right: list) -> Literal[-1, 0, 1]:
    """Recursively compare each list for packet order

    :return: -1 if left < right
    :return: 0 if left == right
    :return: 1 if left > right
    """

    for i in range(max(len(left), len(right))):
        try:
            l = left[i]
        except IndexError:
            return -1
        try:
            r = right[i]
        except IndexError:
            return 1

        if isinstance(l, int):
            if isinstance(r, int):
                if l < r:
                    return -1
                if l > r:
                    return 1
            elif isinstance(r, list):
                val = compare([l], r)
                if val != 0:
                    return val
            else:
                raise ValueError
        elif isinstance(l, list):
            if isinstance(r, int):
                val = compare(l, [r])
                if val != 0:
                    return val
            elif isinstance(r, list):
                val = compare(l, r)
                if val != 0:
                    return val
            else:
                raise ValueError
        else:
            raise ValueError
    return 0


def part1(data: str):
    parsed = parse(data)

    correct = []
    for i, (left, right) in enumerate(parsed):
        if compare(left, right) < 0:
            correct.append(i + 1)
    return sum(correct)


class Packet:
    """
    A wrapper that allows for sorting packets
    """

    def __init__(self, val: list):
        self.val = val

    def __lt__(self, other):
        return compare(self.val, other.val) < 0

    def __eq__(self, other):
        if isinstance(other, Packet):
            return self.val == other.val
        return self.val == other

    def __str__(self):
        return str(self.val)

    def __repr__(self):
        return repr(self.val)


def part2(data: str):
    parsed = parse(data)

    all_packets = [Packet([[2]]), Packet([[6]])]
    for left, right in parsed:
        all_packets.append(Packet(left))
        all_packets.append(Packet(right))

    all_packets.sort()

    # Locate the divider packets
    a = 0
    b = 0
    for i, packet in enumerate(all_packets):
        if packet == [[2]]:
            a = i + 1
        if packet == [[6]]:
            b = i + 1

    return a * b
