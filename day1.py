def counts(data: str):
    return [
        sum([
            int(i)
            for i in group.split("\n")
            if i
        ])
        for group in data.split("\n\n")
    ]


def part1(data: str):
    return max(counts(data))


def part2(data: str):
    return sum(sorted(counts(data))[-3:])
