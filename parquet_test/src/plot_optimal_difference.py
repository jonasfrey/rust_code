import json
from webbrowser import BackgroundBrowser
import matplotlib.pyplot as matplotlib
import subprocess
import sys

if(len(sys.argv)>1):
    s_path_file = sys.argv[1]
else: 
    s_path_file = "./optimal_difference.json"

o_file = open(s_path_file)


a_o_light_curve = json.load(o_file)

n_i = 0


for o_light_curve in a_o_light_curve:
    n_i+=1
    

    f, o_ax = matplotlib.subplots()

    # naming the x axis
    matplotlib.xlabel('hmjs - time')
    # naming the y axis
    matplotlib.ylabel('magnitude - light intensity')

    s_title = f"""n_objectid {o_light_curve['objectid']}
n_umin_estimated {o_light_curve['n_umin_estimated']}
n_t_max_estimated {o_light_curve['n_t_max_estimated']}"""
    # matplotlib.title(s_title)
    matplotlib.text(
        0.5,
        1.15,
        s_title,
        horizontalalignment='center',
        verticalalignment='top',
        transform = o_ax.transAxes, 
        # backgroundcolor =dict(facecolor='red', alpha=0.5)
        fontdict=dict(fontsize=10),
        bbox=dict(facecolor='white', alpha=0.1)
    )

    matplotlib.plot(
        o_light_curve["a_n_hours_modified_julian_date_estimated"],
        o_light_curve["a_n_magnitude_estimated"],
        'go', 
        label="estimated"
    )

    # matplotlib.plot(
    #     o_light_curve["hmjd"],
    #     o_light_curve["mag"],
    #     'bo', 
    #     label="real data"
    # )
    
    matplotlib.legend()

    # matplotlib.show()
    # exit(1)
    # matplotlib.savefig(f"n_objectid_{o_light_curve['objectid']}_{n_i}.png")
    s_picture_prefix = s_path_file.split("/")[-1].split(".")[0]
    s_picture_prefix_full = f"{s_picture_prefix}_plotted_"
    matplotlib.savefig(f"{s_picture_prefix_full}{n_i}.png")
    matplotlib.clf()
    matplotlib.close()



a_s_argument = [
        "ffmpeg", 
        "-y",
        "-framerate", 
        "3", 
        # "-pattern_type"
        # "glob",
        "-i", 
        f"{s_picture_prefix_full}%d.png",
        "-c:v", 
        "libx264", 
        "-profile:v",
        "baseline",
        "-level",
        "3.0",
        "-pix_fmt", 
        "yuv420p",
        "-r", 
        "30", 
        f"{s_picture_prefix_full}.mp4"
    ]
s_command = " ".join(a_s_argument)
print(f"running command:{s_command}")
subprocess.run(
    a_s_argument
)

