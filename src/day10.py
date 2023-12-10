import sys

# split stdin lines in list of lists
maze = [list(line) for line in sys.stdin.read().splitlines()]
maze_size = len(maze)

# find starting position, where "S" is
for line in range(len(maze)):
    for row in range(len(maze[line])):
        if maze[line][row] == 'S':
            start = (line, row)
            break

# hardcoded starting position from the data file
pos = (start[0], start[1], 'R')
maze[start[0]][start[1]] = 'L'

def next_pos(pos):
    if pos[2] == 'R':
        if maze[pos[0]][pos[1]+1] == 'J':
            return (pos[0], pos[1]+1, 'U')
        if maze[pos[0]][pos[1]+1] == '-':
            return (pos[0], pos[1]+1, 'R')
        if maze[pos[0]][pos[1]+1] == '7':
            return (pos[0], pos[1]+1, 'D')
    if pos[2] == 'U':
        if maze[pos[0]-1][pos[1]] == 'F':
            return (pos[0]-1, pos[1], 'R')
        if maze[pos[0]-1][pos[1]] == '|':
            return (pos[0]-1, pos[1], 'U')
        if maze[pos[0]-1][pos[1]] == '7':
            return (pos[0]-1, pos[1], 'L')    
    if pos[2] == 'L':
        if maze[pos[0]][pos[1]-1] == 'L':
            return (pos[0], pos[1]-1, 'U')
        if maze[pos[0]][pos[1]-1] == '-':
            return (pos[0], pos[1]-1, 'L')
        if maze[pos[0]][pos[1]-1] == 'F':
            return (pos[0], pos[1]-1, 'D')
    if pos[2] == 'D':
        if maze[pos[0]+1][pos[1]] == 'J':
            return (pos[0]+1, pos[1], 'L')
        if maze[pos[0]+1][pos[1]] == '|':
            return (pos[0]+1, pos[1], 'D')
        if maze[pos[0]+1][pos[1]] == 'L':
            return (pos[0]+1, pos[1], 'R')

    # no way to go
    raise Exception("No way to go")


# Part 1 - walk the maze until we reach "S" again
maze_visited = [[0 for x in range(maze_size)] for y in range(maze_size)]
maze_visited[pos[0]][pos[1]] = pos[2]
step_count = 1
pos = next_pos(pos)
while (pos[0], pos[1]) != start:
    maze_visited[pos[0]][pos[1]] = pos[2]
    pos = next_pos(pos)
    step_count += 1

# Part 2 - find inside/outside with Nonzero-rule
inside_count = 0
for line in range(len(maze)):
    inside = 0
    for row in range(len(maze[line])):
        if maze_visited[line][row]:
            if maze[line][row] in "|F7":
                if maze_visited[line][row] == 'U':
                    inside += 1
                elif maze_visited[line][row] == 'D':
                    inside -= 1
                elif maze[line][row] == "F":
                    if maze_visited[line][row] == 'R':
                        inside += 1
                    elif maze_visited[line][row] == 'L':
                        inside -= 1
                elif maze[line][row] == "7":
                    if maze_visited[line][row] == 'R':
                        inside -= 1
                    elif maze_visited[line][row] == 'L':
                        inside += 1
        else:
            if inside != 0:
                inside_count += 1
                maze[line][row] = 'I'


# print the maze with visited positions colored and inside marked with "I"
if True:
    for line in range(len(maze)):
        for row in range(len(maze[line])):
            if maze[line][row] == 'S':
                print('\033[93m╚\033[0m', end='')
            elif maze_visited[line][row]:
                print('\033[91m' + maze[line][row].translate(str.maketrans("-|F7LJ.", "═║╔╗╚╝ ")) + '\033[0m', end='')
            elif maze[line][row] == 'I':
                print('\033[92m' + maze[line][row] + '\033[0m', end='')
            else:
                print('\033[90m' + '.' + '\033[0m', end='')
        print()

print("Part1=", int(step_count/2))
print("Part2=", int(inside_count))