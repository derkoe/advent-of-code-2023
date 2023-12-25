import sys
import math 

class Hailstone:
    start = None
    direction = None
    
    def __init__(self, parts):
        self.start = [int(num) for num in  parts[0].split(', ')]
        self.direction = [int(num) for num in  parts[1].split(', ')]

    def __str__(self):
        return f"Hailstone(start={self.start}, direction={self.direction})"

# rays = [[[int(num) for num in parts.split(', ')] for parts in line.replace("\n", "").split(" @ ")] for line in sys.stdin.readlines()]
rays = [Hailstone(line.replace("\n", "").split(" @ ")) for line in sys.stdin.readlines()]

def ray_intersects_x_y_in_area(ray1, ray2, area):
    x1, y1 = ray1.start[0], ray1.start[1]
    dx1, dy1, dz1 = ray1.direction[0], ray1.direction[1], ray1.direction[2]
    x2, y2 = ray2.start[0], ray2.start[1]
    dx2, dy2, dz2 = ray2.direction[0], ray2.direction[1], ray2.direction[2]

    # Calculate the slopes of the two rays
    if dx1 == 0 or dx2 == 0:
        return False  # One or both rays are vertical, so they cannot intersect
    m1 = dy1 / dx1
    m2 = dy2 / dx2

    if (dx1*dx2 + dy1*dy2 + dz1*dz2) == 0:
        print("parallel", ray1, ray2)

    # If the slopes are equal, the rays are parallel and do not intersect
    if m1 == m2:
        return False

    # Calculate the x-coordinate of the intersection point
    x_intersect = (y2 - y1 + m1 * x1 - m2 * x2) / (m1 - m2)
    y_intersect = m1 * (x_intersect - x1) + y1

    inside_x = (dx1 > 0 and x1 < x_intersect or dx1 < 0 and x1 > x_intersect) and (dx2 > 0 and x2 < x_intersect or dx2 < 0 and x2 > x_intersect)
    inside_y = (dy1 > 0 and y1 < y_intersect or dy1 < 0 and y1 > y_intersect) and (dy2 > 0 and y2 < y_intersect or dy2 < 0 and y2 > y_intersect)

    if not (inside_x and inside_y):
        return False

    # If the intersection point is outside of the area, the rays do not intersect
    if x_intersect < area[0] or x_intersect > area[1] or y_intersect < area[0] or y_intersect > area[1]:
        return False

    return True


intersect_count = 0
for i, ray in enumerate(rays):
    for j in range(i+1, len(rays)):
        if ray_intersects_x_y_in_area(ray, rays[j], [200000000000000, 400000000000000]):
            intersect_count += 1

print("Part1 =", intersect_count)