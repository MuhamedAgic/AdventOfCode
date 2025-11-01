class Direction(Enum):
    GROUND = 6
    NORTH_SOUTH = 5
    EAST_WEST = 4
    NORTH_EAST = 3
    NORTH_WEST = 2
    SOUTH_EAST = 1
    SOUTH_WEST = 0
    INVALID = -1

class Pipe: # or node
    def __init__(self):
        self.value = "0"
        self.distance_to_s_from_right = 0
        self.distance_to_s_from_left = 0
        self.x_loc = 0
        self.y_loc = 0

    def __str__(self):
        return f"Current pipe: '{self.value}', x loc: {self.x_loc}, y loc: {self.y_loc}, distance to s from right: '{self.distance_to_s_from_right}', distance to s from left: '{self.distance_to_s_from_right}'"


def get_xy_directions(pipe_type):
    if pipe_type == "|":
        return [[0, 1], [0, -1]]
    elif pipe_type == "-":
        return [[1, 0], [-1, 0]]
    elif pipe_type == "L":
        return [[0, 1], [1, 0]]
    elif pipe_type == "J":
        return [[0, 1], [-1, 0]]
    elif pipe_type == "7":
        return [[0, -1], [-1, 0]]
    elif pipe_type == "F":
        return [[0, -1], [1, 0]]
    elif pipe_type == ".":
        return [[0, 0]]
    elif pipe_type == "S":
        return [[0, 0]]
    else:
        print(f"Invalid pipe type: {pipe_type}")
        return [[0, 0]]

def look_around(pipe, pipes):
    
            



def loop_left(pipes):
    for row in pipes:
        for idx, pipe in enumerate(row):
            if pipe.value == "S":

                
            

def loop_right(pipes):


def day_ten_part_one(filename):
    with open(filename, 'r') as f:
        lines = f.readlines()

    pipe_map = []
    for idx, line in enumerate(lines):
        row = []
        for idy, char in enumerate(line):
            pipe = Pipe()
            pipe.value = char
            pipe.x_loc = idx
            pipe.y_loc = idy
            row.append(pipe)
        pipe_map.append(row)




filename = "D:/git/magic/aoc2023/aoc2023/src/ten/example_input.txt"
ans1 = day_ten_part_one(filename)
print(f"Part one {ans1}")

# ans2 = day_eight_part_two(filename)
# print(f"Part two {ans2}")
