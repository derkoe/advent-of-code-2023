import sys

mirror_maze = [list(line.strip()) for line in sys.stdin.readlines()]

def calc_energized(start=(0, -1, 'R')):
    visited = set()
    beams = [start]
    while beams:
        new_beams = set()
        for beam in beams:
            line, col, direction = beam
            if direction == 'R' and col + 1 < len(mirror_maze[0]):
                if mirror_maze[line][col+1] in '.-':
                    new_beams.add((line, col+1, 'R'))
                if mirror_maze[line][col+1] == '/':
                    new_beams.add((line, col+1, 'U'))
                if mirror_maze[line][col+1] == '\\':
                    new_beams.add((line, col+1, 'D'))
                if mirror_maze[line][col+1] == '|':
                    new_beams.add((line, col+1, 'U'))
                    new_beams.add((line, col+1, 'D'))
            if direction == 'L' and col - 1 >= 0:
                if mirror_maze[line][col-1] in '.-':
                    new_beams.add((line, col-1, 'L'))
                if mirror_maze[line][col-1] == '/':
                    new_beams.add((line, col-1, 'D'))
                if mirror_maze[line][col-1] == '\\':
                    new_beams.add((line, col-1, 'U'))
                if mirror_maze[line][col-1] == '|':
                    new_beams.add((line, col-1, 'U'))
                    new_beams.add((line, col-1, 'D'))
            if direction == 'U' and line - 1 >= 0:
                if mirror_maze[line-1][col] in '.|':
                    new_beams.add((line-1, col, 'U'))
                if mirror_maze[line-1][col] == '/':
                    new_beams.add((line-1, col, 'R'))
                if mirror_maze[line-1][col] == '\\':
                    new_beams.add((line-1, col, 'L'))
                if mirror_maze[line-1][col] == '-':
                    new_beams.add((line-1, col, 'L'))
                    new_beams.add((line-1, col, 'R'))
            if direction == 'D' and line + 1 < len(mirror_maze):
                if mirror_maze[line+1][col] in '.|':
                    new_beams.add((line+1, col, 'D'))
                if mirror_maze[line+1][col] == '/':
                    new_beams.add((line+1, col, 'L'))
                if mirror_maze[line+1][col] == '\\':
                    new_beams.add((line+1, col, 'R'))
                if mirror_maze[line+1][col] == '-':
                    new_beams.add((line+1, col, 'L'))
                    new_beams.add((line+1, col, 'R'))
        new_beams = new_beams - set(visited)
        visited.update(new_beams)
        beams = new_beams

    # count visited positions
    visited = set([(beam[0], beam[1]) for beam in visited])

    return len(visited)

print("Part1 =", calc_energized())

# Part 2 - brute force
max_energized = 0
for i in range(len(mirror_maze)):
    max_energized = max(max_energized, calc_energized((i, -1, 'R')))
    max_energized = max(max_energized, calc_energized((i, len(mirror_maze[i]), 'L')))
for i in range(len(mirror_maze[0])):
    max_energized = max(max_energized, calc_energized((-1, i, 'D')))
    max_energized = max(max_energized, calc_energized((len(mirror_maze), i, 'U')))

print("Part2 =", max_energized)
