#!/usr/bin/env python

import os
import argparse
import importlib
import sys

import requests
import time

from pathlib import Path

dirname = Path(__file__).parent


def download_input(day: int, dest: os.PathLike):
    """
    Dowload the input for the given day

    This will make a request to the advent of code servers to get your input file.

    Note: You need to set your session cookie value in the environment variable 
    `SESSION` or in the file `.session`

    :param day: day to download
    :param dest: location to save the download to
    """

    session = os.environ.get('SESSION')
    if session is None:
        try:
            with open(".session") as f:
                session = f.read().strip()
        except FileNotFoundError:
            raise RuntimeError(
                "A session cookie is required to download input, set it as an "
                "environ variable `SESSION` or save it in `.session`"
            )

    response = requests.get(
        f"https://adventofcode.com/2022/day/{day}/input",
        cookies={
            'session': session
        },
        stream=True
    )

    if response.status_code == 404:
        raise RuntimeError(
            f"Day {day} input not available yet")
    if response.status_code != 200:
        raise RuntimeError(f"Could not download input: {response.reason}")

    if not os.path.exists(os.path.dirname(dest)):
        os.makedirs(os.path.dirname(dest))

    with open(dest, 'wb') as f:
        for line in response.iter_lines():
            f.write(line)
            f.write(b'\n')
    response.close()


def format_time(duration: float):
    """
    Format a duration in nanoseconds

    :param duration: nanoseconds

    :return: a human readable time
    """
    if duration < 1000:
        return f"{duration}ns"
    duration /= 1000
    if duration < 1000:
        return f"{round(duration, 1)}μs"
    duration /= 1000
    if duration < 1000:
        return f"{round(duration, 1)}ms"
    duration /= 1000
    return f"{round(duration, 1)}s"


def main(part: int | None = None, day: int | None = None,
         stdin: bool = False, file: str | None = None):
    """
    Find the solution to an AoC problem

    Each solution should be stored in the module `day{day_number}` and have 
    two methods: `part1(data: str)` and `part2(data: str)`.

    By default, the latest day and part is auto-detected.

    The default input file used is `inputs/day{day_number}.txt`

    If you mark stdin as True, the data will come from stdin, this trumps the 
    file override

    :param part: run solution for part 1 or 2 (optional)
    :param day: run solution for the given day's problem (optional)
    :param stdin: whether or not to use stdin for input (default: False)
    :param file: input file override (optional)

    :return: solution to the problem
    """

    if day is None:
        # Auto-detect the latest implemented day
        day = 0
        for item in os.listdir(dirname):
            if "day" not in item:
                continue
            item = os.path.splitext(item)[0]
            try:
                day = max(day, int(item.split("day")[-1]))
            except ValueError:
                pass
    name = f"day{day}"

    # Load the selected solver
    try:
        module = importlib.import_module(name)
    except ModuleNotFoundError:
        raise RuntimeError(f"Day {day} doesn't exist yet")

    if part is None:
        # Auto-detect the latest implemented part
        latest = 0
        for k in module.__dict__:
            if k.startswith("part"):
                try:
                    latest = max(latest, int(k.split("part")[-1]))
                except ValueError:
                    pass
        if latest != 0:
            part = latest
            solver = getattr(module, f"part{latest}")
        else:
            raise RuntimeError(f"Day {day} Part 1 not yet implemented")
    else:
        try:
            solver = getattr(module, f"part{part}")
        except AttributeError:
            raise RuntimeError(f"Day {day} Part {part} not yet implemented")

    # Load the requested data
    if stdin:
        data = sys.stdin.read()
    else:
        path = dirname / "inputs" / f"{name}.txt"
        if file is None:
            if not path.exists():
                print(f"Downloading input for day {day}")
                download_input(day, path)
        with open(file or path) as f:
            data = f.read()

    print(f"Day {day} Part {part}:")
    start = time.time_ns()
    result = solver(data)
    end = time.time_ns()
    print(f"took {format_time(end - start)}")

    return result


if __name__ == '__main__':
    name = sys.argv[0]
    parser = argparse.ArgumentParser(
        description="The solutions to the problems given in the Advent of Code 2022."
        " Each solution should be stored in a separate module of the name "
        "`day{day_number}` with methods `part{part_number}(data: str)`",
    )

    parser.add_argument(
        "-p", "--part", action="store", type=int,
        help="Part to run (default to the latest implemented)"
    )

    parser.add_argument(
        "-d", "--day", action="store", type=int, default=None,
        help="Day to run (defaults to the latest day available)"
    )

    parser.add_argument(
        "-f", "--file", action="store",
        help="Input file override"
    )

    parser.add_argument(
        "-i", "--stdin", action="store_true", default=False,
        help="Use stdin as the source of data"
    )

    parser.add_argument(
        "-a", "--all", action="store_true", default=False,
        help="Run all available solutions (-i and -f are ignored)"
    )

    args = parser.parse_args()

    try:
        if args.all:
            for i in range(25):
                print(main(part=1, day=i + 1))
                print(main(part=2, day=i + 1))
        else:
            print(main(
                part=args.part, day=args.day,
                file=args.file, stdin=args.stdin
            ))
    except RuntimeError as e:
        print(e)
    except FileNotFoundError as e:
        print(e)
