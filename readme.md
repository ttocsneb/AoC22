# AoC22
My Advent of Code 2022 Solutions

Each day is stored in a separate module with the naming convention
`day{day_number}`. Each module will have two functions available: `part1` and
`part2` with one argument: `data: str`.

The file `aoc.py` manage and run the solutions. It will automatically download
inputs for you, or you can provide custom inputs by providing a file name or
directly piping it into the program with stdin. In order for the program to
download the input for you, you will need to provide a session key for
[adventofcode.com](https://adventofcode.com). This can be done by setting an
environment variable `SESSION`, or saving the key in the file `.session`.

## Usage

Running `aoc.py` without any arguments will run the latest solution available.
You can specify which day and part you want to run or you can run all
solutions. This program will try to download the inputs if they aren't
available. If you want to download inputs yourself, they should be saved in
`inputs/day{day_number}.txt`.

### Example

```
usage: aoc.py [-h] [-p PART] [-d DAY] [-f FILE] [-i] [-a]

The solutions to the problems given in the Advent of Code 2022. Each solution should be
stored in a separate module of the name `day{day_number}` with methods
`part{part_number}(data: str)`

options:
  -h, --help            show this help message and exit
  -p PART, --part PART  Part to run (default to the latest implemented)
  -d DAY, --day DAY     Day to run (defaults to the latest day available)
  -f FILE, --file FILE  Input file override
  -i, --stdin           Use stdin as the source of data
  -a, --all             Run all available solutions (-i and -f are ignored)
```

```shell
# Run the latest day
./aoc.py

# Run every solution
./aoc.py -a

# Run day 1, part 2 using myinput.txt
./aoc.py -d 1 -p 2 -f myinput.txt

# Pipe a custom input into day 1, part 1
echo "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
" | ./aoc.py -d 1 -p 1 -i
```

