from distutils.log import debug
from re import X
import numpy as np
import matplotlib.pyplot as plt
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
        n_M = -2.5*np.log10(n_A) # conversion luminosity to magnitude
    else: n_M = -10 #if umin = 0 and d = t0, amplitude theoretically becomes infinite 
    return n_M
 
def f_ML_theo_for_array(a_t, n_umin, n_tE, n_I, n_t_max):
    a_theo_mag = []
    for n_t in a_t:
        a_theo_mag.append(f_ML_theo(n_t, n_umin, n_tE, n_I, n_t_max))
    return a_theo_mag
 
 
def fit(f_ML_theo_for_array, a_x_t, a_y_mag): # returns number: optimal difference
    a_differences = [] 
    if (len(a_x_t) == 57):
        debug
        print("test")
    # Limits (numbers) - parameters beyond these are not sensible
 
    n_min_mag = min(a_y_mag)
    n_max_mag = max(a_y_mag)
 
    n_min_tE = 1
    n_max_tE = int(max(a_x_t) - min(a_x_t))
    n_steps_umin = 5  # range()-function takes ints only -> multiplies value do eliminate decimal stuff and divides back again
    n_steps_mag = 5
    for n_umin in [(1/n_steps_umin)*x for x in range(1,n_steps_umin)]: # umin has to be between 0 and 1 but range doesn't work for floats - make list of ints and divide again
        for n_tE in range(n_min_tE, n_max_tE):
            for n_mag in [(1/n_steps_mag)*x for x in range(int(n_steps_mag*n_min_mag), int(n_steps_mag*n_max_mag))]: #for I-calculation, gets converted later
                n_range_min = int(n_min_tE + 0.5*n_tE); 
                n_range_max = int(n_max_tE - 0.5*n_tE)
                if(n_range_min == n_range_max):
                    continue
                
                for n_t_max in range(n_range_min,n_range_max): 
                    print(n_t_max)
                    a_theo_mag = f_ML_theo_for_array(
                        a_x_t, 
                        n_umin, 
                        n_tE, 
                        10**(n_mag/-2.5), # # needs I as input but data in mag - conversion here
                        n_t_max
                    )
                    n_difference_theo_data_mean = np.sum(
                        [
                            np.absolute(
                                a_theo_mag[x] - a_y_mag[x]
                            )
                            for x
                            in range(len(a_y_mag))
                        ]
                    )/len(a_y_mag)
                    # print(n_difference_theo_data_mean)
                    # get rid of -/+-differences to take min difference afterwards -> makes list of theo-mag-values for every value and then calculates mean 
                    # if n_difference_theo_data_mean is None: 
                        # print(n_difference_theo_data_mean)
                    a_differences.append(n_difference_theo_data_mean) # save in list and then calculate minimum 

    print(len(a_differences))
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
 
# a_a_data = np.load('C:\Kanti\Microlensing\Python\dotpyfiles\ztf_000202_zr_c14_q3_dr11.parquet_filtered.npy', allow_pickle=True)
dir = "C:/Kanti/Microlensing/Python/some_filtereddata"


s_path_file = "./ztf_000761_zr_c14_q3_dr11.parquet_filtered.npy" # static for testing
s_name_file = s_path_file.split("/")[-1]

a_a_data = np.load(s_path_file, allow_pickle=True) # allow_pickle = True - error in new module version, needs this in order to funtion
a_a_filtered = [
    a_LC for a_LC in a_a_data 
    if 
    fit(f_ML_theo_for_array, a_LC[o_indices.a_hmjd], a_LC[o_indices.a_mag]) < n_max_difference
]
np.save(s_name_file + "_fitted", a_a_filtered)

# # FILTER 
# for s_path_root, a_s_folder, a_s_file in os.walk(dir):
#     for s_name_file in a_s_file:
#                 # process the data
#         n_index = a_s_file.index(s_name_file)
#         print(f"-----------processing----------------")
#         print(f"file: {n_index}") 
#         print("")
        
#         s_path_file = s_path_root + "/" + s_name_file

#         a_a_data = np.load(s_path_file, allow_pickle=True) # allow_pickle = True - error in new module version, needs this in order to funtion
#         a_a_filtered = [
#             a_LC for a_LC in a_a_data 
#             if 
#             fit(f_ML_theo_for_array, a_LC[o_indices.a_hmjd], a_LC[o_indices.a_mag]) < n_max_difference
#         ]
#         np.save(s_name_file + "_fitted", a_a_filtered)