ZTF DR11 lightcurves in parquet format. There are 1,140 field subdirectories, containing 
a total of 163,319 parquet files (one for each field / chip / quadrant / filter 
combination).

File content and format are documented in section 12c of the release notes:
https://irsa.ipac.caltech.edu/data/ZTF/docs/releases/dr11/ztf_release_notes_dr11.pdf

Changed for DR11: catflags, filterid, and rcid have been changed to signed int32, 
readable by PySpark 

To read parquet using python/pandas:
https://pandas.pydata.org/docs/user_guide/io.html?#parquet

The parquet files can be verified on a Unix/Linux OS using:
"md5sum -c checksum.md5"

Total volume: ~ 5.0T

Lightcurve statistics in g-filter:
Number of lightcurves with >=  1 epoch :   1,374,166,492
Number of lightcurves with >=  2 epochs:   1,131,130,033
Number of lightcurves with >=  5 epochs:     947,853,482
Number of lightcurves with >= 10 epochs:     827,676,058
Number of lightcurves with >= 20 epochs:     704,388,801

Lightcurve statistics in r-filter:
Number of lightcurves with >=  1 epoch :   2,185,812,352
Number of lightcurves with >=  2 epochs:   1,871,980,498
Number of lightcurves with >=  5 epochs:   1,593,809,057
Number of lightcurves with >= 10 epochs:   1,425,065,897
Number of lightcurves with >= 20 epochs:   1,251,889,824

Lightcurve statistics in i-filter:
Number of lightcurves with >=  1 epoch :     632,573,766
Number of lightcurves with >=  2 epochs:     533,620,389
Number of lightcurves with >=  5 epochs:     449,931,752
Number of lightcurves with >= 10 epochs:     395,149,645
Number of lightcurves with >= 20 epochs:     335,487,307

--
Last modified: 2022-04-13 
