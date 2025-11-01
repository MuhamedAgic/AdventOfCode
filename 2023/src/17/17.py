import sys
import pandas as pd
from copy import deepcopy


def day_17_part_1(filename):
    #input = open(filename, "r").read().strip()
    with open(filename, 'r') as f:
        lines = f.readlines()

    df_width = len(lines)
    columns = []
    for i in range(df_width):
        columns.append(1)

    df = pd.read_fwf(filename, widths=columns, header=None)
    print(df)
        
filename = "D:/git/magic/aoc2023/aoc2023/src/17/example_input.txt"
ans1 = day_17_part_1(filename)
print(f"Part one {ans1}")

# ans2 = day_eight_part_two(filename)
# print(f"Part two {ans2}")