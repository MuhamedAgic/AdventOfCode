import os

def is_adjacent(line, start_idx, number_length):
    min_bound = start_idx - 1
    max_bound = start_idx + number_length
    for idx, c in enumerate(line):
        if c != '.' and not c.isdigit():
            if idx >= min_bound and idx <= max_bound:
                return True
    return False

def is_symbol_around(lines, line_cnt, line_nr, start_idx, number_str):
    number_length = len(number_str)

    if line_nr == 0:
        current_line = lines[line_nr]
        line_below = lines[line_nr + 1]
        return is_adjacent(current_line, start_idx, number_length) or is_adjacent(line_below, start_idx, number_length)
    elif line_nr == line_cnt - 1:
        line_above = lines[line_nr - 1]
        current_line = lines[line_nr]
        return is_adjacent(line_above, start_idx, number_length) or is_adjacent(current_line, start_idx, number_length)
    elif line_nr > 0 and line_nr < line_cnt:
        line_above = lines[line_nr - 1]
        current_line = lines[line_nr]
        line_below = lines[line_nr + 1]
        return is_adjacent(line_below, start_idx, number_length) or is_adjacent(current_line, start_idx, number_length) or is_adjacent(line_above, start_idx, number_length)
    else:
        print("Invalid linenr {}".format(line_nr))
        return False

def find_part_numbers_w_idx(filename):
    with open(filename, 'r') as f:
        lines = f.readlines()

    line_count = len(lines)

    result = []
    current_number_str = ""

    current_line_nr = 0

    last_idx = 0

    line_cntr = 0

    result_sum = 0

    for line in lines:
        line_cntr += 1
        result_line = []
        for idx, c in enumerate(line):
            if c.isdigit():
                current_number_str += c
            else:
                if current_number_str:
                    current_number = int(current_number_str)

                    start_num_idx = idx - len(current_number_str)

                    if is_symbol_around(lines, line_count, current_line_nr, start_num_idx, current_number_str):
                        result.append((current_number, start_num_idx))
                        result_line.append((current_number, start_num_idx))
                        result_sum += current_number
                    current_number_str = ""
            last_idx = idx

        print("Line {} {}\n".format(line_cntr, result_line))
        current_line_nr += 1

    print("{}".format(result_sum))
    return result

def day_three_part_one(filename):
    output = find_part_numbers_w_idx(filename)
    answer = sum([value for value, _ in output])
    return answer

filename = "D:/git/magic/aoc2023/aoc2023/src/three/input.txt"
day_three_part_one(filename)

# in rust: 560570, fout
# in python: 561307, fout
# met dezelfde code......

def is_special_char(char):
    #print("checking: ", char)
    return ((char != '.') and (not char.isdigit()))

# ff iq0
def has_sticky_symbol(lines, current_line_nr, char_idx):

    if current_line_nr == 0:
        current_line = lines[current_line_nr]
        line_below = lines[current_line_nr + 1]

        #print(current_line)
        #print(line_below)

        if char_idx == 0:
            #kijk rechts, rechts beneden, onder
            # 777..
            # .....
            # .....
            if (is_special_char(current_line[char_idx + 1])
            or is_special_char(line_below[char_idx])
            or is_special_char(line_below[char_idx + 1])):
                return True

        elif char_idx == (len(current_line) - 1):
            if (is_special_char(current_line[char_idx - 1])
            or is_special_char(line_below[char_idx] )
            or is_special_char(line_below[char_idx - 1] )):
                return True

        elif char_idx > 0 and char_idx < (len(current_line) - 1):
            if (is_special_char(current_line[char_idx - 1] )
            or is_special_char(current_line[char_idx + 1])
            or is_special_char(line_below[char_idx - 1] )
            or is_special_char(line_below[char_idx] )
            or is_special_char(line_below[char_idx + 1] )):
                return True
        
        # anders niks
        return False

    elif current_line_nr == (len(lines) - 1):
        line_above = lines[current_line_nr - 1]
        current_line = lines[current_line_nr]
        
        if char_idx == 0:
            # .....
            # .....
            # 777..
            if (is_special_char(current_line[char_idx + 1])
            or is_special_char(line_above[char_idx] )
            or is_special_char(line_above[char_idx + 1] )):
                return True

        elif char_idx == (len(current_line) - 1): # eind
            if (is_special_char(current_line[char_idx - 1])
            or is_special_char(line_above[char_idx] )
            or is_special_char(line_above[char_idx - 1] )):
                return True

        elif char_idx > 0 and char_idx < (len(current_line) - 1):
            if (is_special_char(current_line[char_idx - 1])
            or is_special_char(current_line[char_idx + 1] )
            or is_special_char(line_above[char_idx - 1] )
            or is_special_char(line_above[char_idx] )
            or is_special_char(line_above[char_idx + 1])):
                return True

        return False
            
    elif current_line_nr > 0 and current_line_nr < (len(lines) - 1):
        line_above = lines[current_line_nr - 1]
        current_line = lines[current_line_nr]
        line_below = lines[current_line_nr + 1]

        if char_idx == 0: #begin
            # .....
            # .....
            # 777..
            if (is_special_char(current_line[char_idx + 1])
            or is_special_char(line_above[char_idx])
            or is_special_char(line_above[char_idx + 1])
            or is_special_char(line_below[char_idx])
            or is_special_char(line_below[char_idx + 1])):
                return True

        elif char_idx == (len(current_line) - 1): # eind
            if (is_special_char(current_line[char_idx - 1])
            or is_special_char(line_above[char_idx])
            or is_special_char(line_above[char_idx - 1])
            or is_special_char(line_below[char_idx] )
            or is_special_char(line_below[char_idx - 1] )):
                return True

        elif char_idx > 0 and char_idx < (len(current_line) - 1):
            if (is_special_char(current_line[char_idx + 1])
            or is_special_char(current_line[char_idx - 1])
            or is_special_char(line_above[char_idx])
            or is_special_char(line_above[char_idx - 1] )
            or is_special_char(line_above[char_idx + 1] )
            or is_special_char(line_below[char_idx])
            or is_special_char(line_below[char_idx - 1] )
            or is_special_char(line_below[char_idx + 1])):
                return True

        return False

    else:
        print("Invalid linenr {}".format(current_line_nr))
        return False

def find_part_numbers_w_idx_v2(filename):
    with open(filename, 'r') as f:
        lines = f.readlines()

    line_count = len(lines)

    result = []
    current_number_str = ""

    result_sum = 0

    for idx, line in enumerate(lines):
        result_line = []
        save_curr_num = False
        for idy, c in enumerate(line):
            if c.isdigit():
                current_number_str += c
                #print(f"Checking digit: {lines[idx][idy]} line: {idx} index {idy}")
                if has_sticky_symbol(lines, idx, idy):
                    #print(f"Has sticky! {lines[idx][idy]} line {idx} index {idy}")
                    save_curr_num = True
            elif c.isdigit() and idy == (len(line) - 1):
                if current_number_str != "":
                    if save_curr_num:
                        current_number = int(current_number_str)

                        print(f"{current_number}, ")
                        start_num_idx = idy - len(current_number_str)

                        result.append((current_number, start_num_idx))
                        result_line.append((current_number, start_num_idx))
                        result_sum += current_number

                        save_curr_num = False

                    current_number_str = ""
            else:
                if current_number_str != "":
                    if save_curr_num:
                        current_number = int(current_number_str)

                        print(f"{current_number}, ")
                        start_num_idx = idy - len(current_number_str)

                        result.append((current_number, start_num_idx))
                        result_line.append((current_number, start_num_idx))
                        result_sum += current_number

                        save_curr_num = False

                    current_number_str = ""

        print("Line {} {}\n".format(idx, result_line))

    print("v2 sum {}".format(result_sum))
    return result

def day_three_part_one_v2(filename):
    output = find_part_numbers_w_idx_v2(filename)
    answer = sum([value for value, _ in output])
    return answer

ans = day_three_part_one_v2(filename)
print("ans v2: ", ans)