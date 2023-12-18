import sys

def calc_area(points):
    n = len(points)
    res = 0
    for i in range(n):
        x1, y1 = points[i]
        x2, y2 = points[(i + 1) % n]
        res += x1 * y2 - x2 * y1
    return abs(res) >> 1

def calculate_lagoon_volume(instructions):
    points = []
    x = y = 0
    length_around = 0
    for instruction in instructions:
        direction, distance, _ = instruction
        if direction == 'U':
            y -= int(distance)
        elif direction == 'D':
            y += int(distance)
        elif direction == 'L':
            x -= int(distance)
        elif direction == 'R':
            x += int(distance)
        points.append((x, y))
        length_around += int(distance)

    return calc_area(points) + (length_around // 2) + 1

def calculate_lagoon_volume_p2(instructions):
    points = []
    x = y = 0
    length_around = 0
    for instruction in instructions:
        _, _, color = instruction
        distance, direction = color[:5], color[-1]
        direction = int(color[6], 16)
        distance = int(color[1:6], 16)
        if direction == 3:
            y -= int(distance)
        elif direction == 1:
            y += int(distance)
        elif direction == 2:
            x -= int(distance)
        elif direction == 0:
            x += int(distance)
        points.append((x, y))
        length_around += int(distance)

    return calc_area(points) + (length_around // 2) + 1

instructions = [line.strip().replace('(', '').replace(')', '').split(' ') for line in sys.stdin.readlines()]

print("Part1 =", calculate_lagoon_volume(instructions))
print("Part2 =", calculate_lagoon_volume_p2(instructions))
