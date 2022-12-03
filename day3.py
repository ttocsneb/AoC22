
def to_char(c: str):
    char = ord(c)
    if char < ord('a'):
        return char - ord('A') + 27
    return char - ord('a') + 1


def to_chars(s: str):
    return [to_char(c) for c in s]


def contains_all(*items: set):
    if len(items) == 1:
        return items[0]
    if not items:
        return {}
    return items[0].intersection(contains_all(*items[1:]))


def part1(data: str):
    total = 0
    for sack in data.splitlines():
        l = len(sack) // 2
        total += to_char(list(contains_all(set(sack[:l]), set(sack[l:])))[0])

    return total


def part2(data: str):
    total = 0

    it = iter(data.splitlines())

    try:
        while True:
            # compare the next 3 lines
            total += to_char(list(contains_all(*[
                set(next(it))
                for _ in range(3)
            ]))[0])
    except StopIteration:
        pass

    return total
