import numpy as np


def parse(data: str):
    output = []

    lines = [line.strip() for line in data.splitlines() if line]

    for line in lines:
        split = line.split(' ')
        output.append((split[0], int(split[1])))

    return output


def solve(data: str, n: int):
    parsed = parse(data)

    visited = set()
    visited.add((0, 0))

    head_pos = (0, 0)
    tail_pos = [(0, 0) for _ in range(n)]

    def move(pos: tuple[int, int], direct: tuple[int, int]):
        return pos[0] + direct[0], pos[1] + direct[1]

    def is_adgacent(a: tuple[int, int], b: tuple[int, int]):
        if a == b:
            return True
        if move(a, (-1, 1)) == b:
            return True
        if move(a, (0, 1)) == b:
            return True
        if move(a, (1, 1)) == b:
            return True
        if move(a, (-1, 0)) == b:
            return True
        if move(a, (0, 0)) == b:
            return True
        if move(a, (1, 0)) == b:
            return True
        if move(a, (-1, -1)) == b:
            return True
        if move(a, (0, -1)) == b:
            return True
        if move(a, (1, -1)) == b:
            return True
        return False

    def follow_cmd(pos, cmd):
        if cmd == 'U':
            return move(pos, (0, 1))
        if cmd == 'D':
            return move(pos, (0, -1))
        if cmd == 'L':
            return move(pos, (-1, 0))
        if cmd == 'R':
            return move(pos, (1, 0))
        return pos

    for direct, dist in parsed:
        for _ in range(dist):
            pos = follow_cmd(head_pos, direct)
            last_tail = pos
            for i, tail in enumerate(tail_pos):
                if not is_adgacent(tail, last_tail):
                    if last_tail[0] == tail[0] and last_tail[1] < tail[1]:
                        tail_pos[i] = move(tail, (0, -1))
                    elif last_tail[0] == tail[0] and last_tail[1] > tail[1]:
                        tail_pos[i] = move(tail, (0, 1))
                    if last_tail[1] == tail[1] and last_tail[0] < tail[0]:
                        tail_pos[i] = move(tail, (-1, 0))
                    elif last_tail[1] == tail[1] and last_tail[0] > tail[0]:
                        tail_pos[i] = move(tail, (1, 0))
                    elif last_tail[1] < tail[1] and last_tail[0] < tail[0]:
                        tail_pos[i] = move(tail, (-1, -1))
                    elif last_tail[1] > tail[1] and last_tail[0] > tail[0]:
                        tail_pos[i] = move(tail, (1, 1))
                    elif last_tail[1] < tail[1] and last_tail[0] > tail[0]:
                        tail_pos[i] = move(tail, (1, -1))
                    elif last_tail[1] > tail[1] and last_tail[0] < tail[0]:
                        tail_pos[i] = move(tail, (-1, 1))
                    # tail_pos[i] = follow_cmd(tail, direct)
                    # tail_pos[i] = to_move
                    # if i == 8:
                    #     print(f"Tail Moved from {tail} to {tail_pos[i]}")
                last_tail = tail_pos[i]
            head_pos = pos
            visited.add(tail_pos[-1])

        # print(f'{direct} {dist}')
        # visualize(visited)
    # pprint(parsed)

    # visualize(visited)

    return len(visited)


def visualize(visited: set[tuple[int, int]]):
    min_x = 999999
    max_x = -999999
    min_y = 999999
    max_y = -999999

    for x, y in visited:
        min_x = min(x, min_x)
        min_y = min(y, min_y)
        max_x = max(x, max_x)
        max_y = max(y, max_y)

    buf = ''

    for i in range(min_x, max_x + 1):
        for j in range(min_x, max_x + 1):
            if (i, j) in visited:
                buf += '# '
            else:
                buf += '. '
        buf += '\n'
    print(buf)


def part1(data: str):
    return solve(data, 1)


def part2(data: str):
    return solve(data, 9)
