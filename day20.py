from __future__ import annotations

from tqdm import tqdm


def parse(data: str):
    parsed = []

    lines = [line.strip() for line in data.splitlines() if line]

    for line in lines:
        parsed.append(int(line))

    return parsed


class Node:
    def __init__(self, val: int) -> None:
        self.val = val
        self.next: Node | None = None
        self.prev: Node | None = None

    def remove(self):
        next_node = self.next
        prev_node = self.prev
        if next_node is not None:
            next_node.prev = prev_node
        if prev_node is not None:
            prev_node.next = next_node
        self.next = None
        self.prev = None

    def insert_after(self, node: Node):
        if node.next is not None:
            node.next.prev = self
        self.next = node.next
        node.next = self
        self.prev = node

    def insert_before(self, node: Node):
        if node.prev is not None:
            node.prev.next = self
        self.prev = node.prev
        node.prev = self
        self.next = node

    def __str__(self) -> str:
        nodes = [self.val]
        temp = self.next
        it = 0
        while temp is not self:
            it += 1
            if temp is None:
                break
            nodes.append(temp.val)
            temp = temp.next
        return ' -> '.join(str(val) for val in nodes)

    def __repr__(self) -> str:
        return str(self)


def solve(nodes: list[Node]):
    size = len(nodes) - 1
    for node in nodes:
        if node.val == 0:
            continue
        if node.val > 0:
            temp = node.next
            assert temp is not None
            node.remove()
            for _ in range((node.val - 1) % size):
                assert temp.next is not None
                temp = temp.next
        else:
            temp = node.prev
            assert temp is not None
            node.remove()
            for _ in range((-node.val) % size):
                assert temp.prev is not None
                temp = temp.prev
        node.insert_after(temp)


def part1(data: str):
    parsed = parse(data)

    nodes = [Node(item) for item in parsed]

    for i, node in enumerate(nodes):
        node.prev = nodes[(i - 1) % len(nodes)]
        node.next = nodes[(i + 1) % len(nodes)]

    zero = None
    for node in nodes:
        if node.val == 0:
            zero = node
            break
    assert zero is not None

    solve(nodes)

    assert zero is not None

    vals = []
    for i in [1000, 2000, 3000]:
        temp = zero
        for _ in range(i % len(nodes)):
            temp = temp.next
            assert temp is not None
        vals.append(temp.val)

    return sum(vals)


def part2(data: str):
    parsed = parse(data)

    nodes = [Node(item * 811589153) for item in parsed]

    for i, node in enumerate(nodes):
        node.prev = nodes[(i - 1) % len(nodes)]
        node.next = nodes[(i + 1) % len(nodes)]

    zero = None
    for node in nodes:
        if node.val == 0:
            zero = node
            break
    assert zero is not None

    for _ in tqdm(range(10)):
        solve(nodes)

    assert zero is not None

    vals = []
    for i in [1000, 2000, 3000]:
        temp = zero
        for _ in range(i % len(nodes)):
            temp = temp.next
            assert temp is not None
        vals.append(temp.val)

    return sum(vals)
