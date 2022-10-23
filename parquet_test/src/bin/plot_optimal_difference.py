import json
import matplotlib.pyplot as matplotlib

s_path_file = "./optimal_difference.json"
o_file = open(s_path_file)


a_o_light_curve = json.load(o_file)

n_i = 0
for o_light_curve in a_o_light_curve:
    n_i+=1
    # naming the x axis
    matplotlib.xlabel('hmjs - time')
    # naming the y axis
    matplotlib.ylabel('magnitude - light intensity')

    matplotlib.title(f"n_objectid {o_light_curve['objectid']}")

    matplotlib.plot(o_light_curve["hmjd"], o_light_curve["mag"], 'o')


    # matplotlib.savefig(f"objectid_{o_light_curve['objectid']}.png")
    matplotlib.savefig(f"{n_i}.png")
    matplotlib.clf()

    # break

    # print(o_light_curve["objectid"])

