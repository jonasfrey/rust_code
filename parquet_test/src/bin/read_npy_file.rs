// run with 
// $ cargo run --bin read_npy_file


#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

use text_to_png::TextRenderer;

use device_query::{DeviceQuery, DeviceState, MouseState, Keycode};

use image::{ImageBuffer, Rgb};

use std::{fs, path::Path, sync::Arc};
use std::process::Command;
use parquet::record::Row;
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::file::writer::{SerializedFileWriter};
use show_image::{event, ImageView, ImageInfo, create_window};


use serde::{Deserialize, Serialize};

use serde_json::Result;

use std::{fs::File};

use parquet::{
    file::{
        properties::WriterProperties,
    },
    schema::parser::parse_message_type,
};



macro_rules! f_match_and_assign_nso_to_o_light_curve{
    // internal rule.

    (
        $s_prop_name:ident,
        $s_name:expr,
        $o_struct:expr,
        $value: ident, 
        $s_enum_name: ident, 
        $s_data_type: ident
    ) => {
        // println!($i);
        if(($s_name == stringify!($s_prop_name))){
            // println!("value {:?}", value);
            match $value {
                parquet::record::Field::$s_enum_name(val)=> {
                    let v: $s_data_type = *val;
                    $o_struct.$s_prop_name = v;
                    // std::process::exit(1);
                },
                _ => { 
                    println!("match _");
                }
            }
        }
    }
}
macro_rules! f_match_and_assign_a_to_o_light_curve{
    // internal rule.

    (
        $s_prop_name:ident,
        $s_name:expr,
        $o_struct:expr,
        $value: ident, 
        $s_enum_name: ident, 
        $s_data_type: ident
    ) => {

        if(($s_name == stringify!($s_prop_name))){
            // println!("value {:?}", value);
            match $value {
                parquet::record::Field::ListInternal(val)=> {
                    // println!("{:?}",val.elements());
                    for value in val.elements().iter(){
                        match value{
                            parquet::record::Field::$s_enum_name(val2) => {
                                let v: $s_data_type = *val2;
                                $o_struct.$s_prop_name.push(v);
                            }, 
                            _ => { 
                                println!("match _");
                            }
                        }

                    }
                }, 
                _ => { 
                    println!("match _");
                }
            }
        }

    }
}

#[derive(Serialize)]
struct O_light_curve {
    objectid: i64, //     int64 //wrong=>  int64
    filterid: i64, //     int64 //wrong=>  uint8
    fieldid: i64, //     int64 //wrong=>  int16
    rcid: i64, //     int64 //wrong=>  uint8 
    objra: f64, //     float64 //wrong=>  float32 
    objdec: f64, //     float64 //wrong=>  float32 
    nepochs: i64, //     int64 //wrong=>  int64 
    hmjd: Vec<f64>, //     float64 //wrong=>  list[float32]
    a_n_hours_modified_julian_date_estimated:Vec<f64>,
    mag: Vec<f32>, //     float32 //wrong=>  list[float32]
    a_n_magnitude_estimated:Vec<f64>,
    magerr: Vec<f32>, //     float32 //wrong=>  list[float32] 
    clrcoeff: Vec<f32>, //     float32 //wrong=>  list[float32]
    catflags: Vec<i32>, //     int32 //wrong=> list[uint16]
    n_umin_estimated: f64,
    n_tE_estimated: f64,
    n_I_estimated: f64,
    n_t_max_estimated: f64,
}
fn f_print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}



fn f_n_find_curve(
    o_light_curve: &mut O_light_curve
) -> f64 {

    let mut a_n_difference : Vec<f64> = Vec::new();
    let a_n_t = &o_light_curve.hmjd; // a_x_t
    let a_n_mag = &o_light_curve.mag; // a_y_mag

    let mut n_difference_min: Option<f64> = None;
    // println!("a_n_mag.len(): {:?}", a_n_mag.len());
    if(a_n_mag.len() > 0){
                    
        let mut n_mag_min = a_n_mag[0];
        let mut n_mag_max = a_n_mag[0];

        for n_mag in a_n_mag.iter(){
            if(*n_mag < n_mag_min){
                n_mag_min = *n_mag;
            }
            if(*n_mag > n_mag_max){
                n_mag_max = *n_mag;
            }
        }
        
        println!("n_mag_min {:?}", n_mag_min);
        println!("n_mag_max {:?}", n_mag_max);
        let n_min_tE = 1;
        let n_max_tE = (n_mag_max - n_mag_min) as u32;

        println!("n_min_tE: {:?}", n_min_tE);
        println!("n_max_tE: {:?}", n_max_tE);

        for n_count in 1..100{
            let n_umin = n_count as f32 * 0.01;
            for n_tE in n_min_tE..n_max_tE{
                for n_mag2 in 100*n_mag_min as i64 ..100*n_mag_max as i64{
                    let n_mag2_f32 = n_mag2 as f32 * 0.01;
                    
                    let n_min = (n_mag_min + (0.5*n_tE as f32))as u32; 
                    let n_max = (n_mag_max - (0.5*n_tE as f32))as u32; 
                    for n_t_max in n_min..n_max{
                        // let a_n : Vec<f64> = Vec::new();
                        let mut a_n_hours_modified_julian_date_estimated : Vec<f64>  = Vec::new();
                        let mut a_n_magnitude_estimated : Vec<f64>  = Vec::new();
                        let mut n_sum_difference_theo_data_mean = 0.0;
                        for n_index_mag in 0..a_n_mag.len(){
                            // d, umin, tE, I, t_max
                            let n_mag = a_n_mag[n_index_mag]; 
                            let n_t = a_n_t[n_index_mag];
                            let n_microlensing_theoretical = f_n_microlensing_theoretical(
                                n_t, 
                                n_umin as f64, 
                                n_tE as f64, 
                                f32::powf(10.0, (n_mag2_f32 / -2.5)) as f64,
                                n_t_max as f64
                            );
                            // println!("n_mag {:?}", n_mag);
                            // println!("n_microlensing_theoretical {:?}", n_microlensing_theoretical as f32);
                            a_n_hours_modified_julian_date_estimated.push(n_t);
                            a_n_magnitude_estimated.push(n_microlensing_theoretical);
                            n_sum_difference_theo_data_mean+=(n_microlensing_theoretical - n_mag as f64).abs();
                        }
                        
                        a_n_difference.push(n_sum_difference_theo_data_mean);

                        o_light_curve.a_n_magnitude_estimated = a_n_magnitude_estimated;
                        o_light_curve.a_n_hours_modified_julian_date_estimated = a_n_hours_modified_julian_date_estimated;
                        // println!("n_sum_difference_theo_data_mean :{:?}", n_sum_difference_theo_data_mean);

                        if(n_difference_min.is_some()){
                            if(n_sum_difference_theo_data_mean < n_difference_min.unwrap()){

                                o_light_curve.n_umin_estimated = n_umin as f64;
                                o_light_curve.n_t_max_estimated = n_t_max as f64;
                                n_difference_min = Some(n_sum_difference_theo_data_mean);
                            }
                        }else{
                            o_light_curve.n_umin_estimated = n_umin as f64;
                            o_light_curve.n_t_max_estimated = n_t_max as f64;
                            n_difference_min = Some(n_sum_difference_theo_data_mean);
                        }

                        // let n_difference_theo_data_mean = 
                    }
                }
                
            }

        }


    }

    println!("n_difference_min: {:?}", n_difference_min);
    return 0.0;
    // return n_difference_min.unwrap();
    
}
fn f_exmaple_read_single_file(
    s_path_file: &String
){

    let s_file_name = String::from(s_path_file);

    let o_path = Path::new(&s_file_name);
    let mut n_max_difference = 0.25;
    let mut a_o_light_curve__optimal_difference : Vec<O_light_curve> = Vec::new();


    if let Ok(file) = File::open(&o_path) {
        let reader = SerializedFileReader::new(file).unwrap();


        let parquet_metadata = reader.metadata();
        // assert_eq!(parquet_metadata.num_row_groups(), 1);
        println!("parquet_metadata {:?}", parquet_metadata);
        // std::process::exit(1);
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
                a_n_hours_modified_julian_date_estimated: Vec::new(),
                mag: Vec::new(), 
                a_n_magnitude_estimated: Vec::new(), 
                magerr: Vec::new(), 
                clrcoeff: Vec::new(), 
                catflags: Vec::new(),
                n_umin_estimated: 0.0,
                n_tE_estimated: 0.0,
                n_I_estimated: 0.0,
                n_t_max_estimated: 0.0,
            };

            let mut n_difference_min = 111111.0;


            for (idx, (s_name, value)) in o_row.get_column_iter().enumerate() {
                // println!("column index: {}, column name: {}, column value: {}", idx, s_name, value);
                // // println!()

                f_match_and_assign_nso_to_o_light_curve!(objectid, s_name, o_light_curve, value, Long, i64);
                f_match_and_assign_nso_to_o_light_curve!(filterid, s_name, o_light_curve, value, Long, i64);
                f_match_and_assign_nso_to_o_light_curve!(fieldid, s_name, o_light_curve, value, Long, i64);
                f_match_and_assign_nso_to_o_light_curve!(rcid, s_name, o_light_curve, value, Long, i64);
                f_match_and_assign_nso_to_o_light_curve!(objra, s_name, o_light_curve, value, Double, f64);
                f_match_and_assign_nso_to_o_light_curve!(objdec, s_name, o_light_curve, value, Double, f64);
                f_match_and_assign_nso_to_o_light_curve!(nepochs, s_name, o_light_curve, value, Long, i64);
                // arrays
                f_match_and_assign_a_to_o_light_curve!(hmjd, s_name, o_light_curve, value, Double, f64);
                f_match_and_assign_a_to_o_light_curve!(mag, s_name, o_light_curve, value, Float, f32);
                f_match_and_assign_a_to_o_light_curve!(magerr, s_name, o_light_curve, value, Float, f32);
                f_match_and_assign_a_to_o_light_curve!(clrcoeff, s_name, o_light_curve, value, Float, f32);
                f_match_and_assign_a_to_o_light_curve!(catflags, s_name, o_light_curve, value, Int, i32);

                // println!("o_light_curve.objectid {:?}", o_light_curve.objectid);

                

                // # Limits (numbers) - parameters beyond these are not sensible
                // let min_mag = a_y_mag.iter().min();
                // match min_mag {
                //     Some(min) => println!( "Min value: {}", min ),
                //     None      => println!( "Vector is empty" ),
                // }
                let n_difference_min = f_n_find_curve(
                    &mut o_light_curve
                );




            }

            if(n_difference_min < n_max_difference){
                println!("n_difference_min {:?}",n_difference_min);
                a_o_light_curve__optimal_difference.push(o_light_curve);
            }
            else{
                a_o_light_curve__optimal_difference.push(o_light_curve);
            }

            n_index+=1;
        }

    }
    let json = serde_json::to_string(&a_o_light_curve__optimal_difference).unwrap();
    // let mut file = File::create("test.json").unwrap();
    // file.write_all(&json).unwrap();


    std::fs::write(
        "./optimal_difference.json",
        serde_json::to_string(&a_o_light_curve__optimal_difference).unwrap()
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

fn f_convert_npy_to_parquet_and_create_json(){
    
    let s_path_file_original = "ztf_000722_zr_c07_q4_dr11.parquet_filtered.npy";
    let s_path_file_npy = String::from(s_path_file_original.clone());
    let mut s_path_file_parquet = String::from(s_path_file_npy.clone()); 
    let mut s_extension_parquet = String::from(".parquet");
    s_path_file_parquet.push_str(&mut s_extension_parquet);

    f_convert_npy_to_parquet(&s_path_file_npy);
    // println!("done first!");
    f_exmaple_read_single_file(&s_path_file_parquet);
}

#[show_image::main]
fn main() {

    let a_s_arg: Vec<String> = std::env::args().collect();
    let mut b_function_called = false;
    if(a_s_arg.len() == 1){
        f_convert_npy_to_parquet_and_create_json();
    }else{
        if(
            a_s_arg[1] == String::from("f_convert_npy_to_parquet_and_create_json")
        ){
            f_convert_npy_to_parquet_and_create_json();
        }
        if(a_s_arg[1] == String::from("f_animate_microlensing_theoretical")){
            f_animate_microlensing_theoretical();
        }
        if(a_s_arg[1] == String::from("f_test_manual")){
            f_test_manual();
        }
    }
    println!("a_s_arg {:?}", a_s_arg);

}

// def ML_theo(d, umin, tE, I, t_max): #theoretische ML-Funktion, input = time-array, output = mag-array
//     #umin => between 0 and 1 - the smaller, the bigger the amplitude
//     #tE => duration of Event - the bigger, the wider the curve
//     #I => intensity I = (light intensity)/Area of star without amplification
//     #t_max => time when amplification A of I reaches maximum a
//     u = np.sqrt(umin**2 + ((d-t_max)/tE)**2)
//     if u*np.sqrt(u**2 + 4) != 0:
//         A = I*((u**2 + 2) / (u*np.sqrt(u**2 + 4))) 
//         M = -2.5*np.log10(A) # conversion to magnitude
//     else: M = -10 #if umin = 0 and d = t0, amplitude theoretically becomes infinite 
//     return M 

fn f_n_microlensing_theoretical(
    n_d: f64,
    n_umin: f64,
    n_tE: f64,
    n_I: f64,
    n_t_max: f64,
)->f64{
    let n_static = -2.5;
    let n_u = (f64::powf(n_umin,2.0) + (f64::powf((n_d-n_t_max)/n_tE, 2.0))).sqrt();
    let n_u2 = n_u*(f64::powf(n_u,2.0) + 4.0).sqrt();
    if(n_u2 != 0.0){
        let n_A = n_I*((f64::powf(n_u, 2.0)+ 2.0) / n_u2);
        let n_M = n_static * n_A.log10();
        return n_M
    }else{
        return -10.0
    }
}

fn f_u32__color_rgba_mixed(
    src: u32, 
    dst: u32, 
    t: u32, 
    // uint32_t src, uint32_t dst, uint32_t t
)-> u32{
    // assert(t <= 255);
    let s: u32 = 255 - t;
    let n_mixed : u32 = (
        (((((src >> 0)  & 0xff) * s +
           ((dst >> 0)  & 0xff) * t) >> 8)) |
        (((((src >> 8)  & 0xff) * s +
           ((dst >> 8)  & 0xff) * t)     )  & !0xff) |
        (((((src >> 16) & 0xff) * s +
           ((dst >> 16) & 0xff) * t) << 8)  & !0xffff) |
        (((((src >> 24) & 0xff) * s +
           ((dst >> 24) & 0xff) * t) << 16) & !0xffffff)
    );
    // println!(" n_mixed  le bytes {:?} ", n_mixed.to_le_bytes());
    // println!(" n_mixed  be bytes {:?} ", n_mixed.to_be_bytes());
    return n_mixed
}

fn f_a_n_u8__color_rgba_mixed_fast2(
    n_r_1: u8,
    n_g_1: u8,
    n_b_1: u8,
    n_a_1: u8,
    n_r_2: u8,
    n_g_2: u8,
    n_b_2: u8,
    n_a_2: u8,
)->Vec<u8>{
    let mut n_u32_colora: u32 = 0; 
    n_u32_colora = n_u32_colora | (n_a_1 as u32) << 8*0;
    n_u32_colora = n_u32_colora | (n_b_1 as u32) << 8*1;
    n_u32_colora = n_u32_colora | (n_g_1 as u32) << 8*2;
    n_u32_colora = n_u32_colora | (n_r_1 as u32) << 8*3;
    let mut n_u32_colorb: u32 = 0; 
    n_u32_colorb = n_u32_colorb | (n_a_2 as u32) << 8*0;
    n_u32_colorb = n_u32_colorb | (n_b_2 as u32) << 8*1;
    n_u32_colorb = n_u32_colorb | (n_g_2 as u32) << 8*2;
    n_u32_colorb = n_u32_colorb | (n_r_2 as u32) << 8*3;
    // println!("n_u32_colora {:?}", n_u32_colora);
    // println!("n_u32_colorb {:?}", n_u32_colorb);
    let n_u32_color_result = f_u32__color_rgba_mixed(
        n_u32_colora,
        n_u32_colorb,
        n_a_2 as u32
    );
    // println!("n_u32_color_result {:?}", n_u32_color_result);

    return vec![
        ((n_u32_color_result >> 8*3) & 255) as u8,
        ((n_u32_color_result >> 8*2) & 255) as u8,
        ((n_u32_color_result >> 8*1) & 255) as u8,
        ((n_u32_color_result >> 8*0) & 255) as u8,
    ]

}
fn f_a_n_u8__color_rgba_mixed_fast(
    n_r_1: u8, //rA
    n_g_1: u8, //gA
    n_b_1: u8, //bA
    n_a_1: u8, //n_a_1
    n_r_2: u8, //rB
    n_g_2: u8, //gB
    n_b_2: u8, //bB
    n_a_2: u8, //aB
)-> Vec<u8>{
    let mut n_u32_colora: u32 = 0; 
    n_u32_colora = (n_a_1 as u32) << 8*3;
    n_u32_colora = (n_b_1 as u32) << 8*2;
    n_u32_colora = (n_g_1 as u32) << 8*1;
    n_u32_colora = (n_r_1 as u32) << 8*0;
    let mut n_u32_colorb: u32 = 0; 
    n_u32_colorb = (n_a_2 as u32) << 8*3;
    n_u32_colorb = (n_b_2 as u32) << 8*2;
    n_u32_colorb = (n_g_2 as u32) << 8*1;
    n_u32_colorb = (n_r_2 as u32) << 8*0;
    
    let n_u32_alpha : u32 = (((2 as i32).pow(32)) as f64 /2.0) as u32;

    let rb1: u32 = ((0x100 - n_u32_alpha) * (n_u32_colora & 0xFF00FF)) >> 8;
    let rb2: u32 = (n_u32_alpha * (n_u32_colorb & 0xFF00FF)) >> 8;
    let g1: u32  = ((0x100 - n_u32_alpha) * (n_u32_colora & 0x00FF00)) >> 8;
    let g2: u32  = (n_u32_alpha * (n_u32_colorb & 0x00FF00)) >> 8;
    let n_u32_result = ((rb1 | rb2) & 0xFF00FF) + ((g1 | g2) & 0x00FF00);
    // println!("n_u32_result {:?}", n_u32_result);
    return vec![
        ((n_u32_result >> 8*0) & 255) as u8,
        ((n_u32_result >> 8*1) & 255) as u8,
        ((n_u32_result >> 8*2) & 255) as u8,
        ((n_u32_result >> 8*3) & 255) as u8,
    ]
}
fn f_a_n_u8__color_rgba_mixed(

    n_r_1: f32, //rA
    n_g_1: f32, //gA
    n_b_1: f32, //bA
    n_a_1: f32, //n_a_1
    n_r_2: f32, //rB
    n_g_2: f32, //gB
    n_b_2: f32, //bB
    n_a_2: f32, //aB

) -> Vec<u8>{


    let n_max : f32 = 255.0;

    // let n_r_mixed = (n_r_1 * n_a_1 / n_max) + (n_r_2 * n_b_2 * (n_max - n_a_1) / (n_max*n_max));
    // let n_g_mixed = (n_g_1 * n_a_1 / n_max) + (n_g_2 * n_b_2 * (n_max - n_a_1) / (n_max*n_max));
    // let n_b_mixed = (n_b_1 * n_a_1 / n_max) + (n_b_2 * n_b_2 * (n_max - n_a_1) / (n_max*n_max));
    // let n_a_mixed = n_a_1 + (n_b_2 * (n_max - n_a_1) / n_max);

    let n_a_mixed = n_a_1 + (n_a_2 * (n_max - n_a_1) / n_max);
    let n_r_mixed = (n_r_1 * n_a_1 + n_r_2 * n_a_2 * (n_max - n_a_1) / n_max)/n_a_mixed ;
    let n_g_mixed = (n_g_1 * n_a_1 + n_g_2 * n_a_2 * (n_max - n_a_1) / n_max)/n_a_mixed ;
    let n_b_mixed = (n_b_1 * n_a_1 + n_b_2 * n_a_2 * (n_max - n_a_1) / n_max)/n_a_mixed ;


    return vec![
        n_r_mixed as u8,
        n_g_mixed as u8,
        n_b_mixed as u8,
        n_a_mixed as u8
    ]
    // return a_color_rgba_mixed;
}

fn f_write_from_o_text_to_png_text_png(
    a_n_u8__image: &mut Vec<u8>,
    n_image_scale_x : u32,
    n_image_scale_y : u32,
    n_image_channels: u32,
    n_rect_translation_x : u32,
    n_rect_translation_y : u32,
    o_text_to_png_text_png: text_to_png::TextPng,
){

    let n_rect_scale_x = o_text_to_png_text_png.size.width;
    let n_rect_scale_y = o_text_to_png_text_png.size.height;
    println!("o_text_to_png_text_png {:?}", o_text_to_png_text_png);
    println!("o_text_to_png_text_png {:?}", o_text_to_png_text_png.data.len());

    let a_n_u8__o_text_to_png_text_png = o_text_to_png_text_png.data;
    let mut n_rgba_value = 0;
    let mut n_pixel_index = 0;
    let mut n_x = 0;
    let mut n_x_on_image = 0;
    let mut n_y = 0;
    let mut n_y_on_image = 0;
    let mut n_channel = 0;
    
    let n_index_max = a_n_u8__image.len()-1;

    while(n_y < n_rect_scale_y){
        
        n_y_on_image = n_y + n_rect_translation_y;
        if(n_y_on_image < 0 || n_y_on_image > n_image_scale_y-1){
            n_y+=1;
            continue;
        }
        n_x = 0;
        while(n_x < n_rect_scale_x){
            n_x_on_image = n_x + n_rect_translation_x;
            if(n_x_on_image < 0 || n_x_on_image > n_image_scale_x-1){
                n_x+=1;
                continue;
            }
            let n_index_pixel = 
                ((n_y_on_image) * n_image_scale_x * n_image_channels) +
                ((n_x_on_image) * n_image_channels);

            let n_index_pixel_on_a_n_u8__o_text_to_png_text_png = 
                ((n_y) * n_rect_scale_x * n_image_channels) +
                ((n_x) * n_image_channels);


            n_channel = 0;
            while(n_channel < n_image_channels){

                let n_index = (n_index_pixel+n_channel);
                a_n_u8__image[n_index as usize] = 
                    a_n_u8__o_text_to_png_text_png[(n_index_pixel_on_a_n_u8__o_text_to_png_text_png+n_channel) as usize];
                n_channel+=1;
            }
            
            n_x+=1;
        }
        
        n_y+=1;
    }

}
    

fn f_write_color(
    a_n_u8__image: &mut Vec<u8>,
    n_image_scale_x : u32,
    n_image_scale_y : u32,
    n_image_channels: u32,
    n_rect_translation_x : u32,
    n_rect_translation_y : u32,
    n_rect_scale_x : u32,
    n_rect_scale_y : u32,
    a_n_u8__color : &Vec<u8>
){

    // let mut a_vec: Vec<u8> = Vec::new();

    let mut n_rgba_value = 0;
    let mut n_pixel_index = 0;
    let mut n_x = 0;
    let mut n_x_on_image = 0;
    let mut n_y = 0;
    let mut n_y_on_image = 0;
    let mut n_channel = 0;
    
    let n_index_max = a_n_u8__image.len()-1;

    while(n_y < n_rect_scale_y){
        
        n_y_on_image = n_y + n_rect_translation_y;
        if(n_y_on_image < 0 || n_y_on_image > n_image_scale_y-1){
            n_y+=1;
            continue;
        }
        n_x = 0;
        while(n_x < n_rect_scale_x){
            n_x_on_image = n_x + n_rect_translation_x;
            if(n_x_on_image < 0 || n_x_on_image > n_image_scale_x-1){
                n_x+=1;
                continue;
            }
            let n_index_pixel = 
                ((n_y_on_image) * n_image_scale_x * n_image_channels) +
                ((n_x_on_image) * n_image_channels);
            if(a_n_u8__color[3] == 0){
                n_x+=1;
                continue;
            }

            let a_n_u8__color_mixed = &f_a_n_u8__color_rgba_mixed_fast2(
                a_n_u8__image[(n_index_pixel+0)as usize],
                a_n_u8__image[(n_index_pixel+1)as usize],
                a_n_u8__image[(n_index_pixel+2)as usize],
                a_n_u8__image[(n_index_pixel+3)as usize],
                a_n_u8__color[0],
                a_n_u8__color[1],
                a_n_u8__color[2],
                a_n_u8__color[3],
            );
            // println!("a_n_u8__color {:?}", a_n_u8__color);
            // println!("a_n_u8__color_image {:?}", 
            //     vec![
            //         a_n_u8__image[(n_index_pixel+0)as usize],
            //         a_n_u8__image[(n_index_pixel+1)as usize],
            //         a_n_u8__image[(n_index_pixel+2)as usize],
            //         a_n_u8__image[(n_index_pixel+3)as usize],
            //     ]    
            // );
            // println!("a_n_u8__color_mixed {:?}", a_n_u8__color_mixed);

            n_channel = 0;
            while(n_channel < n_image_channels){
                let n_index = (n_index_pixel+n_channel);
                a_n_u8__image[n_index as usize] = a_n_u8__color_mixed[n_channel as usize];
                n_channel+=1;
            }
                // a_vec.push(a_n_u8__image[n_index as usize]);
            
            n_x+=1;
        }
            // println!("x|y {:?}|{:?}", n_x, n_y);
        
        n_y+=1;
        // println!("x|y {:?}|{:?}", n_x, n_y);
    }

    // return a_vec
}

fn f_test_manual(){
    let o_file = fs::File::open("./artificial_test_data.json")
    .expect("file should open read only");
    let o_json: serde_json::Value = serde_json::from_reader(o_file)
        .expect("file should be proper JSON");
    let a_o = o_json.as_array().unwrap();
    let mut a_o_light_curve__artificial_test_data : Vec<O_light_curve> = Vec::new();
    for o in a_o{
        
        let mut o_light_curve = O_light_curve {
            objectid: 0,
            filterid: 0,
            fieldid: 0,
            rcid: 0, 
            objra: 0.0, 
            objdec: 0.0, 
            nepochs: 0, 
            hmjd: Vec::new(),
            a_n_hours_modified_julian_date_estimated: Vec::new(),
            mag: Vec::new(), 
            a_n_magnitude_estimated: Vec::new(), 
            magerr: Vec::new(), 
            clrcoeff: Vec::new(), 
            catflags: Vec::new(),
            n_umin_estimated: 0.0,
            n_tE_estimated: 0.0,
            n_I_estimated: 0.0,
            n_t_max_estimated: 0.0,
        };
    
        // let a_n_t = &o_light_curve.hmjd; // a_x_t
        // let a_n_mag = &o_light_curve.mag; // a_y_mag
        for n_time in o["a_n_time"].as_array().unwrap().iter(){
            o_light_curve.hmjd.push(
                n_time.as_i64().unwrap() as f64
            )
        }
        for n_magnitude in o["a_n_magnitude"].as_array().unwrap().iter(){
            o_light_curve.mag.push(
                n_magnitude.as_f64().unwrap() as f32
            )
        }

        let n_difference_min = f_n_find_curve(
            &mut o_light_curve
        );

        a_o_light_curve__artificial_test_data.push(
            o_light_curve
        );

        // println!("o {:?}", o);
    }

    let json = serde_json::to_string(&a_o_light_curve__artificial_test_data).unwrap();
    // let mut file = File::create("test.json").unwrap();
    // file.write_all(&json).unwrap();


    std::fs::write(
        "./a_o_light_curve__artificial_test_data.json",
        serde_json::to_string(&a_o_light_curve__artificial_test_data).unwrap()
    ).unwrap();

}

fn f_o_text_to_png_text_png(
    s_text: &String, 
    a_n_u8__color: &Vec<u8>, 
    n_font_size: u32
)-> text_to_png::TextPng{
    let o_renderer = Some(TextRenderer::default());

    let o_text_png_color = text_to_png::Color::new(
        (a_n_u8__color[0 as usize]) as u8,
        (a_n_u8__color[1 as usize]) as u8,
        (a_n_u8__color[2 as usize]) as u8,
    );
    let o_text_png = o_renderer.clone().unwrap().render_text_to_png_data(
        s_text.as_str(),
        n_font_size,
        o_text_png_color
    ).unwrap();

    // let a_n_u8__text: Vec<u8> = o_text_png.data;
    return o_text_png;

}

fn f_animate_microlensing_theoretical(){

    let n_scale_x = 512.0;
    // let n_scale_x = 100.0;
    let n_scale_y = 512.0;
    // let n_scale_y = 100.0;
    let n_pixel_channels = 4;

    let mut a_n_u8__image: Vec<u8> = 
    vec![
        255;
        (
            n_scale_x *  
            n_scale_y * 
            (n_pixel_channels as f64)
        ) as usize
    ];

    let s_name = String::from("microlensing curve theoretical, press space and move mouse!");
    let o_window = create_window(s_name.clone(), Default::default()).unwrap();
    let mut o_image = ImageView::new(
        ImageInfo::rgba8(
            n_scale_x as u32,  
            n_scale_y as u32
        ),
        &a_n_u8__image
    );
    let a_n_u8__color = vec![255,0,0,255];
    let a_n_u8__color_white = vec![255,255,255,255];
    
    let o_device_state = DeviceState::new();


    let a_n_u8__color_clear = vec![0,0,255,1];
    let mut n_time : u64 = 0;

    let mut a_n_param = vec![
        String::from("n_umin"),
        String::from("n_tE"),
        String::from("n_I"),
        String::from("n_t_max"),
    ];

    let mut n_index_a_n_param = 0;

    let mut b_space_down = false; 
    let mut b_space_down_last = false; 

    let n_screen_scale_x = 1920.0;
    let n_screen_scale_y = 1080.0;
    let n_screen_translation_x = 0.0;
    let n_screen_translation_y = 1080.0;

    let mut n_umin = 1.0;
    let mut n_tE = 1.0;
    let mut n_I = 1.0;
    let mut n_t_max = 1.0;
    
    while(true){
        n_time +=1;
        let o_mouse_state: MouseState = o_device_state.get_mouse();
        // println!("Current Mouse Coordinates: {:?}", o_mouse_state.coords);

        let n_mouse_x_normalized = ((o_mouse_state.coords.0 as f64) - n_screen_translation_x) / n_screen_scale_x;
        let n_mouse_y_normalized = ((o_mouse_state.coords.1 as f64) - n_screen_translation_y) / n_screen_scale_y;
        // println!("n_mouse_normalized x|y {:?}|{:?}", n_mouse_x_normalized, n_mouse_y_normalized);

        let a_o_keycode: Vec<Keycode> = o_device_state.get_keys();
        if(a_o_keycode.contains(&Keycode::Space)){
            b_space_down = true;
        }else{
            b_space_down = false;
        }
        if(b_space_down && b_space_down_last == false){
            n_index_a_n_param = (n_index_a_n_param +1) % a_n_param.len(); 
        }
        f_write_color(
            &mut a_n_u8__image,
            n_scale_x as u32,
            n_scale_y as u32,
            n_pixel_channels,
            0,
            0,
            n_scale_x as u32,
            n_scale_y as u32,
            &a_n_u8__color_clear
       );

        //    let o_text_to_png_text_png = f_o_text_to_png_text_png(
        //     &String::from("text"),
        //     &a_n_u8__color_white,
        //     10
        //    );
        //    f_write_from_o_text_to_png_text_png(
        //     &mut a_n_u8__image,
        //     n_scale_x as u32,
        //     n_scale_y as u32,
        //     n_pixel_channels,
        //     0,
        //     0,
        //     o_text_to_png_text_png,
        //    );
        
        let mut n_x = 0;
        let n_x_max = 500;
        let mut n_umin_amplitude = 0.2;
        let mut n_tE_duration_event = 0.2;
        let mut n_t_max = 1.0;



        println!("a_n_param[n_index_a_n_param] {:?}", &a_n_param[n_index_a_n_param]);

        if(a_n_param[n_index_a_n_param]  == String::from("n_umin")){
            n_umin = 1.0 * n_mouse_x_normalized;
            println!("n_umin : {:?}", n_umin);
        }
        if(a_n_param[n_index_a_n_param]  == String::from("n_tE")){
            n_tE = 1000.0 * n_mouse_x_normalized;
            println!("n_tE : {:?}", n_tE);
        }
        if(a_n_param[n_index_a_n_param]  == String::from("n_I")){
            n_I = 1.0 * n_mouse_x_normalized;
            println!("n_I : {:?}", n_I);
        }
        if(a_n_param[n_index_a_n_param]  == String::from("n_t_max")){
            n_t_max = n_x_max as f64 * n_mouse_x_normalized;
            println!("n_t_max : {:?}", n_t_max);
        }


       while(n_x < n_x_max){
        
            // println!("n_umin_amplitude {:?}",n_umin_amplitude);
            // println!("n_tE_duration_event {:?}",n_tE_duration_event);
            
            // f_theo(o,1,5,1,200,1)*500
            let n_y = f_n_microlensing_theoretical(
                n_x as f64, 
                n_umin,
                n_tE,
                n_I,
                n_t_max
            ) * 200.0;

            // let n_y = f_microlensing_theoretical(
            //     n_x as f64,//n_d_time: f64,
            //     n_umin_amplitude,//n_umin_amplitude: f64, 
            //     n_tE_duration_event,//n_tE_duration_event: f64, 
            //     n_t_max as f64//n_t_max: f64, 
            // );
            // println!("n_y{:?}",n_y);
            n_x+=1;

            f_write_color(
                &mut a_n_u8__image,
                n_scale_x as u32,
                n_scale_y as u32,
                n_pixel_channels,
                //  o_mouse_state.coords.0.try_into().unwrap(),
                //  o_mouse_state.coords.1.try_into().unwrap(),
                (n_x as f64 + (n_scale_x /2.0)) as u32,
                (n_y + (n_scale_x /2.0)) as u32,
                3,
                3,
                &a_n_u8__color
            );
       }

        // f_write_color(
        //      &mut a_n_u8__image,
        //      n_scale_x as u32,
        //      n_scale_y as u32,
        //      n_pixel_channels,
        //     //  o_mouse_state.coords.0.try_into().unwrap(),
        //     //  o_mouse_state.coords.1.try_into().unwrap(),
        //     (n_time % n_scale_x as u64) as u32,
        //     (n_time % n_scale_y as u64) as u32,
        //      20,
        //      20,
        //      &a_n_u8__color
        // );


        o_image = ImageView::new(
            ImageInfo::rgba8(
                n_scale_x as u32,  
                n_scale_y as u32
            ),
            &a_n_u8__image
        );
        
        o_window.set_image(s_name.clone(), o_image).unwrap();
        b_space_down_last = b_space_down;
    }
}