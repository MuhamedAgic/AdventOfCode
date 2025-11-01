import sys
import pandas as pd
from copy import deepcopy

def get_trench_coorinates(operations):
    trench_coorinates = []

    start_coord = [0, 0]
    trench_coorinates.append(start_coord)
    print(f"BIJ START {trench_coorinates}")
    current_coordinate = [0, 0]
    for operation in operations:
        if operation[0] == "L":
            for i in range(operation[1]):
                current_coordinate[0] -= 1
                coord = deepcopy(current_coordinate)
                #print(f"{operation[0]}: {coord}")
                trench_coorinates.append(coord)
        elif operation[0] == "R":
            for i in range(operation[1]):
                current_coordinate[0] += 1
                coord = deepcopy(current_coordinate)
                #print(f"{operation[0]}: {coord}")
                trench_coorinates.append(coord)
        elif operation[0] == "U":
            for i in range(operation[1]):
                current_coordinate[1] += 1
                coord = deepcopy(current_coordinate)
                #print(f"{operation[0]}: {coord}")
                trench_coorinates.append(coord)
        elif operation[0] == "D":
            for i in range(operation[1]):
                current_coordinate[1] -= 1
                coord = deepcopy(current_coordinate)
                #print(f"{operation[0]}: {coord}")
                trench_coorinates.append(coord)

    # normalize, min value = 0
    min_x = min(x for x, _ in trench_coorinates)
    min_y = min(y for _, y in trench_coorinates)

    print(f"Min xy {min_x}, {min_y}")

    # alleen als kleiner dan 0
    if min_x > 0: min_x = 0
    if min_y > 0: min_y = 0

    normalized_trench_coorinates = [[x + abs(min_x), y + abs(min_y)] for x, y in trench_coorinates]
    return normalized_trench_coorinates

def calc_amount_trenches(coordinates):
    min_x, max_x = min(x for x, _ in coordinates), max(x for x, _ in coordinates)
    min_y, max_y = min(y for _, y in coordinates), max(y for _, y in coordinates)

    total_trenches = 0
    for i in range(max_y + 1):
        trenches_curr_row = [[x, y] for x, y in coordinates if y == i]
        if not trenches_curr_row:
            continue
        print(f"Curr row {i}: {trenches_curr_row}")
        # do max - min + 1: ...##....#... -> ...#######...
        trench_cnt_curr_row = max(x for x, _ in trenches_curr_row) - min(x for x, _ in trenches_curr_row) + 1
        total_trenches += trench_cnt_curr_row
    return total_trenches

def day_18_part_1(filename):
    #input = open(filename, "r").read().strip()
    with open(filename, 'r') as f:
        lines = f.readlines()

    operations = []
    for line in lines:
        dir = line.split()[0]
        steps = int(line.split()[1].split()[0])
        operations.append((dir, steps))

    #print(operations)
    coords = get_trench_coorinates(operations)
    print(coords)
    amount_trenches = calc_amount_trenches(coords)
    return amount_trenches
    # begin top left


        
filename = "D:/git/magic/aoc2023/aoc2023/src/18/input.txt"
ans1 = day_18_part_1(filename)
print(f"Part one {ans1}")

# 99731 te hoog

# ans2 = day_18_part_1(filename)
# print(f"Part two {ans2}")