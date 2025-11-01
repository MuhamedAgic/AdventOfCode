import pandas as pd

def day_thirteen_part_one(filename):
    with open(filename, 'r') as f:
        lines = f.readlines()

    input_col_cnt = len(lines[0]) - 1
    input_col_layout = []
    for i in range(input_col_cnt):
        input_col_layout.append(1)

    input_df = pd.read_fwf(filename, widths=input_col_layout, header=None)

    print(input_df)




        
filename = "D:/git/magic/aoc2023/aoc2023/src/thirteen/example_input.txt"
ans1 = day_thirteen_part_one(filename)
print(f"Part one {ans1}")


# ans2 = day_eight_part_two(filename)
# print(f"Part two {ans2}")
