def parse(data: str):
    parsed = set()

    lines = [line.strip() for line in data.splitlines() if line]

    for line in lines:
        parsed.add(eval(line))

    return parsed


def find_sides(dims: tuple[int, int, int]):

    new_sides = [
        (dims[0], dims[1], dims[2] - 1),
        (dims[0], dims[1], dims[2] + 1),
        (dims[0], dims[1] - 1, dims[2]),
        (dims[0], dims[1] + 1, dims[2]),
        (dims[0] - 1, dims[1], dims[2]),
        (dims[0] + 1, dims[1], dims[2]),
    ]
    return new_sides


def adjacent(points: set[tuple[int, int, int]]):

    possible_sides = list()
    for point in points:
        possible_sides.extend(find_sides(point))

    sides = [point for point in possible_sides if point not in points]

    return sides


def part1(data: str):
    parsed = parse(data)

    return len(adjacent(parsed))


def bounding_box(data: set[tuple[int, int, int]]):
    max_x = float('-inf')
    min_x = float('inf')
    max_y = float('-inf')
    min_y = float('inf')
    max_z = float('-inf')
    min_z = float('inf')

    for point in data:
        max_x = max(point[0], max_x)
        max_y = max(point[1], max_y)
        max_z = max(point[2], max_z)
        min_x = min(point[0], min_x)
        min_y = min(point[1], min_y)
        min_z = min(point[2], min_z)

    return (int(min_x), int(max_x)), (int(min_y), int(max_y)), (int(min_z), int(max_z))


def in_bouding_box(point: tuple[int, int, int],
                   bounds: tuple[tuple[int, int], tuple[int, int], tuple[int, int]]):
    if point[0] < bounds[0][0] or point[0] > bounds[0][1]:
        return False
    if point[1] < bounds[1][0] or point[1] > bounds[1][1]:
        return False
    if point[2] < bounds[2][0] or point[2] > bounds[2][1]:
        return False
    return True


def is_bound(data: set[tuple[int, int, int]], point: tuple[int, int, int],
             bound_points: set[tuple[int, int, int]],
             bounds: tuple[tuple[int, int], tuple[int, int], tuple[int, int]]):
    seen = set(bound_points)

    to_check = [point]

    while to_check:
        next_point = to_check.pop()
        if next_point in seen:
            continue
        if not in_bouding_box(next_point, bounds):
            return False, bound_points
        seen.add(next_point)
        to_check.extend(
            p for p in find_sides(next_point)
            if p not in seen and p not in data
        )

    return True, seen


def part2(data: str):
    parsed = parse(data)

    sides = adjacent(parsed)

    bounds = bounding_box(parsed)
    bound_points = set()
    new_sides = []
    for side in sides:
        bound, bound_points = is_bound(parsed, side, bound_points, bounds)
        if not bound:
            new_sides.append(side)

    return len(new_sides)
