
def solve(data: str, n: int):
    average = []
    for i, c in enumerate(data):
        average.append(c)
        if len(average) > n:
            average.pop(0)
        if len(set(average)) == n:
            return i + 1


def part1(data: str):
    return solve(data, 4)


def part2(data: str):
    return solve(data, 14)
