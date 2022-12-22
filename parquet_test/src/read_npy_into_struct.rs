// #[macro_use]
// extern crate npy_derive;
// extern crate npy;

// use std::io::Read;
// use npy::NpyData;


// use std::io::Read;
// use npy::NpyData;

// // #[derive(Serializable, Debug)]
// struct Array {
//     objectid: u64,
//     filterid: u64,
//     fieldid: u64,
//     rcid: u64, 
//     objra: f64, 
//     objdec: f64, 
//     nepochs: u64, 
//     hmjd: Vec<f64>,
//     mag: Vec<f32>, 
//     magerr: Vec<f32>, 
//     clrcoeff: Vec<f32>, 
//     catflags: Vec<u32>
// }

// fn main() {
//     let mut buf = vec![];
//     std::fs::File::open("./ztf_000722_zr_c07_q4_dr11.parquet_filtered.npy").unwrap()
//         .read_to_end(&mut buf).unwrap();

//     let data: NpyData<Array> = NpyData::from_bytes(&buf).unwrap();
//     for arr in data {
//         eprintln!("{:?}", arr);
//     }
// }

// use ndarray::array;
// use ndarray_npy::write_npy;
// fn main() {

//     let arr = array![[1, 2, 3], [4, 5, 6]];
//     write_npy("array.npy", &arr).unwrap();

// }