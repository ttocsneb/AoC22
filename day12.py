import numpy as np


def parse(data: str):

    lines = [line.strip() for line in data.splitlines() if line]

    heightmap = []
    start = None
    end = None

    for i, line in enumerate(lines):
        row = []
        for j, c in enumerate(line):
            if c == 'S':
                start = (i, j)
                c = 'a'
            elif c == 'E':
                end = (i, j)
                c = 'z'
            elev = ord(c) - ord('a')
            row.append(elev)
        heightmap.append(row)

    assert start is not None
    assert end is not None

    return np.array(heightmap), start, end


def solve_dynamic(hmap: np.ndarray, end: tuple[int, int]):
    costs = np.zeros(hmap.shape)
    costs[:, :] = np.Inf
    costs[end] = 0

    def get_options(pos: tuple[int, int]) -> list[tuple[int, int]]:
        options = []
        if pos[0] > 0:
            options.append((pos[0] - 1, pos[1]))
        if pos[0] < hmap.shape[0] - 1:
            options.append((pos[0] + 1, pos[1]))
        if pos[1] > 0:
            options.append((pos[0], pos[1] - 1))
        if pos[1] < hmap.shape[1] - 1:
            options.append((pos[0], pos[1] + 1))

        avail = []

        height = hmap[pos]
        for option in options:
            new_height = hmap[option]
            if height - new_height <= 1:
                avail.append(option)

        return avail

    queue = [end]
    while queue:
        item = queue.pop()
        dist = costs[item]
        for option in get_options(item):
            if costs[option] > dist + 1:
                costs[option] = dist + 1
                queue.append(option)

    return costs


def part1(data: str):
    parsed, start, end = parse(data)

    return int(solve_dynamic(parsed, end)[start])


def part2(data: str):
    parsed, _, end = parse(data)

    costs = solve_dynamic(parsed, end)

    possible = costs[parsed == 0]

    return int(np.min(possible))
