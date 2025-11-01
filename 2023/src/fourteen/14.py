import pandas as pd

def tilt_to_north(df):
    # change df so that rocks are tilted north
    for col_idx in range(df.shape[1]):
        for row_idx in range(df.shape[0]):
            if df[col_idx][row_idx] == "O":
                if row_idx > 0: # bij de eerste kun je niet - 1 doen
                    if df[col_idx][row_idx - 1] == ".": # als die omhoog kan
                        df[col_idx][row_idx - 1] = "O"
                        df[col_idx][row_idx] = "."  
                        #print(f"Rock moved from ({row_idx}, {col_idx}) to ({row_idx - 1}, {col_idx})")
    return df   

def tilt_to_south(df):
    # change df so that rocks are tilted south
    for col_idx in range(df.shape[1]):
        for row_idx in range(df.shape[0]):
            if df[col_idx][row_idx] == "O":
                if row_idx < df.shape[0] - 1: # bij de laatste kun je niet + 1 doen
                    if df[col_idx][row_idx + 1] == ".": # als die omlaag kan
                        df[col_idx][row_idx + 1] = "O"
                        df[col_idx][row_idx] = "."  
                        #print(f"Rock moved from ({row_idx}, {col_idx}) to ({row_idx - 1}, {col_idx})")
    return df   

def tilt_to_east(df):
    # change df so that rocks are tilted east
    for col_idx in range(df.shape[1]):
        for row_idx in range(df.shape[0]):
            if df[col_idx][row_idx] == "O":
                if col_idx > 0: # bij de eerste kun je niet - 1 doen
                    if df[col_idx - 1][row_idx] == ".": # als die naar rechts kan
                        df[col_idx - 1][row_idx] = "O"
                        df[col_idx][row_idx] = "."  
                        #print(f"Rock moved from ({row_idx}, {col_idx}) to ({row_idx - 1}, {col_idx})")
    return df   

def tilt_to_west(df):
    # change df so that rocks are tilted west
    for col_idx in range(df.shape[1] - 1): # bij de laatste kun je niet + 1 doen
        for row_idx in range(df.shape[0]):
            if df[col_idx][row_idx] == "O":
                if df[col_idx + 1][row_idx] == ".": # als die naar links kan
                    df[col_idx + 1][row_idx] = "O"
                    df[col_idx][row_idx] = "."  
                    #print(f"Rock moved from ({row_idx}, {col_idx}) to ({row_idx - 1}, {col_idx})")
    return df   

def i_spin_my_df_right_round(df):
    df = tilt_to_north(df)
    df = tilt_to_west(df)
    df = tilt_to_south(df)
    df = tilt_to_east(df)
    return df

def get_rock_coordinates(df):
    row, col = (df.map(lambda x: str(x).startswith('O'))).values.nonzero()
    print("Rock rows", row)
    row = df.shape[1] - row # col count starts from bottom
    print("Rock rows transformed", row)
    coordinates = list(zip(row, col))
    #print(coordinates)
    return coordinates

def calc_rock_pressure(df):
    # elke circeltje, rij index tellen en sommeren
    coordinates = get_rock_coordinates(df)
    return sum(coordinate[0] for coordinate in coordinates)

def day_fourteen_part_one(filename):
    #input = open(filename, "r").read().strip()
    with open(filename, 'r') as f:
        lines = f.readlines()

    df_width = len(lines)
    columns = []
    for i in range(df_width):
        columns.append(1)

    df = pd.read_fwf(filename, widths=columns, header=None)
    print(df)

    # for i in range(len(lines)):
    #     df = tilt_to_north(df)

    # PART 2
    for i in range(1000000000):
        print(i)
        for i in range(len(lines)):
            df = i_spin_my_df_right_round(df)
        
    print(df)
    pressure = calc_rock_pressure(df)
    return pressure
    


filename = "D:/git/magic/aoc2023/aoc2023/src/fourteen/input.txt"
ans1 = day_fourteen_part_one(filename)
print(f"Part one {ans1}")

# ans2 = day_fourteen(filename)
# print(f"Part two {ans2}")
