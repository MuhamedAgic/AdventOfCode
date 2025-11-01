import sys
sys.setrecursionlimit(3000000)

#************************* PART ONE ********************************
def get_differences_from(all_differences, current_depth=0):
    differences = []
    for idx, val in enumerate(all_differences[current_depth]):
        if idx > 0:
            differences.append(val - all_differences[current_depth][idx - 1])
    print(f"Processing: {all_differences[current_depth]}")
    print(f"{all_differences}\n\n")
    print(f"len all before {len(all_differences)}")
    all_differences.append(differences)
    print(f"len all after {len(all_differences)}")
    current_depth += 1

    if not all(val == 0 for val in all_differences[-1]):
        # als niet allemaal nullen, dan niet klaar
        get_differences_from(all_differences, current_depth=current_depth)
    return all_differences


def extrapolate(all_differences):
    all_differences = all_differences[::-1] # reverse begin bij 0, 0, 0...
    print(f"\nExtrapolating: {all_differences}\n\n")
    for idx, difference in enumerate(all_differences):
        print(f"Extrapolating: {difference}\n\n")
        if idx == len(all_differences) - 1:
            return 
        if idx > 0:
            list_to_append_value = all_differences[idx + 1]
            last_val_curr = difference[-1]
            last_val_next = list_to_append_value[-1]
            list_to_append_value.append(last_val_curr + last_val_next)

def get_extrapolated_value(all_differences):
    return all_differences[0][-1]

def day_nine_part_one(filename):
    with open(filename, 'r') as f:
        lines = f.readlines()

    extrapolated_vals = []
    for line in lines:
        all_differences = [[int(val) for val in line.split()]]

        get_differences_from(all_differences)
        extrapolate(all_differences)
        extrapolated_val = get_extrapolated_value(all_differences)
        extrapolated_vals.append(extrapolated_val)

    return sum(extrapolated_vals)

#************************* PART TWO ********************************

def extrapolate_v2(all_differences):
    all_differences = all_differences[::-1] # reverse begin bij 0, 0, 0...
    print(f"\nExtrapolating: {all_differences}\n\n")
    for idx, difference in enumerate(all_differences):
        print(f"Extrapolating: {difference}\n\n")
        if idx == len(all_differences) - 1:
            return 

        list_to_append_value = all_differences[idx + 1]
        first_val_curr = difference[0]
        first_val_next = list_to_append_value[0]
        # iets anders
        x = first_val_next - first_val_curr
        list_to_append_value.insert(0, x)

def get_extrapolated_value_v2(all_differences):
    return all_differences[0][0]

def day_nine_part_two(filename):
    with open(filename, 'r') as f:
        lines = f.readlines()

    extrapolated_vals = []
    for line in lines:
        all_differences = [[int(val) for val in line.split()]]

        get_differences_from(all_differences)
        extrapolate_v2(all_differences)
        extrapolated_val = get_extrapolated_value_v2(all_differences)
        extrapolated_vals.append(extrapolated_val)

    return sum(extrapolated_vals)


filename = "D:/git/magic/aoc2023/aoc2023/src/nine/input.txt"
# ans1 = day_nine_part_one](filename)
# print(f"Part one {ans1}")

ans2 = day_nine_part_two(filename)
print(f"Part two {ans2}")