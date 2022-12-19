from __future__ import annotations

import re


def parse(data: str):
    blueprints = []

    lines = [line.strip() for line in data.splitlines() if line]

    bot_type = re.compile(r".*Each\s+(\w+)")

    for line in lines:
        requirements = line.split('.')
        blueprint = {}
        for robot_data in requirements:
            if not robot_data:
                continue
            m = bot_type.match(robot_data)
            assert m is not None
            bot = m[1]
            robot = {}
            for elements in robot_data.split('and'):
                element = elements.strip().split(' ')
                robot[element[-1]] = int(element[-2])
            blueprint[bot] = robot
        blueprints.append(blueprint)

    return blueprints


class Mine:
    def __init__(self, costs: dict, ore: int = 0, clay: int = 0, obsidian: int = 0,
                 geode: int = 0, ore_bots: int = 0, clay_bots: int = 0,
                 obsidian_bots: int = 0, geode_bots: int = 0, iteration: int = 0):
        self.ore = ore
        self.clay = clay
        self.obsidian = obsidian
        self.geode = geode
        self.ore_bots = ore_bots
        self.clay_bots = clay_bots
        self.obsidian_bots = obsidian_bots
        self.geode_bots = geode_bots
        self.costs = costs
        self.iteration = iteration

    def clone(self):
        return self.__class__(
            costs=self.costs,
            ore=self.ore,
            clay=self.clay,
            obsidian=self.obsidian,
            geode=self.geode,
            ore_bots=self.ore_bots,
            clay_bots=self.clay_bots,
            obsidian_bots=self.obsidian_bots,
            geode_bots=self.geode_bots,
            iteration=self.iteration
        )

    def collect(self):
        self.ore += self.ore_bots
        self.clay += self.clay_bots
        self.obsidian += self.obsidian_bots
        self.geode += self.geode_bots

    def is_buildable(self, bot: str):
        needed = self.costs[bot]

        # Check availability
        for k, v in needed.items():
            if getattr(self, k) < v:
                return False
        return True

    def build_bot(self, bot: str):
        if not self.is_buildable(bot):
            return False

        needed = self.costs[bot]
        # Use costs
        for k, v in needed.items():
            setattr(self, k, getattr(self, k) - v)

        return True

    def complete_bot(self, bot: str):
        building_bot = bot + '_bots'
        setattr(self, building_bot, getattr(self, building_bot) + 1)

    def greedy(self, n: int):

        copy = self.clone()

        for _ in range(self.iteration, n):
            to_build = None
            for bot in ['geode', 'obsidian', 'clay', 'ore']:
                if copy.is_buildable(bot):
                    to_build = bot
                    break
            copy.run_iter(to_build)
        return copy.geode

    def run_iter(self, bot: str | None = None):
        self.collect()
        building_bot = None
        if bot is not None:
            if self.build_bot(bot):
                building_bot = bot
        if building_bot is not None:
            self.complete_bot(building_bot)
        self.iteration += 1

    def branches(self):
        no_build = self.clone()
        no_build.run_iter()
        branches = [no_build]
        for bot in ['geode', 'obsidian', 'clay', 'ore']:
            if self.is_buildable(bot):
                build = self.clone()
                build.run_iter(bot)
                branches.append(build)

        return branches

    def compare_a(self):
        return self.geode_bots, self.geode, self.obsidian_bots, \
            self.obsidian, self.clay_bots, self.clay, self.ore_bots, self.ore

    def compare_b(self):
        return self.geode_bots, self.obsidian_bots, self.clay_bots, \
            self.ore_bots, self.geode, self.obsidian, self.clay, self.ore

    def __str__(self) -> str:
        return ','.join(
            f'{k}={v}'
            for k, v in self.__dict__.items()
            if k not in ['costs']
        )

    def __repr__(self) -> str:
        return str(self)


def part1(data: str):
    parsed = parse(data)

    scores = []

    time = 24

    for bid, blueprint in enumerate(parsed):
        mine = Mine(blueprint, ore_bots=1)
        options = [mine]

        highest_score = 0

        for _ in range(time):
            next_options = []
            for mine in options:
                for branch in mine.branches():
                    highest_score = max(branch.geode, highest_score)
                    next_options.append(branch)
            options = sorted(
                next_options, key=lambda x: x.compare_a(), reverse=True)[:1000]

        scores.append(highest_score * (bid + 1))

    return sum(scores)


def part2(data: str):
    parsed = parse(data)

    scores = []

    time = 32

    for blueprint in parsed[:3]:
        mine = Mine(blueprint, ore_bots=1)
        options = [mine]

        highest_score = 0

        for _ in range(time):
            next_options = []
            for mine in options:
                for branch in mine.branches():
                    highest_score = max(branch.geode, highest_score)
                    next_options.append(branch)
            options = sorted(
                next_options, key=lambda x: x.compare_b(), reverse=True)[:1000]
            # options = sorted(next_options)[:1000]
        scores.append(highest_score)

    return scores[0] * scores[1] * scores[2]
