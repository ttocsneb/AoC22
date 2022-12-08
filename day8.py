import numpy as np


def parse(data: str):
    output = []
    lines = [line.strip() for line in data.splitlines() if line]

    for line in lines:
        row = []
        for c in line:
            row.append(int(c))
        output.append(row)

    return np.array(output)


def check(grid: np.ndarray, pos: tuple[int, int], direct: tuple[int, int]):
    height = grid[pos]

    def is_valid():
        if pos[0] < 0:
            return False
        if pos[0] >= grid.shape[0]:
            return False
        if pos[1] < 0:
            return False
        if pos[1] >= grid.shape[1]:
            return False
        return True

    def move(p: tuple[int, int]):
        return p[0] + direct[0], p[1] + direct[1]

    num_found = 0
    pos = move(pos)
    while is_valid():
        num_found += 1
        new_height = grid[pos]
        if new_height >= height:
            return False, num_found
        pos = move(pos)
    return True, num_found


def part1(data: str):
    grid = parse(data)

    num_vis = 0

    for i in range(grid.shape[0]):
        for j in range(grid.shape[1]):
            # Check if you can see beyond the tree line in at least one cardinal
            # direction
            if check(grid, (i, j), (-1, 0))[0]:
                num_vis += 1
                continue
            if check(grid, (i, j), (1, 0))[0]:
                num_vis += 1
                continue
            if check(grid, (i, j), (0, -1))[0]:
                num_vis += 1
                continue
            if check(grid, (i, j), (0, 1))[0]:
                num_vis += 1
                continue
    return num_vis


def part2(data: str):
    grid = parse(data)

    scores = []

    for i in range(grid.shape[0]):
        for j in range(grid.shape[1]):
            # Multiply the number of visible trees in each cardinal direction
            score = 1
            score *= check(grid, (i, j), (-1, 0))[1]
            score *= check(grid, (i, j), (1, 0))[1]
            score *= check(grid, (i, j), (0, -1))[1]
            score *= check(grid, (i, j), (0, 1))[1]
            scores.append(score)

    return max(scores)
