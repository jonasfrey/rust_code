import numpy 
import sys
import time
import inspect

n_max = 100000000

def f_iterate_numpy_array():
    a_n = numpy.arange(1, n_max, 1, dtype=int)
    n_sum = 0
    for n in a_n:
        n_sum += n

    print(f"n_sum: {n_sum}")
    print("done")


def f_iterate_while_loop():
    n = 0
    n_sum = 0
    while(n < n_max):
        n_sum += n
        n+=1

    print(f"n_sum: {n_sum}")
    print("done")

def f_iterate_range():
    n = 0
    n_sum = 0
    for n in range(1,n_max):
        n_sum += n

    print(f"n_sum: {n_sum}")
    print("done")

def f_iterate_array_from_range():
    n = 0
    n_sum = 0
    for n in [n for n in range(1, n_max)]:
        n_sum += n

    print(f"n_sum: {n_sum}")
    print("done")




a_s_function_name = [
    "f_iterate_numpy_array",
    "f_iterate_while_loop",
    "f_iterate_range",
    "f_iterate_array_from_range",
]

for s_function_name in a_s_function_name: 
    print("------------------------")
    print(f"function name: '{s_function_name}'")
    print(f"function body:")
    a_s_line_function = inspect.getsource(f_iterate_array_from_range)
    print(a_s_line_function)
    print(f"function return:")
    n_ts_seconds_first = time.time()
    n = locals()[s_function_name]()
    n_ts_seconds_second = time.time()
    n_ts_seconds_delta = n_ts_seconds_second - n_ts_seconds_first
    print(n)
    print(f"time delta seconds: {n_ts_seconds_delta}")
    





if(len(sys.argv) > 1):
    locals()[sys.argv[1]]()