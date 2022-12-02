

def solve(data: str, table: dict):
    return sum(
        table[inp]
        for inp in data.splitlines()
    )


def part1(data: str):
    table = {
        # Play Rock (1 point)
        'A X': 3 + 1,  # Rock  vs Rock  (tie 3 points)
        'B X': 0 + 1,  # Paper vs Rock  (los 0 points)
        'C X': 6 + 1,  # Sciss vs Rock  (win 6 points)
        # Play Paper (2 point)
        'A Y': 6 + 2,  # Rock  vs Paper (win 6 points)
        'B Y': 3 + 2,  # Paper vs Paper (tie 3 points)
        'C Y': 0 + 2,  # Sciss vs Paper (los 0 poitns)
        # Play Sciss (3 point)
        'A Z': 0 + 3,  # Rock  vs Sciss (los 0 poitns)
        'B Z': 6 + 3,  # Paper vs Sciss (win 6 points)
        'C Z': 3 + 3,  # Sciss vs Sciss (tie 3 points)
    }
    return solve(data, table)


def part2(data: str):
    table = {
        # Lose (0 points)
        'A X': 0 + 3,  # Rock  vs Sciss (3 points)
        'B X': 0 + 1,  # Paper vs Rock  (1 point)
        'C X': 0 + 2,  # Sciss vs Paper (2 points)
        # Tie (3 points)
        'A Y': 3 + 1,  # Rock  vs Rock  (1 point)
        'B Y': 3 + 2,  # Paper vs Paper (2 points)
        'C Y': 3 + 3,  # Sciss vs Sciss (3 points)
        # Win (6 points)
        'A Z': 6 + 2,  # Rock  vs Paper (2 points)
        'B Z': 6 + 3,  # Paper vs Sciss (3 points)
        'C Z': 6 + 1,  # Sciss vs Rock  (1 point)
    }
    return solve(data, table)
