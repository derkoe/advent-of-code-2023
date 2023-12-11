import sys
from collections import deque
from itertools import combinations


# Function to expand the universe
def expand_universe(input_data):
    # duplicate rows that have no # in them
    expanded_lines = []
    for row in input_data:
        expanded_lines.append(row)
        if "#" not in row:
            expanded_lines.append(row)
    
    columns_to_expand = []
    for col in range(len(input_data[0])):
        column = [row[col] for row in input_data]
        if "#" not in column:
            columns_to_expand.append(col)

    # duplicate columns that have no # in them
    expanded = []
    for row in expanded_lines:
        new_row = []
        for col in range(len(row)):
            new_row.append(row[col])
            if col in columns_to_expand:
                new_row.append(row[col])
        expanded.append(new_row)

    return expanded

def mark_empty_rows_cols(universe):
    emtpty_rows = []
    empty_colums = []
    for row in range(len(universe)):
        if "#" not in universe[row]:
            universe[row] = ["-"] * len(universe[row])
            emtpty_rows.append(row)
    for col in range(len(universe[0])):
        column = [row[col] for row in universe]
        if "#" not in column:
            empty_colums.append(col)
            for row in range(len(universe)):
                if universe[row][col] == "-":
                    universe[row][col] = "+"
                else:
                    universe[row][col] = "|"

    return (emtpty_rows, empty_colums)

def find_galaxies(universe):
    galaxies = []
    for row in range(len(universe)):
        for col in range(len(universe[row])):
            if universe[row][col] == "#":
                galaxies.append((row, col))
    return galaxies

# manhattan distance
def manhattan_distance(universe, start, end):
    return abs(start[0] - end[0]) + abs(start[1] - end[1])

# Function to calculate sum of shortest paths
def sum_of_shortest_paths(galaxies, universe):
    total_length = 0
    for galaxy1, galaxy2 in combinations(galaxies, 2):
        path_length = manhattan_distance(universe, galaxy1, galaxy2)
        total_length += path_length
    return total_length

def parse_input(input_data):
    universe = [list(line) for line in input_data]
    return universe

# Main program
def part1(galaxy):
    (empty_rows, empty_cols) = mark_empty_rows_cols(galaxy)
    for row in galaxy:
        print("".join(row))
    galaxies = find_galaxies(galaxy)
    distance = sum_of_shortest_paths(galaxies, empty_rows, empty_cols)
    return distance

# Input data
input_data = [
    "...#......",
    ".......#..",
    "#.........",
    "..........",
    "......#...",
    ".#........",
    ".........#",
    "..........",
    ".......#..",
    "#...#....."
]

# Solve the puzzle
print("Part1=", part1(parse_input([list(line) for line in sys.stdin.read().splitlines()])))
