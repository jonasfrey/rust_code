#![allow(unused_parens)]
#![allow(non_camel_case_types)]

use std::fs;
use std::mem;
use std::io::Write;
use serde::{Serialize, Deserialize};
use std::env;
use serde_json::{Value};
use std::process;
use std::io::{self};
use std::path::Path;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
struct O_person_static_strings {
    s_name_pre: [u8; 20],
    s_name_last: [u8; 20],
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
struct O_person_dynamic_strings {
    s_name_pre: String,
    s_name_last: String,
}

fn main() {

    f_static_string_size();
    f_dynamic_string_size();
    // println!("Hello, world!");
}



fn f_static_string_size(){

    let s_file_name = "a_o_person_static_strings.bin";

    // println!("read as vec");
    let b_path_exists = Path::new(s_file_name).exists();
    let mut a_o_person_static_strings : Vec<O_person_static_strings> = Vec::new();

    if(b_path_exists){
        let a = fs::read(s_file_name).expect("Unable to read file");
        a_o_person_static_strings = bincode::deserialize(&a).unwrap();
    }
    let s_name_pre = String::from("hans");
    let mut s_name_pre_padded : [u8; 20] = [0;20];
    s_name_pre_padded[..s_name_pre.len()].copy_from_slice(s_name_pre.clone().as_bytes());

    let s_name_last = String::from("peter");
    let mut s_name_last_padded : [u8; 20] = [0;20];
    s_name_last_padded[..s_name_last.len()].copy_from_slice(s_name_last.clone().as_bytes());

    
    let o_person_static_strings = O_person_static_strings {
        s_name_pre: s_name_pre_padded,
        s_name_last: s_name_last_padded 
    };
    a_o_person_static_strings.push(o_person_static_strings);
    let a_o_person_static_strings_encoded: Vec<u8> = bincode::serialize(&a_o_person_static_strings).unwrap();
    fs::write(s_file_name, &a_o_person_static_strings_encoded).expect("writing not working"); // the first 8 bytes are used to store the number of elements in the vector

}

fn f_dynamic_string_size(){
    let s_file_name = "a_o_person_dynamic_strings.bin";

    // println!("read as vec");
    let b_path_exists = Path::new(s_file_name).exists();
    let mut a_o_person_dynamic_strings : Vec<O_person_dynamic_strings> = Vec::new();

    if(b_path_exists){
        let a = fs::read(s_file_name).expect("Unable to read file");
        a_o_person_dynamic_strings = bincode::deserialize(&a).unwrap();
    }

    
    let o_person_dynamic_strings = O_person_dynamic_strings {
        s_name_pre: String::from("hans"),
        s_name_last: String::from("peter") 
    };
    a_o_person_dynamic_strings.push(o_person_dynamic_strings);
    let a_o_person_dynamic_strings_encoded: Vec<u8> = bincode::serialize(&a_o_person_dynamic_strings).unwrap();
    fs::write(s_file_name, &a_o_person_dynamic_strings_encoded).expect("writing not working"); // the first 8 bytes are used to store the number of elements in the vector

}