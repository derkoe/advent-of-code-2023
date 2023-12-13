import sys

import sys

# Read from stdin and split blocks by double newline
blocks = sys.stdin.read().split('\n\n')

# Split each block into lines and convert each line into a list of characters
blocks = [[line for line in block.splitlines()] for block in blocks]

def calc_diff(line1, line2):
    diff = 0
    for a, b in zip(line1, line2):
        if a != b:
            diff += 1
    return diff

# find the fold for mirrored lines
def find_mirrored_line_idx(lines, diff_allowed=0):
    for lineno in range(len(lines)):
        diff = calc_diff(lines[lineno], lines[lineno-1])
        if lineno > 0 and diff <= diff_allowed:
            for diffline in range(1, lineno):
                if lineno + diffline > len(lines) - 1:
                    break
                if lineno - diffline - 1 < 0:
                    break
                diff += calc_diff(lines[lineno + diffline], lines[lineno - diffline - 1])

            if diff == diff_allowed:
                return lineno

    return 0


def transpose_lines(lines):
    transposed_lines = []
    for col in range(len(lines[0])):
        transposed_lines.append("".join([line[col] for line in lines]))
    return transposed_lines


part1 = 0
part2 = 0
for blockid, lines in enumerate(blocks):
    part1 += find_mirrored_line_idx(lines) * 100
    part2 += find_mirrored_line_idx(lines, diff_allowed=1) * 100
    transposed = transpose_lines(lines)
    part1 += find_mirrored_line_idx(transposed)
    part2 += find_mirrored_line_idx(transposed, diff_allowed=1)

print("Part1 =", part1)
print("Part2 =", part2)
