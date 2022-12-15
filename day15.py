def parse(data: str):
    parsed = []

    lines = [line.strip() for line in data.splitlines() if line]

    for line in lines:
        split = line.split(':')
        sensor = split[0]
        beacon = split[1]

        sensor_split = sensor.split(',')
        sensor_x = int(sensor_split[0].split('=')[1])
        sensor_y = int(sensor_split[1].split('=')[1])

        beacon_split = beacon.split(',')
        beacon_x = int(beacon_split[0].split('=')[1])
        beacon_y = int(beacon_split[1].split('=')[1])

        radius = dist((sensor_x, sensor_y), (beacon_x, beacon_y))

        parsed.append(((sensor_x, sensor_y), (beacon_x, beacon_y), radius))

    return parsed


def dist(sensor, beacon):
    return abs(sensor[0] - beacon[0]) + abs(sensor[1] - beacon[1])


def compress_intervals(intervals):
    final_intervals = []
    for start, end in sorted(intervals):
        if not final_intervals:
            final_intervals.append((start, end))
            continue

        s, e = final_intervals[-1]
        if start <= e + 1:
            final_intervals[-1] = (s, max(e, end))
        else:
            final_intervals.append((start, end))
    return final_intervals


def solve(parsed, y_level: int):

    intervals = []
    beacons = set()

    for sensor, beacon, radius in parsed:
        if beacon[1] == y_level:
            beacons.add(beacon[0])
        dist_to_y = dist(sensor, (sensor[0], y_level))

        if radius >= dist_to_y:
            rel_dist = radius - dist_to_y
            a = sensor[0] - rel_dist
            b = sensor[0] + rel_dist
            intervals.append((min(a, b), max(a, b)))

    return compress_intervals(intervals), beacons


def part1(data: str):
    parsed = parse(data)

    impossible, known = solve(parsed, 2000000)
    # impossible, known = solve(parsed, 10)

    count = 0
    for start, end in impossible:
        count += end - start + 1
        for beacon in known:
            if start <= beacon <= end:
                count -= 1

    return count


def find_possible(sensors, max_y):

    shortest = []

    for s1, _, r1 in sensors:
        for s2, _, r2 in sensors:
            if s1 == s2:
                continue
            distance = dist(s1, s2)
            overlap = distance - (r1 + r2)
            if overlap > 0:
                shortest.append((overlap, s1, s2, r1, r2))

    to_check = []

    def check_range(start, end):
        if start < 0:
            start = 0
        if end > max_y:
            end = max_y
        to_check.append((min(start, end), max(start, end)))

    for overlap, s1, s2, r1, r2 in shortest:
        if s1[1] > s2[1]:
            # Check if no overlap vertical
            if s1[1] - r1 > s2[1] + r2:
                check_range(s2[1] + r2, s1[1] - r1)
        else:
            # Check if no overlap vertical
            if s2[1] - r2 > s1[1] + r1:
                check_range(s1[1] + r1, s2[1] - r2)

    return compress_intervals(to_check)


def part2(data: str):
    parsed = parse(data)

    max_range = 4000000
    # max_range = 20

    possible = find_possible(parsed, max_range)
    # count = 0
    # for start, end in possible:
    #     count += end - start + 1
    #
    # print(
    #     f"Reduced search space to {count}/{max_range} "
    #     f"({int(count / max_range * 100)}%)"
    # )

    def finish(x, y):
        return x * 4000000 + y

    # i = 0
    for s, e in possible:
        for y in range(s, e + 1):
            to_check, _ = solve(parsed, y)
            # Check for any gaps in the to_check intervals
            x = 0
            for s, e in to_check:
                if s > x:
                    return finish(s - 1, y)
                x = e
            if x < max_range:
                return finish(x + 1, y)

            # if i % 100000 == 0:
            #     print(f'Iter {i} ({int(i / count * 100)}%)')
            #     # print(f'Iter {i} ({int(i / max_range * 100)}%)')
            # i += 1
