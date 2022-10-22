// run with 
// $ cargo run --bin read_npy_file
use std::{fs, path::Path, sync::Arc};
use std::process::Command;
use parquet::record::Row;
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::file::writer::{SerializedFileWriter};


use serde::{Deserialize, Serialize};

use serde_json::Result;

use std::{fs::File};

use parquet::{
    file::{
        properties::WriterProperties,
    },
    schema::parser::parse_message_type,
};

#[derive(Serialize)]
struct O_light_curve {
    objectid: i64,
    filterid: u64,
    fieldid: u64,
    rcid: u64, 
    objra: f64, 
    objdec: f64, 
    nepochs: u64, 
    hmjd: Vec<f64>,
    mag: Vec<f32>, 
    magerr: Vec<f32>, 
    clrcoeff: Vec<f32>, 
    catflags: Vec<u32>
}
fn f_print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn f_exmaple_read_single_file(
    s_path_file: &String
){

    let s_file_name = String::from(s_path_file);

    let o_path = Path::new(&s_file_name);

    let mut a_o_light_curve : Vec<O_light_curve> = Vec::new();


    if let Ok(file) = File::open(&o_path) {
        let reader = SerializedFileReader::new(file).unwrap();


        let parquet_metadata = reader.metadata();
        // assert_eq!(parquet_metadata.num_row_groups(), 1);
        println!("parquet_metadata {:?}", parquet_metadata);
        // std::process::exit(1);
        // println!("parquet_metadata.file_metadata().num_rows() {:?}", parquet_metadata.file_metadata().num_rows());
        let row_group_reader = reader.get_row_group(0).unwrap();
        // assert_eq!(row_group_reader.num_columns(), 1);
        println!("row_group_reader.num_columns() {:?}", row_group_reader.num_columns());
        let mut row_iterator = row_group_reader.get_row_iter(None).unwrap();

        let mut b_is_none = false; 
        let mut n_index = 0; 
        while(b_is_none == false){
            let o_row_option = row_iterator.next();
            if(o_row_option.is_none()){
                break;
            }
            let o_row = o_row_option.unwrap();
            println!("n_index {:?}", n_index);
            let mut o_light_curve = O_light_curve {
                objectid: 0,
                filterid: 0,
                fieldid: 0,
                rcid: 0, 
                objra: 0.0, 
                objdec: 0.0, 
                nepochs: 0, 
                hmjd: Vec::new(),
                mag: Vec::new(), 
                magerr: Vec::new(), 
                clrcoeff: Vec::new(), 
                catflags: Vec::new()
            };

            for (idx, (s_name, value)) in o_row.get_column_iter().enumerate() {
                // println!("column index: {}, column name: {}, column value: {}", idx, s_name, value);
                // println!()
                if((s_name == "objectid")){
                    println!("value {:?}", value);
                    match value {
                        parquet::record::Field::Long(val)=> {
                            let test: i64 = *val;
                            o_light_curve.objectid = test;
                            // std::process::exit(1);
                        }, 
                        _ => { 
                            println!("match _");
                        }
                    }
                    // match value {
                    //     parquet::record::Field(_, value) => println!("value: {}", value),
                    //     _ => println!("Something else"),
                    // }
                    // let mut n_i64: i64 = value;
                    // println!("val {:?}", parquet::record::Field::convert_int64(value, n_i64));
                    // println!("val {:?}", value);
                    // println!("f_print_type_of {:?}", f_print_type_of(value));
                    // o_light_curve.objectid = *&value.convert_int64();
                }
                // std::process::exit(1);

                // println!("column index: {}, column name: {}, column value: {}", idx, name, field);
            }
            println!("{:?}", o_light_curve.objectid);
            a_o_light_curve.push(o_light_curve);

            n_index+=1;
        }

    }
    let json = serde_json::to_string(&a_o_light_curve).unwrap();
    // let mut file = File::create("test.json").unwrap();
    // file.write_all(&json).unwrap();


    std::fs::write(
        "./test.json",
        serde_json::to_string(&a_o_light_curve).unwrap()
    ).unwrap();

    // let mut output = File::create("./test.json").unwrap();
    // write!(output, "{:?}", output).unwrap();

    // if let Ok(file) = File::open(&o_path) {
    //     let reader = SerializedFileReader::new(file).unwrap();
    //     let o_path = Path::new("output_parquet.parquet");
    //     let o_output_file = File::create(&o_path).unwrap();
    //     let o_props = Arc::new(WriterProperties::builder().build());
    //     let o_schema: &parquet::schema::types::Type = reader.metadata().file_metadata().schema();
    //     let o_arc_schema = Arc::new(o_schema.to_owned());
    //     let mut writer = SerializedFileWriter::new(
    //         o_output_file,
    //         o_arc_schema,
    //         o_props
    //     ).unwrap();
        
    //     let mut row_group_writer = writer.next_row_group().unwrap();
    //     while let Some(mut col_writer) = row_group_writer.next_column().unwrap() {
    //         // ... write values to a column writer

    //         col_writer.close().unwrap()
    //     }
    //     row_group_writer.close().unwrap();
    //     writer.close().unwrap();
    // }



}
fn f_example_read_multiple_files(){

    let paths = vec![
        "ztf_000468_zi_c16_q4_dr7.parquet",
    ];
    // Create a reader for each file and flat map rows
    let rows = paths.iter()
        .map(|p| SerializedFileReader::try_from(*p).unwrap())
        .flat_map(|r| r.into_iter());

    for row in rows {
        f_print_type_of(&row);

        for (n_index, (s_prop_name, value)) in row.get_column_iter().enumerate() {

            // if(s_prop_name == "catflags"){
            //     if(value >= 1){
            //         //....
            //     }
            // }
            println!("n_index: {:?}", n_index);
            println!("s_prop_name: {:?}", s_prop_name);
            println!("value: {:?}", value);
        }
        // println!("{}", row);
        // println!("{}", row["objectid"]);
    }
}

fn f_example_read_and_write(){
    
    let paths = vec![
        "ztf_000468_zi_c16_q4_dr7.parquet",
    ];
    // Create a reader for each file and flat map rows
    let rows = paths.iter()
        .map(|p| SerializedFileReader::try_from(*p).unwrap())
        .flat_map(|r| r.into_iter());

    for row in rows {
        println!("row: {:?}", row);
        // 
        // f_print_type_of(&row);
// 
        for (n_index, (s_prop_name, value)) in row.get_column_iter().enumerate() {

            // s_prop_name "nepochs"
            // value 1234
            // n_index 6

            println!("n_index: {:?}", n_index);
            println!("s_prop_name: {:?}", s_prop_name);
            println!("value: {:?}", value);
            //
        }
        // println!("{}", row);
        // println!("{}", row["objectid"]);
    }
}

fn f_convert_npy_to_parquet(
    s_path_file_npy: &String
){
    //python3 convert_numpy_to_parquet.py ztf_000722_zr_c07_q4_dr11.parquet_filtered.npy

    let output = Command::new("python3")
        .args(["convert_numpy_to_parquet.py", s_path_file_npy])
        .output()
        .expect("failed to execute process");
    
    let s_output = output.stdout;
    
    println!("s_output {:?}", s_output);

}
fn main() {
    let s_path_file_original = "ztf_000722_zr_c07_q4_dr11.parquet_filtered.npy";
    let s_path_file_npy = String::from(s_path_file_original.clone());
    let mut s_path_file_parquet = String::from(s_path_file_npy.clone()); 
    let mut s_extension_parquet = String::from(".parquet");
    s_path_file_parquet.push_str(&mut s_extension_parquet);

    f_convert_npy_to_parquet(&s_path_file_npy);
    // println!("done first!");
    f_exmaple_read_single_file(&s_path_file_parquet);
    // f_example_read_multiple_files();

}

