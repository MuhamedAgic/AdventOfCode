from operator import *
from functools import reduce # python3 compatibility
from str import *

def get_amount_winning_possibilities(race_time, race_distance_record):
    amount_winning_possibilities = 0

    for mm_p_s in range(race_time):
        time_left = race_time - mm_p_s
        if mm_p_s * time_left > race_distance_record:
            amount_winning_possibilities += 1

    return amount_winning_possibilities

def day_6_part_2(filename):
    with open(filename, 'r') as f:
        lines = f.readlines()

    #print(lines[0].split(":")[1].replace(" ", ""))
    race_time = int(lines[0].split(":")[1].replace(" ", ""))
    race_distance = int(lines[1].split(":")[1].replace(" ", ""))

    print(f"Race time {race_time}")
    print(f"Race distance {race_distance}")

    winning_possibilities_all_rounds = []

    current_amount_possibilities = get_amount_winning_possibilities(race_time, race_distance)
    winning_possibilities_all_rounds.append(current_amount_possibilities)

    print(f"winning possibilities all rounds: {winning_possibilities_all_rounds}")
    print(reduce(mul, winning_possibilities_all_rounds, 1))
    return reduce(mul, winning_possibilities_all_rounds, 1)

filename = "D:/git/magic/aoc2023/aoc2023/src/six/input.txt"
day_6_part_2(filename)