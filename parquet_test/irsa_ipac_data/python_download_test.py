import wget
s_url = "https://irsa.ipac.caltech.edu/data/ZTF/lc/lc_dr11/0/field000262/ztf_000262_zg_c04_q1_dr11.parquet"
response = wget.download(s_url)