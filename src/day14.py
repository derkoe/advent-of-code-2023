import sys

rocks = [list(line) for line in sys.stdin.read().splitlines()]

# All rocks (O) roll to the north, work column by column, "#" stops the rocks from rolling
def roll_rocks_north(rocks):
    for col in range(len(rocks[0])):
        moved = True
        while moved:
            moved = False
            for row in range(len(rocks) - 1, 0, -1):
                if rocks[row][col] == "O" and rocks[row - 1][col] == ".":
                    rocks[row][col] = "."
                    rocks[row - 1][col] = "O"
                    moved = True
    return rocks

# turn the matrix 90 degrees counter-clockwise, keep a list
def rotate_matrix(matrix):
    return [list(row) for row in zip(*matrix[::-1])]    


# Sum the weights of the rocks
# The amount of load caused by a single rounded rock (O) is equal to the number of rows from the rock to the south edge
def sum_weights(rocks):
    total_weight = 0
    for i, line in enumerate(rocks):
        total_weight += line.count("O") * (len(rocks) - i)
    return total_weight

print("Part1 =", sum_weights(roll_rocks_north(rocks)))

# Part 2
cycle_weights = []
for cycle in range(1, 300):
    for i in range(4):
        rocks = rotate_matrix(roll_rocks_north(rocks))
    weights = sum_weights(rocks)
    if cycle >= 200 and cycle < 218:
        cycle_weights.append(weights)

print(cycle_weights)
print("Part2 =", sum_weights(rocks))

# cylce in our data is 18
# cycle starts at 200 and is [84202, 84191, 84210, 84220, 84237, 84244, 84276, 84294, 84328, 84341, 84341, 84332, 84332, 84314, 84299, 84268, 84239, 84206]
# with that we can deduce that after 1000000000 iterations the value is: cycle[(1000000000-200)%18 = 8] = 84328

