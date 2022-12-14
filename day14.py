import numpy as np


def relative_pos(pos: tuple[int, int], rel: tuple[int, int]):
    return pos[0] - rel[0], pos[1] - rel[1]


def parse(data: str):
    parsed = []

    lines = [line.strip() for line in data.splitlines() if line]

    max_x = 500
    min_x = 500
    max_y = 0
    min_y = 0

    for line in lines:
        paths = []
        for element in line.split('->'):
            split = element.strip().split(',')
            x = int(split[0])
            y = int(split[1])
            max_x = max(x, max_x)
            min_x = min(x, min_x)
            max_y = max(y, max_y)
            min_y = min(y, min_y)
            paths.append((y, x))

        parsed.append(paths)

    max_y += 1
    min_x = min(500 - max_y, min_x)
    max_x = max(500 + max_y, max_x)

    h = int(max_y - min_y) + 1
    w = int(max_x - min_x) + 1

    grid = np.zeros((h, w))

    for path in parsed:
        for i, point in enumerate(path[:-1]):
            a = relative_pos(point, (min_y, min_x))
            b = relative_pos(path[i + 1], (min_y, min_x))

            while True:
                grid[a] = 1
                if a == b:
                    break
                if a[0] < b[0]:
                    a = (a[0] + 1, a[1])
                elif a[0] > b[0]:
                    a = (a[0] - 1, a[1])
                if a[1] < b[1]:
                    a = (a[0], a[1] + 1)
                elif a[1] > b[1]:
                    a = (a[0], a[1] - 1)

    return grid, (min_y, min_x)


def print_grid(grid: np.ndarray):

    def get_char(c: int):
        if c == 0:
            return ' '
        if c == 1:
            return '#'
        if c == 2:
            return 'o'
        return '?'

    buffer = []
    for row in grid:
        buffer.append(''.join(get_char(c) for c in row))
    print('\n'.join(buffer))


def in_bounds(grid: np.ndarray, point: tuple[int, int]):
    if 0 > point[0] or grid.shape[0] <= point[0]:
        return False
    if 0 > point[1] or grid.shape[1] <= point[1]:
        return False
    return True


def step(grid: np.ndarray, rel: tuple[int, int], infinite=False):
    def next_move(pos: tuple[int, int]):
        if grid[pos] != 0:
            return None
        down = (pos[0] + 1, pos[1])
        if infinite and not in_bounds(grid, down):
            return None
        if not in_bounds(grid, down) or grid[down] == 0:
            return down

        left = (pos[0] + 1, pos[1] - 1)
        if not in_bounds(grid, left) or grid[left] == 0:
            return left

        right = (pos[0] + 1, pos[1] + 1)
        if not in_bounds(grid, right) or grid[right] == 0:
            return right

        return None

    starting_pos = relative_pos((0, 500), rel)
    pos = starting_pos
    while True:
        next_pos = next_move(pos)
        if next_pos is not None:
            if not in_bounds(grid, next_pos):
                return False
            pos = next_pos
            continue
        break

    grid[pos] = 2
    if pos == starting_pos:
        return False
    return True


def solve(grid, rel, infinite=False):
    i = 1
    while step(grid, rel, infinite):
        i += 1
    # print_grid(grid)
    return grid


def part1(data: str):
    grid, offset = parse(data)
    solve(grid, offset)

    return len(grid[grid == 2])


def part2(data: str):
    grid, offset = parse(data)
    solve(grid, offset, True)

    return len(grid[grid == 2])
