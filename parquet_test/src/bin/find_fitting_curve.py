from array import array
import numpy as np
import pyarrow.parquet as pq
import matplotlib.pyplot as plt
from scipy.optimize import curve_fit as fit
import os
 
n_max_difference = 0.25 #because mean of standard deviation is approx. 1.5 
#-> that's the noise of a constant source so same noise around ML theory function too
 
# FUNCTIONS
def f_ML_theo(n_d, n_umin, n_tE, n_I, n_t_max): #theoretische ML-Funktion, input = time-array, output = mag-array
    #umin => between 0 and 1 - the smaller, the bigger the amplitude
    #tE => duration of Event - the bigger, the wider the curve
    #I => intensity I = (light intensity)/Area of star without amplification
    #t_max => time when amplification A of I reaches maximum a
    n_u = np.sqrt(n_umin**2 + ((n_d-n_t_max)/n_tE)**2)
    if n_u*np.sqrt(n_u**2 + 4) != 0:
        n_A = n_I*((n_u**2 + 2) / (n_u*np.sqrt(n_u**2 + 4))) 
        n_M = -2.5*np.log10(n_A) # conversion to magnitude
    else: n_M = -10 #if umin = 0 and d = t0, amplitude theoretically becomes infinite 
    return n_M
 
def f_ML_theo_for_array(a_t, n_umin, n_tE, n_I, n_t_max):
    a_theo_mag = []
    for n_t in a_t:
        a_theo_mag.append(f_ML_theo(n_t, n_umin, n_tE, n_I, n_t_max))
 
 
def fit(f_theo = function, a_x_t = array, a_y_mag = array): # returns number: optimal difference
    a_differences = [] 
 
    # Limits (numbers) - parameters beyond these are not sensible
 
    n_min_mag = min(a_y_mag)
    n_max_mag = max(a_y_mag)
 
    n_min_tE = 1
    n_max_tE = int(n_max_mag) - int(n_min_mag)
 
    for n_umin in [0.01*x for x in range(1,100)]: # umin has to be between 0 and 1 but range doesn't work for floats - make list of ints and divide again
        for n_tE in range(n_min_tE, n_max_tE):
            for n_mag in [0.01*x for x in range(int(100*n_min_mag), int(100*n_max_mag))]: #for I-calculation, gets converted later
                for n_t_max in range(int(n_min_mag + 0.5*n_tE), int(n_max_mag - 0.5*n_tE)): 
                    n_difference_theo_data_mean = np.sum(
                        [np.absolute(
                            f_theo(a_x_t, 
                                   n_umin, 
                                   n_tE, 
                                   10**(n_mag/-2.5), # convert mag to I 
                                   n_t_max)[x] - a_y_mag[x])
                                   for x in range(len(a_y_mag) # do for every mag-value
                            )]) # needs I as input but data in mag - conversion here
                    # get rid of -/+-differences to take min difference afterwards -> makes list of theo-mag-values for every value and then calculates mean 
                    a_differences.append(n_difference_theo_data_mean) # save in list and then calculate minimum 
 
    n_optimal_difference = min(a_differences)    
    return n_optimal_difference
 
 
class O_indices: #object for indices as "objectid", not as "lc[0]"
    n_objectid = 0
    n_filterid = 1
    n_fieldid = 2
    n_rcid = 3
    n_objra = 4
    n_objdec = 5
    n_nepochs = 6
    a_hmjd = 7
    a_mag = 8
    a_magerr = 9
    a_clrcoeff = 10
    a_catflags = 11
o_indices = O_indices()
 
#a_a_data = np.load('C:\Kanti\Microlensing\Python\dotpyfiles\ztf_000202_zr_c14_q3_dr11.parquet_filtered.npy', allow_pickle=True)
 
# FILTER 
for s_path_root, a_s_folder, a_s_file in os.walk(dir):
    for s_name_file in a_s_file:
        s_path_file = s_path_root + "/" + s_name_file
        a_a_data = np.load(s_path_file, allow_pickle=True) # allow_pickle = True - error in new module version, needs this in order to funtion
        a_a_filtered = [
            a_LC for a_LC in a_a_data 
            if 
            fit(f_ML_theo, a_LC[o_indices.a_hmjd], a_LC[o_indices.a_mag]) < n_max_difference
        ]
        np.save(s_name_file + "_fitted", a_a_filtered)