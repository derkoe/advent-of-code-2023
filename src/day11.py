import sys
from collections import deque
from itertools import combinations

def empty_rows_cols(universe):
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
def manhattan_distance_with_expansions(start, end, empty_rows, empty_cols, factor):
    manhattan = abs(start[0] - end[0]) + abs(start[1] - end[1])
    for row in empty_rows:
        if min(start[0], end[0]) < row < max(start[0], end[0]):
            manhattan += factor - 1
    for col in empty_cols:
        if min(start[1], end[1]) < col < max(start[1], end[1]):
            manhattan += factor - 1
    return manhattan

# Function to calculate sum of shortest paths
def sum_of_shortest_paths(galaxies, empty_rows, empty_cols, factor):
    total_length = 0
    for galaxy1, galaxy2 in combinations(galaxies, 2):
        total_length += manhattan_distance_with_expansions(galaxy1, galaxy2, empty_rows, empty_cols, factor)
    return total_length

def parse_input(input_data):
    universe = [list(line) for line in input_data]
    return universe

def solve(lines, factor):
    galaxy = parse_input([list(line) for line in lines])
    (empty_rows, empty_cols) = empty_rows_cols(galaxy)
    if factor == 2:
        for row in galaxy:
            print("".join(row))
    galaxies = find_galaxies(galaxy)
    distance = sum_of_shortest_paths(galaxies, empty_rows, empty_cols, factor)
    return distance

# Solve the puzzle
lines = sys.stdin.read().splitlines()
print("Part1 =", solve(lines, 2))
print("Part2 =", solve(lines, 1000000))
