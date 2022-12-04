from typing import Callable


def make_range(s: str):
    split = s.split('-')
    a = int(split[0])
    b = int(split[1])
    return a, b


def full_overlap(item: str):
    split = item.split(',')

    a, b = make_range(split[0])
    c, d = make_range(split[1])

    if a >= c and b <= d:
        return True
    if c >= a and d <= b:
        return True
    return False


def any_overlap(item: str):
    split = item.split(',')

    a, b = make_range(split[0])
    c, d = make_range(split[1])

    if c <= a <= d or c <= b <= c:
        return True
    if a <= c <= b or a <= d <= b:
        return True
    return False


def solve(data: str, f: Callable[[str], bool]):
    items = 0

    for line in data.split('\n'):
        if not line:
            continue
        if f(line):
            items += 1

    return items


def part1(data: str):
    return solve(data, full_overlap)


def part2(data: str):
    return solve(data, any_overlap)
