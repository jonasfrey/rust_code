from array import array
from locale import nl_langinfo
from platform import python_branch
from xml.dom import minicompat
import numpy as np
import pyarrow.parquet as pq
import matplotlib.pyplot as plt
from scipy.optimize import curve_fit as fit
import os

max_difference = 0.25  # because mean of standard deviation is approx. 1.5
# -> that's the noise of a constant source so same noise around ML theory function too
# ->

# FUNCTIONS


# theoretische ML-Funktion, input = time-array, output = mag-array
def ML_theo(d, umin, tE, I, t_max):
    # umin => between 0 and 1 - the smaller, the bigger the amplitude
    # tE => duration of Event - the bigger, the wider the curve
    # I => intensity I = (light intensity)/Area of star without amplification
    # t_max => time when amplification A of I reaches maximum a
    u = np.sqrt(umin**2 + ((d-t_max)/tE)**2)
    if u*np.sqrt(u**2 + 4) != 0:
        A = I*((u**2 + 2) / (u*np.sqrt(u**2 + 4)))
        M = -2.5*np.log10(A)  # conversion to magnitude
    else:
        M = -10  # if umin = 0 and d = t0, amplitude theoretically becomes infinite
    return M


def ML_theo_for_array(a_t, umin, tE, I, t_max):
    a_theo_mag = []
    for t in a_t:
        a_theo_mag.append(ML_theo(t, umin, tE, I, t_max))


def fit(f_theo=function, a_x_t=array, a_y_mag=array):  # returns number: optimal difference
    a_differences = []

    # Limits (numbers) - parameters beyond these are not sensible

    min_mag = min(a_y_mag)
    max_mag = max(a_y_mag)

    min_tE = 1
    max_tE = int(max_mag) - int(min_mag)

    # umin has to be between 0 and 1 but range doesn't work for floats - make list of ints and divide agai
    n_index = 1
    while(n_index < 100):
        umin = 0.01*n_index

    for umin in [0.01*x for x in range(1, 100)]:
        for tE in range(min_tE, max_tE):
            # for I-calculation, gets converted later
            for mag in [0.01*x for x in range(int(100*min_mag), int(100*max_mag))]:
                for t_max in range(int(min_mag + 0.5*tE), int(max_mag - 0.5*tE)):
                    difference_theo_data_mean = np.sum(
                        [
                            np.absolute(
                                f_theo(
                                    a_x_t,
                                    umin,
                                    tE,
                                    10**(mag/-2.5),
                                    t_max
                                )[x] - a_y_mag[x]
                            )
                            for x
                            in range(len(a_y_mag))
                        ]
                    )  # needs I as input but data in mag - conversion here
                    # get rid of -/+-differences to take min difference afterwards -> makes list of theo-mag-values for every value and then calculates mean
                    # save in list and then calculate minimum
                    a_differences.append(difference_theo_data_mean)

    optimal_difference = min(a_differences)
    return optimal_difference


class O_indices:  # object for indices as "objectid", not as "lc[0]"
    objectid = 0
    filterid = 1
    fieldid = 2
    rcid = 3
    objra = 4
    objdec = 5
    nepochs = 6
    hmjd = 7
    mag = 8
    magerr = 9
    clrcoeff = 10
    catflags = 11


o_indices = O_indices()

#a_a_data = np.load('C:\Kanti\Microlensing\Python\dotpyfiles\ztf_000202_zr_c14_q3_dr11.parquet_filtered.npy', allow_pickle=True)

# FILTER
for s_path_root, a_s_folder, a_s_file in os.walk(dir):
    for s_name_file in a_s_file:
        s_path_file = s_path_root + "/" + s_name_file
        a_a_data = np.load(s_path_file)
        a_a_filtered = [
            a_LC for a_LC in a_a_data
            if
            fit(ML_theo, a_LC[o_indices.hmjd],
                a_LC[o_indices.mag]) < max_difference
        ]
        np.save(s_name_file + "_fitted", a_a_filtered)
