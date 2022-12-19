from __future__ import annotations

from heapq import heappush, heappop
import re


def parse(data: str):
    parsed = {}

    lines = [line.strip() for line in data.splitlines() if line]

    parse = re.compile(
        r"Valve (\w{2}) .* rate=(\d+);")

    for line in lines:
        groups = parse.match(line)
        assert groups is not None
        a = groups[1]
        b = int(groups[2])

        valves = {}
        for group in line.split(','):
            valve = group.split(' ')[-1]
            valves[valve] = 1

        parsed[a] = (
            b,
            valves)

    return parsed


def reduce_tree(tree: dict[str, tuple[int, dict[str, int]]], start: str):
    to_remove = []
    for valve, (pressure, next_valves) in tree.items():
        if pressure != 0 or valve == start:
            continue
        to_remove.append(valve)

        for a, a_dist in next_valves.items():
            # Remove Any references to this valve
            a_next_valves = tree[a][1]
            del a_next_valves[valve]
            for b, b_dist in next_valves.items():
                if a == b:
                    continue
                b_next_valves = tree[b][1]

                total_dist = a_dist + b_dist
                if a_next_valves.get(b, float('inf')) > total_dist:
                    a_next_valves[b] = total_dist
                if b_next_valves.get(a, float('inf')) > total_dist:
                    b_next_valves[a] = total_dist

    for valve in to_remove:
        del tree[valve]


def print_tree(tree: dict[str, tuple[int, dict[str, int]]]):
    for valve, (pressure, next_valves) in tree.items():
        formatted_valves = ', '.join(
            f'{v}={d}' for v, d in next_valves.items()
        )
        print(
            f'Valve {valve} has flow rate={pressure}; tunnels lead to valves {formatted_valves}'
        )


class Map:
    def __init__(self, layout: dict[str, tuple[int, dict[str, int]]],
                 agents: list[tuple[str, int, bool]], time: int,
                 open_valves: set[str] | None = None, score: int = 0,
                 best_order: list | None = None, previous: Map | None = None) -> None:
        self.layout = layout
        self.open_valves: set[str] = set(open_valves) if open_valves else set()
        self.best_order = best_order or sorted(
            layout.items(), key=lambda x: x[1][0], reverse=True
        )
        self.agents = list(agents)
        self.time = time
        self.score = score
        self.previous = previous
        self._theoretical_best_score = None

    def clone(self):
        return self.__class__(self.layout, self.agents, self.time, self.open_valves)

    def _agent_moves(self, agent: int) -> list[tuple[str, int, bool]]:
        cur_pos, dist, open_valve = self.agents[agent]
        if dist > 0:
            return [(cur_pos, dist, open_valve)]
        moves = []
        for next_valve, dist in self.layout[cur_pos][1].items():
            moves.append((next_valve, dist, False))
        if cur_pos not in self.open_valves:
            moves.append((cur_pos, 1, True))
        return moves

    def _all_agent_moves(self, cur_agent: int = 0) -> list[list[tuple[str, int, bool]]]:
        agent_moves = self._agent_moves(cur_agent)
        if len(self.agents) == cur_agent + 1:
            cur_agent_moves = []
            for move in agent_moves:
                cur_agent_moves.append([move])
            return cur_agent_moves
        other_agent_moves = self._all_agent_moves(cur_agent + 1)

        cur_agent_moves = []
        for move in agent_moves:
            for other_move in other_agent_moves:
                cur_agent_moves.append([move, *other_move])
        return cur_agent_moves

    @property
    def theoretical_best_score(self):
        if self._theoretical_best_score is not None:
            return self._theoretical_best_score
        score = self.score
        time = self.time
        steps_left = len(self.agents)
        for valve, (pressure, _) in self.best_order:
            if pressure == 0:
                continue
            if steps_left == 0:
                steps_left = len(self.agents)
                time -= 2
            if time <= 1:
                break
            if valve not in self.open_valves:
                score += pressure * (time - 1)
                steps_left -= 1
        self._theoretical_best_score = score
        return score

    @property
    def is_done(self):
        return self.score == self.theoretical_best_score

    def possible_moves(self) -> list[list[tuple[str, int, bool]]]:
        possible_moves = []
        for moves in self._all_agent_moves():
            # Prune any invalid states
            valid = True
            opening = set()
            for move, _, open_valve in moves:
                # This move is to open a valve
                if open_valve:
                    # Don't try to open a jammed valve
                    if self.layout[move][0] == 0:
                        valid = False
                        break
                    # Don't try to open a valve some else is trying to open
                    if move in opening:
                        valid = False
                        break
                    opening.add(move)
            if not valid:
                continue
            possible_moves.append(moves)
        return possible_moves

    def branches(self) -> list[Map]:
        new_states = []
        for moves in self.possible_moves():
            time_travel = float('inf')
            for (_, dist, _) in moves:
                time_travel = min(time_travel, dist)
            time_travel = int(time_travel)
            time = self.time - time_travel
            score = self.score
            open_valves = set(self.open_valves)
            updated_moves = []
            for move, dist, open_valve in moves:
                if open_valve:
                    score += self.layout[move][0] * (self.time - 1)
                    open_valves.add(move)

                updated_moves.append((move, dist - time_travel, open_valve))

            new_state = self.__class__(
                self.layout, updated_moves, time, open_valves, score, self.best_order, self
            )
            new_states.append(new_state)
        return new_states

    def path(self) -> list[str]:
        buffer = [
            agent
            for (agent, _, _) in self.agents
        ]
        if self.previous is not None:
            for i, path in enumerate(self.previous.path()):
                if self.agents[i][1] == 0:
                    buffer[i] = path + ' ' + buffer[i]
                else:
                    buffer[i] = f'{path} {buffer[i]}({self.agents[i][1]})'

        return buffer

    def __lt__(self, other: Map):
        if len(self.open_valves) > len(other.open_valves):
            return True
        elif len(self.open_valves) < len(other.open_valves):
            return False
        if self.time > other.time:
            return True
        elif self.time < other.time:
            return False
        if self.score > other.score:
            return True
        if self.score < other.score:
            return False
        return self.theoretical_best_score > other.theoretical_best_score
        # if self.theoretical_best_score > other.theoretical_best_score:
        #     return True
        # if self.theoretical_best_score < other.theoretical_best_score:
        #     return False
        # return self.score > other.score

    def __str__(self) -> str:
        path = '\n  '.join(self.path())
        return f'{self.score} points ({self.time} left)\n  {path}'

    def __repr__(self) -> str:
        return str(self)


def solve_many(layout, start: list[str], time: int):
    starts = [(s, 0, False) for s in start]
    states = [Map(layout, starts, time)]

    bssf = 0
    solution = None

    try:
        i = 0
        while states:
            state = heappop(states)

            if state.theoretical_best_score < bssf:
                continue

            for next_state in state.branches():

                if next_state.theoretical_best_score <= bssf:
                    continue
                if next_state.score > bssf:
                    bssf = next_state.score
                    solution = next_state
                    print(
                        f'{i}: bssf={bssf}, to_check={len(states)}, bssf={solution}'
                    )

                if next_state.is_done:
                    continue

                heappush(states, next_state)
            i += 1
            if i % 10000 == 0:
                print(f'{i}: bssf={bssf}, to_check={len(states)}')
    except KeyboardInterrupt:
        pass

    return solution


def part1(data: str):
    parsed = parse(data)
    reduce_tree(parsed, 'AA')

    solution = solve_many(parsed, ['AA'], 30)
    print(solution)

    return solution.score if solution is not None else 0


def part2(data: str):
    parsed = parse(data)
    reduce_tree(parsed, 'AA')

    solution = solve_many(parsed, ['AA', 'AA'], 26)
    print(solution)

    return solution.score if solution is not None else 0
