import sys

import sys

# Read from stdin and split blocks by double newline
blocks = sys.stdin.read().split('\n\n')

# Split each block into lines and convert each line into a list of characters
blocks = [[line for line in block.splitlines()] for block in blocks]

# find the fold for mirrored lines
def find_mirrored_line_idx(lines):
    for lineno in range(len(lines)):
        if lineno > 0 and lines[lineno] == lines[lineno-1]:
            all_same = True
            for diffline in range(1, lineno):
                if lineno + diffline > len(lines) - 1:
                    break
                if lineno - diffline - 1 < 0:
                    break
                if lines[lineno + diffline] != lines[lineno - diffline - 1]:
                    all_same = False

            if all_same:
                return lineno

    return 0

def transpose_lines(lines):
    transposed_lines = []
    for col in range(len(lines[0])):
        transposed_lines.append("".join([line[col] for line in lines]))
    return transposed_lines

sum = 0
for blockid, lines in enumerate(blocks):
    sum += find_mirrored_line_idx(lines) * 100
    sum += find_mirrored_line_idx(transpose_lines(lines))

print("Part1 =", sum)