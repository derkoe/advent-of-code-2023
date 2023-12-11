import sys
from functools import cache

# spring rows are in the form "???#...#.?#??#?#?"
# . is ok, # is damaged, ? is unknown
# groups are in the form "2,3,4,5"
# contiguous group of damaged springs is listed in the order those groups appear in the row
# this function counts the possible arrangements of springs that could have led to the given row
@cache
def count_arrangements(springs, groups):

    if not springs:
        return len(groups) == 0

    if not groups:
        return "#" not in springs

    count = 0

    if springs[0] in ".?":
        count += count_arrangements(springs[1:], groups)

    if groups[0] <= len(springs) and "." not in springs[: groups[0]]:
        if groups[0] == len(springs) or springs[groups[0]] != "#":
            count += count_arrangements(springs[groups[0] + 1 :], groups[1:])

    return count


total_p1 = 0
total_p2 = 0
for line in sys.stdin:
    springs, groups = line.split()
    springs = springs
    groups = tuple(map(int, groups.split(',')))
    total_p1 += count_arrangements(springs, groups)
    total_p2 += count_arrangements("?".join([springs] * 5), groups*5)

print("Part1 =", total_p1)
print("Part2 =", total_p2)