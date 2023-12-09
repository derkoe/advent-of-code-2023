import sys

lines = sys.stdin.read().splitlines()
# split line numbers separated by spaces
number_lines = [[int(x) for x in line.split(' ')] for line in lines]

# Part 1
part1 = 0
for numbers in number_lines:
    all_diffs = [numbers]
    diff = numbers
    while not all(x == 0 for x in diff):
        diff = [diff[i+1] - diff[i] for i in range(len(diff)-1)]
        all_diffs.append(diff)

    part1 += sum(line[-1] for line in all_diffs)

print("Part1=", part1)


# Part 2
part2 = 0
for numbers in number_lines:
    all_diffs = [numbers]
    diff = numbers
    while not all(x == 0 for x in diff):
        diff = [diff[i+1] - diff[i] for i in range(len(diff)-1)]
        all_diffs.append(diff)
    
    extrapolate = all_diffs[-1][0]
   
    for line in range(len(all_diffs) - 2, 0, -1):
        extrapolate = all_diffs[line][0] - extrapolate
    extrapolate = all_diffs[0][0] - extrapolate
    part2 += extrapolate

print("Part2=", part2)
