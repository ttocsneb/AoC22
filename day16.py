import re


def parse(data: str):
    parsed = {}

    lines = [line.strip() for line in data.splitlines() if line]

    parse = re.compile(
        r"Valve (\w{2}) .* rate=(\d+);")

    for line in lines:
        groups = parse.match(line)
        print(line)
        assert groups is not None
        a = groups[1]
        b = int(groups[2])

        valves = []
        for group in line.split(','):
            valves.append(group.split(' ')[-1])

        parsed[a] = (
            b,
            valves)

    return parsed


def solve(parsed, position: str):

    best_order = sorted(parsed.items(), key=lambda x: x[1][0], reverse=True)
    print(best_order)

    def find_max_score(pos: str, open_valves, time: int, score: int):

        for valve, (pressure, _) in best_order:
            if valve not in open_valves:
                if pos == valve:
                    time = time - 1
                else:
                    time = time - 2
                if time <= 0:
                    return score
                score = score + pressure * time
        return score

    # states = [(find_max_score(position, set(), 30, 0),
    #            0, set(), position, 30, [])]
    states = [(find_max_score(position, set(), 30, 0),
               0, set(), position, 30)]
    best_score = 0
    # best_directions = []

    i = 0
    while states:
        # possible_score, score, open_valves, position, time, directions = states.pop(
        #     0)
        possible_score, score, open_valves, position, time = states.pop(
            0)

        # possible_score = find_max_score(position, open_valves, time, score)

        if time <= 0:
            continue
        if possible_score <= best_score:
            continue

        pressure, next_valves = parsed[position]
        for valve in next_valves:
            # directions_copy = list(directions)
            # directions_copy.append(f"go {valve}")
            possible_score = find_max_score(
                position, open_valves, time - 1, score)
            # states.append(
            #     (possible_score, score, open_valves, valve, time - 1, directions_copy))
            states.append(
                (possible_score, score, open_valves, valve, time - 1))

        if position not in open_valves and pressure > 0:
            # directions_copy = list(directions)
            # directions_copy.append(f"open {position}")
            open_valves_copy = set(open_valves)
            open_valves_copy.add(position)
            time -= 1
            score = score + time * pressure
            if score > best_score:
                best_score = score
                # best_directions = directions_copy
            possible_score = find_max_score(
                position, open_valves_copy, time, score)
            if possible_score >= best_score:

                # states.append(
                #     (possible_score, score, open_valves_copy, position, time, directions_copy))
                states.append(
                    (possible_score, score, open_valves_copy, position, time))

        # states.sort(reverse=True)
        states.sort(key=lambda x: (x[1], x[0]), reverse=True)
        if i % 1000 == 0:
            print(
                f'{i}: Best so far: {best_score}, items left: {len(states)}, next_item: {states[0][:5]}')
        i += 1
    # print(', '.join(best_directions))

    return best_score


def solve2(parsed, position: str):

    best_order = sorted(parsed.items(), key=lambda x: x[1][0], reverse=True)
    print(best_order)

    def find_max_score(pos: str, open_valves, time: int, score: int):
        for valve, (pressure, _) in best_order:
            if valve not in open_valves:
                if pos == valve:
                    time = time - 1
                else:
                    time = time - 2
                if time <= 0:
                    return score
                score = score + pressure * time
        return score

    # states = [(find_max_score(position, set(), 30, 0),
    #            0, set(), position, 30, [])]
    states = [(find_max_score(position, set(), 26, 0),
               0, set(), position, position, 26)]
    best_score = 0
    # best_directions = []

    i = 0
    while states:
        # possible_score, score, open_valves, position, time, directions = states.pop(
        #     0)
        possible_score, score, open_valves, pos1, pos2, time = states.pop(
            0)

        # possible_score = find_max_score(position, open_valves, time, score)

        if time <= 0:
            continue
        if possible_score <= best_score:
            continue

        pressure, next_valves = parsed[pos1]
        for valve in next_valves:
            # directions_copy = list(directions)
            # directions_copy.append(f"go {valve}")
            possible_score = find_max_score(
                position, open_valves, time - 1, score)
            # states.append(
            #     (possible_score, score, open_valves, valve, time - 1, directions_copy))
            states.append(
                (possible_score, score, open_valves, valve, time - 1))

        if position not in open_valves and pressure > 0:
            # directions_copy = list(directions)
            # directions_copy.append(f"open {position}")
            open_valves_copy = set(open_valves)
            open_valves_copy.add(pos1)
            time -= 1
            score = score + time * pressure
            if score > best_score:
                best_score = score
                # best_directions = directions_copy
            possible_score = find_max_score(
                pos1, open_valves_copy, time, score)
            if possible_score >= best_score:

                # states.append(
                #     (possible_score, score, open_valves_copy, position, time, directions_copy))
                states.append(
                    (possible_score, score, open_valves_copy, position, time))

        # states.sort(reverse=True)
        states.sort(key=lambda x: (x[1], x[0]), reverse=True)
        if i % 1000 == 0:
            print(
                f'{i}: Best so far: {best_score}, items left: {len(states)}, next_item: {states[0][:5]}')
        i += 1
    # print(', '.join(best_directions))

    return best_score


def part1(data: str):
    parsed = parse(data)

    return solve(parsed, 'AA')


def part2(data: str):
    parsed = parse(data)

    return solve2(parsed, 'AA')
