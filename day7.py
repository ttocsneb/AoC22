

def parse(data: str):
    tree = {}
    commands = [cmd.strip() for cmd in data.split('$') if cmd]

    def get_dir(path: list[str]):
        t = tree
        for item in path:
            t.setdefault(item, {})
            t = t[item]
        return t

    cdir = []
    for command in commands:
        lines = command.splitlines()
        cmd = lines[0]
        items = lines[1:]

        if cmd.startswith('cd'):
            folder = cmd.split(' ')[1]
            if folder == '..':
                cdir.pop()
            else:
                cdir.append(folder)
        elif cmd.startswith('ls'):
            struct = get_dir(cdir)
            for entry in items:
                split = entry.split(' ')
                if split[0] == 'dir':
                    struct[split[1]] = {}
                else:
                    struct[split[1]] = int(split[0])

    return tree


def flatten(name: str, tree: dict):
    flat = {
        name: 0
    }
    for k, v in tree.items():
        if isinstance(v, dict):
            for k, v in flatten(f'{name}/{k}', v).items():
                flat[name] += v
                flat[k] = v
        else:
            flat[name] += v
    return flat


def get_size(tree: dict):
    size = 0
    for v in tree.values():
        if isinstance(v, dict):
            s = get_size(v)
            size += s
        else:
            size += v
    return size


def part1(data: str):
    tree = parse(data)
    flat = flatten('', tree)

    total = 0
    for val in flat.values():
        if val < 100000:
            total += val

    return total


def part2(data: str):
    tree = parse(data)
    flat = flatten('', tree)

    size = 70000000
    free = 30000000
    total_free = size - get_size(tree)

    options = []
    for val in flat.values():
        if val + total_free > free:
            options.append(val)

    return min(options)
