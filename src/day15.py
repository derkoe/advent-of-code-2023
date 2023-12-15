import sys

instructions = [instruction for instruction in sys.stdin.read().split(",")]

def hash(label):
    val = 0
    for char in label:
        val += ord(char)
        val *= 17
        val %= 256
    return val

# Part 1
part1 = 0
for instruction in instructions:
    part1 += hash(instruction)

print("Part1 =", part1)

# Part 2
boxes = {}
for instruction in instructions:
    if "-" in instruction:
        label = instruction[:-1]
        box_contents = boxes.get(hash(label)) 
        if box_contents != None:
            boxes[hash(label)] = [l for l in box_contents if l[0] != label]
    else:
        label, focal_length = instruction.split("=")
        box_contents = boxes.get(hash(label)) or []
        found = False
        for entry in box_contents:
            if entry[0] == label:
                found = True
                entry[1] = focal_length
                break
        if not found:
            box_contents.append([label, focal_length])
        boxes[hash(label)] = box_contents

part2 = 0
for key, value in boxes.items():
    for index, entry in enumerate(value):
        part2 += (key+1) * (index+1) * int(entry[1])

print("Part2 =", part2)