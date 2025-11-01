import pandas as pd
from scipy.spatial.distance import cityblock
from scipy.spatial.distance import cdist
import numpy as np

def expand_cosmos(cosmos_df):
    print(cosmos_df)

    # Rijen en kolommen met alleen puntjes
    dots_rows = (cosmos_df == ".").all(axis=1) # boolean array, voldoet row
    dots_columns = (cosmos_df == ".").all(axis=0)

    print(dots_rows)
    print(dots_columns)

    # voeg puntjes toe rows
    move_x_row = 0
    for idx, row in enumerate(dots_rows):
        if row:
            cosmos_df.loc[idx+move_x_row+0.5] = pd.Series(["."] * cosmos_df.shape[1])
            cosmos_df = cosmos_df.sort_index().reset_index(drop=True)
            move_x_row += 1

    # Add a column of dots
    move_x_col = 0
    for idx, col in enumerate(dots_columns):
        if col:
            move_x_col += 1
            cosmos_df.insert(loc=idx+move_x_col, column="e" + str(idx), value=".")

    print("\n\n* * * * * * * C O S M I C * * * E X P A N S I O N * * * * * *\n\n")
    print(cosmos_df)
    return cosmos_df

def expand_cosmos_v2(cosmos_df):
    print(cosmos_df)

    # Rijen en kolommen met alleen puntjes
    dots_rows = (cosmos_df == ".").all(axis=1) # boolean array, voldoet row
    dots_columns = (cosmos_df == ".").all(axis=0)

    print(dots_rows)
    print(dots_columns)

    expansion_factor = 1000000 - 1
                                     
    # Add a column of dots
    move_x_col = 0
    for idx, col in enumerate(dots_columns):
        if col:
            for i in range(expansion_factor):
                cosmos_df.insert(loc=idx+move_x_col+i, column="e" + str(idx) + "-" + str(i), value=".")
            move_x_col += expansion_factor

    # transpose, en doe dan hetzelfde ;)
    cosmos_df = cosmos_df.T

    move_x_col = 0
    for idx, col in enumerate(dots_rows): # alleen dan voor elke row die alleen puntjes had
        if col:
            for i in range(expansion_factor):
                cosmos_df.insert(loc=idx+move_x_col+i, column="e" + str(idx) + "-" + str(i), value=".")
            move_x_col += expansion_factor

    # terug transposen (al hoeft dat niet denk ik)
    cosmos_df = cosmos_df.T

    print("\n\n* * * * * * * C O S M I C * * * E X P A N S I O N * * * * * *\n\n")
    print(cosmos_df)
    return cosmos_df

def calc_dist(coord1, coord2):
    return abs(coord2[0] - coord1[0]) + abs(coord2[1] - coord1[1])

def get_distances_from_galaxy_coordinates(galaxy_coordinates):
    print("TOTAAL: ", len(galaxy_coordinates))
    distances_all_galaxies = []
    for idx, coordinate in enumerate(galaxy_coordinates):
        distances_current_galaxy = []
        for idy, other_coordinate in enumerate(galaxy_coordinates):
            if idy >= idx:
                distances_current_galaxy.append(calc_dist(coordinate, other_coordinate)) # bij hetzelfde, 0

        print(len(distances_current_galaxy))
        distances_all_galaxies.append(distances_current_galaxy)

    print(f"Einde: {distances_all_galaxies}")
    return sum([sum(distances) for distances in distances_all_galaxies])

def get_galaxy_coordinates(expanded_cosmos):
    row, col = (expanded_cosmos.map(lambda x: str(x).startswith('#'))).values.nonzero()
    coordinates = list(zip(row, col))
    #print(coordinates)
    return coordinates

def day_eleven(filename):
    with open(filename, 'r') as f:
        lines = f.readlines()

    cosmos_width = len(lines[0]) - 1
    columns = []
    for i in range(cosmos_width):
        columns.append(1)

    cosmos_df = pd.read_fwf(filename, widths=columns, header=None)
    expanded_cosmos_df = expand_cosmos_v2(cosmos_df)
    np.savetxt(r'expanded_input.txt', expanded_cosmos_df.values, fmt='%s')
    galaxy_coordinates = get_galaxy_coordinates(expanded_cosmos_df)
    summed_distances = get_distances_from_galaxy_coordinates(galaxy_coordinates)
    return summed_distances


filename = "D:/git/magic/aoc2023/aoc2023/src/eleven/input.txt"
# ans1 = day_eleven_part_one(filename)
# print(f"Part one {ans1}")

ans2 = day_eleven(filename)
print(f"Part two {ans2}")
