import json
import matplotlib.pyplot as matplotlib

s_path_file = "./optimal_difference.json"
o_file = open(s_path_file)


a_o_light_curve = json.load(o_file)

for o_light_curve in a_o_light_curve:

    # naming the x axis
    matplotlib.xlabel('hmjs - time')
    # naming the y axis
    matplotlib.ylabel('magnitude - light intensity')

    matplotlib.plot(o_light_curve["hmjd"], o_light_curve["mag"], 'o')

    matplotlib.savefig(f"objectid_${o_light_curve['objectid']}.png")
    break

    # print(o_light_curve["objectid"])

