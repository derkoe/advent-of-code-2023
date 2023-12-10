import sys

# split stdin lines in list of lists
maze = [list(line) for line in sys.stdin.read().splitlines()]
maze_size = len(maze)

# find starting position, where "S" is
for y in range(len(maze)):
    for x in range(len(maze[y])):
        if maze[y][x] == 'S':
            start = (y, x)
            break

pos = (start[0], start[1]+1, 'R')

def next_pos(pos):
    if pos[2] == 'R':
        # up
        if maze[pos[0]][pos[1]] == 'J':
            return (pos[0]-1, pos[1], 'U')
        # right
        if maze[pos[0]][pos[1]] == '-':
            return (pos[0], pos[1]+1, 'R')
        # down
        if maze[pos[0]][pos[1]] == '7':
            return (pos[0]+1, pos[1], 'D')
    if pos[2] == 'U':
        # right
        if maze[pos[0]][pos[1]] == 'F':
            return (pos[0], pos[1]+1, 'R')
        # up
        if maze[pos[0]][pos[1]] == '|':
            # maze[pos[0]][pos[1]] = '↑'
            return (pos[0]-1, pos[1], 'U')
        # left
        if maze[pos[0]][pos[1]] == '7':
            return (pos[0], pos[1]-1, 'L')
    if pos[2] == 'L':
        # up
        if maze[pos[0]][pos[1]] == 'L':
            return (pos[0]-1, pos[1], 'U')
        # left
        if maze[pos[0]][pos[1]] == '-':
            return (pos[0], pos[1]-1, 'L')
        # down
        if maze[pos[0]][pos[1]] == 'F':
            return (pos[0]+1, pos[1], 'D')
    if pos[2] == 'D':
        # left
        if maze[pos[0]][pos[1]] == 'J':
            return (pos[0], pos[1]-1, 'L')
        # down
        if maze[pos[0]][pos[1]] == '|':
            return (pos[0]+1, pos[1], 'D')
        # right
        if maze[pos[0]][pos[1]] == 'L':
            return (pos[0], pos[1]+1, 'R')
    
    # no way to go
    raise Exception("No way to go")


# walk the maze until we reach "S" again
maze_visited = [[0 for x in range(maze_size)] for y in range(maze_size)]
maze_visited[pos[0]][pos[1]] = 1
pos = next_pos(pos)
maze_visited[pos[0]][pos[1]] = 1
step_count = 1

while maze[pos[0]][pos[1]] != 'S':
    maze_visited[pos[0]][pos[1]] = 1
    pos = next_pos(pos)
    step_count += 1

# last step back to "S"
step_count += 1

# print the maze with visited positions colored
for y in range(len(maze)):
    for x in range(len(maze[y])):
        if maze_visited[y][x] == 1:
            # replace tiles with pipe ascii symbols
            # if maze[y][x] == '-':
            #     maze[y][x] = '─'
            # if maze[y][x] == '|':
            #     maze[y][x] = '│'
            # if maze[y][x] == 'F':
            #     maze[y][x] = '┌'
            # if maze[y][x] == 'L':
            #     maze[y][x] = '└'
            # if maze[y][x] == 'J':
            #     maze[y][x] = '┘'
            # if maze[y][x] == '7':
            #     maze[y][x] = '┐'
            # maze[y][x] = maze[y][x].maketrans("-|F7LJ.", "─│┌┐└┘ ")

            print('\033[91m' + maze[y][x] + '\033[0m', end='')
        elif maze[y][x] == '.':
            # print dot in green
            print('\033[92m0\033[0m', end='')
        else:
            print(maze[y][x], end='')
    print()

print("Part1=", int(step_count/2))