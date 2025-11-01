import sys
sys.setrecursionlimit(90000000) # wanted to set on 65 duizend miljoen

class Node:
    def __init__(self):
        self.location = "000"
        self.left = "000"
        self.right = "000"

    def __str__(self):
        return f"You are here: '{self.location}', left: '{self.left}', right: '{self.right}'"

#************************* PART ONE ********************************

def goto_zzz(steps_to_take, nodes, current_node_loc="AAA", at_current_step=0, steps_taken=0):
    if current_node_loc == "ZZZ":
        print(f"Total steps: {steps_taken}")
        return steps_taken

    if at_current_step == len(steps_to_take) - 1: # instructies opnieuw uitvoeren
        at_current_step = 0

    current_node = next((node for node in nodes if node.location == current_node_loc), None)
    #print(f"{current_node}")
    if current_node is None:
        print("beltegoed is op, spreek je later!")
        return steps_taken

    current_step = steps_to_take[at_current_step]

    steps_taken += 1
    at_current_step += 1

    if current_step == "L":
        #print(f"Going to '{current_node.left}'")
        goto_zzz(steps_to_take, nodes, current_node.left, at_current_step=at_current_step, steps_taken=steps_taken)
    elif current_step == "R":
        #print(f"Going to '{current_node.right}'")
        goto_zzz(steps_to_take, nodes, current_node.right, at_current_step=at_current_step, steps_taken=steps_taken)

def day_eight_part_one(filename):
    with open(filename, 'r') as f:
        lines = f.readlines()

    steps_to_take = lines[0]
    nodes = []
    print(steps_to_take)
    for idx, line in enumerate(lines):
        if idx > 1:
            location = line.split("=")[0].replace(" ", "")
            left     = line.split("=")[1].split(",")[0].replace(" (", "")
            right    = line.split("=")[1].split(",")[1].replace(" ", "").replace(")\n", "")
            if idx == len(lines):
                right    = line.split("=")[1].split(",")[1].replace(" ", "").replace(")", "")

            node = Node()
            node.location = location
            node.left = left
            node.right = right

            nodes.append(node)

    return goto_zzz(steps_to_take, nodes)

#************************* PART TWO ********************************
def goto_zzz_p2(steps_to_take, nodes, current_node_loc, at_current_step=0, steps_taken=0):
    if current_node_loc.endswith("Z"):
        print(f"RETURNING Total steps: {steps_taken}, at instruction: {at_current_step}")
        return steps_taken, at_current_step

    if at_current_step == (len(steps_to_take) - 1): # instructies opnieuw uitvoeren
        at_current_step = 0

    current_node = next((node for node in nodes if node.location == current_node_loc), None)
    #print(f"{current_node}")
    if current_node is None:
        print(f"Node not found! Loc: {current_node_loc}")
        return steps_taken, at_current_step

    current_step = steps_to_take[at_current_step]

    steps_taken += 1
    at_current_step += 1

    if current_step == "L":
        #print(f"Going to '{current_node.left}'")
        goto_zzz_p2(steps_to_take, nodes, current_node.left, at_current_step=at_current_step, steps_taken=steps_taken)
    elif current_step == "R":
        #print(f"Going to '{current_node.right}'")
        goto_zzz_p2(steps_to_take, nodes, current_node.right, at_current_step=at_current_step, steps_taken=steps_taken)
    else:
        print("Instructie ontvangen die geen L of R is!")

    return steps_taken, at_current_step


def goto_zzz_p2_v2(steps_to_take, nodes, current_nodes, at_current_step=0, steps_taken=0):
    all_start_nodes_taken_steps = []
    all_stopped_at_step = []
    steps_taken_from_this_node = 0
    stopped_at_step = 0
    for idx, current_node in enumerate(current_nodes):
        print(f"\nGoing to z from: {current_node}")
        try:
            steps_taken_from_this_node, stopped_at_step = goto_zzz_p2(steps_to_take, nodes, current_node.location, at_current_step=at_current_step, steps_taken=steps_taken)
            print(f"Huidige node stats! {steps_taken_from_this_node}, {stopped_at_step}")
        except:
            print(f"None! {steps_taken_from_this_node}, {stopped_at_step}")
        
        all_start_nodes_taken_steps.append(steps_taken_from_this_node)
        all_stopped_at_step.append(stopped_at_step)

    # ga net zo lang door totdat alle getallen hetzelfde zijn, hetzelfde aantal stappen gezet, want dan pas ben je simultaneous
    print(all_start_nodes_taken_steps)
    if len(set(all_start_nodes_taken_steps)) == 1:
        print(f"Total steps: {all_start_nodes_taken_steps[0]}")
        return all_start_nodes_taken_steps[0]
    else:
        # doorgaan vanaf de max steps taken die een node heeft gedaan.
        step_to_resume_from = all_stopped_at_step.index(all_start_nodes_taken_steps.index(max(all_start_nodes_taken_steps)))
        steps_taken = max(all_start_nodes_taken_steps)
        print("\n\n\nNOT SIMULTANEOUS, resuming from {step_to_resume_from}, steps taken already {steps_taken}\n\n\n")
        goto_zzz_p2_v2(steps_to_take, 
                       nodes, 
                       current_nodes, 
                       at_current_step = step_to_resume_from, 
                       steps_taken = step_to_resume_from)


def day_eight_part_two(filename):
    with open(filename, 'r') as f:
        lines = f.readlines()

    steps_to_take = lines[0]
    nodes = []
    print(steps_to_take)
    for idx, line in enumerate(lines):
        if idx > 1:
            location = line.split("=")[0].replace(" ", "")
            left     = line.split("=")[1].split(",")[0].replace(" (", "")
            right    = line.split("=")[1].split(",")[1].replace(" ", "").replace(")\n", "")
            if idx == len(lines) - 1:
                right = line.split("=")[1].split(",")[1].replace(")", "").replace(" ", "")

            node = Node()
            node.location = location
            node.left = left
            node.right = right

            nodes.append(node)

    # for node in nodes:
    #     print(f"{node}")

    nodes_starting_with_a = [node for node in nodes if node.location.endswith("A")]
    for node in nodes_starting_with_a:
        print(f"Starting with A: {node}")
    steps = goto_zzz_p2_v2(steps_to_take, nodes, nodes_starting_with_a)
    return steps


filename = "D:/git/magic/aoc2023/aoc2023/src/eight/input.txt"
#ans1 = day_eight_part_one(filename)
#print(f"Part one {ans1}")

ans2 = day_eight_part_two(filename)
print(f"Part two {ans2}")



