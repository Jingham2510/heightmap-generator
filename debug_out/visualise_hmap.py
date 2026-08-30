"""
Although similar to the heightmap visualisation tool used in the camera_cntrl section of rustbot_cntrl
this is a slightly modified script used to generate the debug outputs of the heightmap_generator trajectory generator
"""

import sys
import matplotlib.pyplot as plt
import re
import numpy as np
import math


class HeightMap:
    lower_bounds = []
    upper_bounds = []
    cells = [[]]

    def __init__(self, lower_bounds, upper_bounds, cells):
        self.lower_bounds = lower_bounds
        self.upper_bounds = upper_bounds
        self.cells = cells

    """
    Display the heightmap using imshow
    """

    def save(self, title):

        fig, ax = plt.subplots()

        im = plt.imshow(self.cells, cmap="plasma")
        # Colorbar settings
        cbar = fig.colorbar(im)
        cbar.set_label("Depth (m)", size=16)
        cbar.ax.tick_params(labelsize=16)

        # Tick settings
        plt.yticks([])
        plt.xticks([])

        plt.savefig(f"{title}.png", dpi =200)

        plt.close()


    """
    Save the heightmap without any extra info (i.e. no cmap or colorbar)
    """
    def save_less(self, title):
        fig, ax = plt.subplots()

        im = plt.imshow(self.cells)
        # Colorbar settings
        # Tick settings
        plt.yticks([])
        plt.xticks([])

        plt.savefig(f"{title}.png", dpi =200)

        plt.close()

    """
    Save a heightmap as a scatter plot
    Determine the scatter points by looking at every cell and saving the cells that have information in
    """
    def save_as_scatter(self, title):

        X = []
        Y = []
        #Determine which cells have info in
        it = np.nditer(self.cells, flags=['multi_index'])
        for cell in it:
            if not math.isnan(cell) and cell != 0.0:
                X.append(it.multi_index[1])
                Y.append(1000 - it.multi_index[0])


        plt.scatter(X, Y)

        plt.title(f"{title}")
        plt.savefig(f"{title}.png", dpi =200)

        plt.close()

       

        return





"""
Returns a hmap as a numpy matrix from a given file

@args
file : An already opened file (i.e. using open())
"""


def heightmap_from_file(file, skip_first):
    row_list = []

    lower_bounds = [0.0, 0.0]
    upper_bounds = [0.0, 0.0]

    for line in file:
        # Extract the bounds from the first line
        if not skip_first:
            skip_first = True

            # remove the non numbers
            # stripped_line = line.strip("bnds:").strip("[").strip("]")
            #

            split = line.split("[")

            low = re.sub("[^0-9\\.,]", "", split[1])

            high = re.sub("[^0-9\\.,]", "", split[2])

            low = low.split(",")
            high = high.split(",")

            # Extract the number and set the bounds
            for i in range(2):
                lower_bounds[i] = float(low[i])
                upper_bounds[i] = float(high[i])

            continue

        # Create the row
        row = []

        # Extract each value in the row
        for value in line.strip().split(","):
            # Ensure the value is not empty
            if value != "":
                if value == "0.0":
                    row.append(np.nan)
                else:
                    row.append(np.float64(value))

        row_list.append(row)

    cells = np.matrix(row_list)

    return HeightMap(lower_bounds, upper_bounds, cells)


if __name__ == "__main__":


    #print the system arguments
    for arg in sys.argv:
        #Match the argument to what to generate
        match arg:
            #Difference map creation
            case "--diff_map":
                diff_map = heightmap_from_file(open("debug_out/difference_map.txt"), False)
                diff_map.save("debug_out/difference_map")
                diff_loaded = True

            #Edge map creation
            case _ if "edges" in arg:
                split_arg = arg.split("_")

                for i in range(int(split_arg[1]) + 1):
                    fp = open(f"debug_out/shape_{i}.txt")
                    shape = heightmap_from_file(fp, False)
                    shape.save_less(f"debug_out/shape_{i}")                   


            #Point map creation
            case "--waypoint":
                points_map = heightmap_from_file(open("debug_out/waypoints.txt"), False)
                points_map.save_as_scatter("debug_out/waypoints")
   
   

    