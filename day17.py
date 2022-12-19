from typing import Any
import numpy as np
from tqdm import tqdm

shapes = [
    np.array([
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [1, 1, 1, 1]
    ]).transpose(),
    np.array([
        [0, 0, 0, 0],
        [0, 1, 0, 0],
        [1, 1, 1, 0],
        [0, 1, 0, 0],
    ]).transpose(),
    np.array([
        [0, 0, 0, 0],
        [0, 0, 1, 0],
        [0, 0, 1, 0],
        [1, 1, 1, 0],
    ]).transpose(),
    np.array([
        [1, 0, 0, 0],
        [1, 0, 0, 0],
        [1, 0, 0, 0],
        [1, 0, 0, 0],
    ]).transpose(),
    np.array([
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [1, 1, 0, 0],
        [1, 1, 0, 0],
    ]).transpose()
]


class Instructions:
    def __init__(self, instructions: str):
        self.instructions = instructions.replace('\n', '')
        self.index = 0

    def __next__(self):
        next_instruction = self.instructions[self.index]
        self.index += 1
        if self.index >= len(self.instructions):
            self.index = 0
        return next_instruction


class Board:
    def __init__(self, instructions: Instructions, max_iters: int):
        self.max_iters = max_iters
        self.board = np.zeros((7, 7))
        self.height = 0
        self.height_offset = 0
        # self.rel_height = 0
        self.item_index = 0
        self.instructions = instructions
        self.rocks = 0

        self.seen: dict[tuple[int, int, tuple], tuple[int, int]] = {}

    @property
    def actual_height(self):
        return self.height + self.height_offset

    def summery(self):
        positions = np.where(self.board == 2)
        rel_heights = [0] * 7
        for i in range(7):
            heights = np.where(positions[0] == i)[0]
            if heights.shape[0] > 0:
                rel_heights[i] = np.max(
                    (self.board.shape[1] - positions[1][heights])
                )

        return tuple(h - self.height for h in rel_heights)

    def _resize(self):
        '''Resize the board to fit a new piece'''

        # positions = np.where(self.board == 2)
        # rel_heights = [0] * 7
        # for i in range(7):
        #     heights = np.where(positions[0] == i)[0]
        #     if heights.shape[0] > 0:
        #         rel_heights[i] = np.max(
        #             self.board.shape[1] - positions[1][heights]
        #         )
        #
        # min_height = min(*rel_heights)
        #
        # if min_height > 0:
        #     # move cut the un-neaded data off
        #
        #     self.board[:, min_height:] = self.board[:, :-min_height]
        #     self.board[:, :min_height] = 0
        #
        #     self.rel_height += min_height

        # wanted_height = (self.height - self.rel_height) + 7
        wanted_height = self.height + 7
        if self.board.shape[1] < wanted_height:
            added = np.zeros((7, wanted_height - self.board.shape[1]))
            self.board = np.concatenate([added, self.board], axis=1)

    def insert_next_piece(self):
        self._resize()

        next_piece = shapes[self.item_index]
        self.item_index += 1
        if self.item_index >= len(shapes):
            self.item_index = 0

        self.board[2:2 + 4, 0:4] = next_piece

    def next_move(self):
        direction = next(self.instructions)
        to_move = np.where(self.board == 1)

        # Check if can move in the wanted direction

        if direction == '<':
            dest = (to_move[0] - 1, to_move[1])
        elif direction == '>':
            dest = (to_move[0] + 1, to_move[1])
        else:
            raise NotImplemented("input must be `<` or `>`")

        if np.any(dest[0] >= 7) or np.any(dest[0] < 0):
            # Make sure stays in bounds
            return False

        if np.any(self.board[dest] > 1):
            # Make sure stays doesn't collide with anything
            return False

        self.board[to_move] = 0
        self.board[dest] = 1
        return True

    def next_down(self):
        to_move = np.where(self.board == 1)

        dest = (to_move[0], to_move[1] + 1)

        if np.any(dest[1] >= self.board.shape[1]) or np.any(self.board[dest] > 1):
            # Make sure stays in bounds or doesn't collide with anything

            height = int(self.board.shape[1] - np.min(to_move[1]))
            # self.height = max(height + self.rel_height, self.height)
            self.height = max(height, self.height)

            self.board[to_move] = 2
            return False

        self.board[to_move] = 0
        self.board[dest] = 1
        return True

    def drop_item(self):
        self.insert_next_piece()
        while True:
            self.next_move()
            if not self.next_down():
                key = (
                    self.instructions.index, self.item_index, self.summery()
                )
                if seen := self.seen.get(key):
                    l_r, l_h = seen
                    remainder = self.max_iters - self.rocks
                    repitition = remainder // (self.rocks - l_r)
                    self.height_offset = repitition * (self.height - l_h)
                    self.rocks += repitition * (self.rocks - l_r)
                    self.seen = {}
                    # print("I've seen this all before")
                self.seen[key] = (self.rocks, self.height)
                break

        self.rocks += 1
        return self.actual_height

    def run(self):
        while self.rocks < self.max_iters:
            self.drop_item()
        return self.actual_height

    def __str__(self):
        buffer = []

        def to_char(c: int):
            if c == 0:
                return '.'
            if c == 1:
                return '@'
            if c == 2:
                return '#'
            return '?'

        for row in self.board.transpose():
            buffer.append(''.join(to_char(c) for c in row))

        return '\n'.join(buffer)

    def __repr__(self) -> str:
        return str(self)


def part1(data: str):
    instructions = Instructions(data)
    board = Board(instructions, max_iters=2022)

    height = board.run()
    return height


def part2(data: str):
    instructions = Instructions(data)
    board = Board(instructions, max_iters=1000000000000)

    height = board.run()
    return height
