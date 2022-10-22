import numpy as np
# import pyarrow as pa
import pyarrow
import fastparquet
import pandas 
import sys

# s_path_file = "./ztf_000722_zr_c07_q4_dr11.parquet_filtered.npy"
s_path_file = sys.argv[1]

a_data = np.load(s_path_file, allow_pickle=True)
o_pandas_dataframe = pandas.DataFrame(
    a_data, 
    columns=[
        "objectid",
        "filterid",
        "fieldid",
        "rcid",
        "objra",
        "objdec",
        "nepochs",
        "hmjd",
        "mag",
        "magerr",
        "clrcoeff",
        "catflags",
    ]
)
o_pandas_dataframe.to_parquet(s_path_file+".parquet")

print("{b_done:true}")