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
struct O_person {
    n_id: u32, // if type == number
    s_name: String,
    b_male: bool,
    n_age: f32,
}

static s_object_name: &'static str = "O_person"; // == model name , 'A_'+..== table name /file name
static s_file_name: &'static str = "./A_o_person"; // 
// call it like 
// cargo run '{"object": {"s_name": "jonas"}, "s_function_name": "f_a_o_create"}'
fn main(){

    // let a_s_argument: Vec<String> = env::args().collect();
    // if(a_s_argument.len() != 2 ){
    //     eprintln!(r#"a JSON string in format '{{"s_function_name": "...", "object":{{...}}}}' must be passed as the first argument"#);
    //     process::exit(1);
    // }
    // let s_first_arg = a_s_argument[1].to_owned();
    // // println!("a_s_argument {:?}", a_s_argument);

    // let v: Value = serde_json::from_str(&s_first_arg).unwrap();



    // println!("read as vec");
    let b_path_exists = Path::new(s_file_name).exists();
    let mut a_o_person : Vec<O_person> = Vec::new();

    if(b_path_exists){
        let a = fs::read(s_file_name).expect("Unable to read file");
        a_o_person = bincode::deserialize(&a).unwrap();
        let s_a_o_person = serde_json::to_string(&a_o_person).unwrap();
        println!("s_a_o_person {:?}", s_a_o_person);
    }
    let mut b_write = true;

    a_o_person.push(
        O_person{
            n_id: 0,
            s_name: String::from("hans ludolf frinz von steinenberg"),
            b_male: true,
            n_age: 55.55,
        }
    );
    a_o_person.push(
        O_person{
            n_id: 2,
            s_name: String::from("lorenz alfsson von braunstein"),
            b_male: true,
            n_age: 58.22,
        }
    );
    a_o_person.push(
        O_person{
            n_id: 1,
            s_name: String::from("miranda manuela magdalene von maldemart"),
            b_male: false,
            n_age: 33.33,
        }
    );
    let o_person_with_long_float =        O_person{
        n_id: 1,
        s_name: String::from("this string is just 65 bytes long,,,,,,,,,,,,,,,,,,,,,,,,,,,,,yes"),
        b_male: false,
        n_age: 33.3300018,
    };
    println!("o_person_with_long_float.n_age: {:?}",&o_person_with_long_float.n_age);
    a_o_person.push(
        o_person_with_long_float
    );
    let mut a_o_person_return = a_o_person.clone();
    
    if(b_write){
        let a_o_person_encoded: Vec<u8> = bincode::serialize(&a_o_person).unwrap();
        // println!("writing file");
        fs::write(s_file_name, &a_o_person_encoded).expect("writing not working"); // the first 8 bytes are used to store the number of elements in the vector
    }
    
    let a_o_person_return_encoded: Vec<u8> = bincode::serialize(&a_o_person_return).unwrap();
    
    // let s_file_name_function_return = format!("{}_{}", s_file_name, v["s_function_name"].as_str().unwrap()); 
    // fs::write(s_file_name_function_return, &a_o_person_return_encoded).expect("writing not working"); // the first 8 bytes are used to store the number of elements in the vector

    io::stdout().write_all(&a_o_person_return_encoded).unwrap();
    // let a_o_person_return_json = serde_json::to_string(&a_o_person_return).unwrap();
    // println!("a_o_person_return_json {:?}",a_o_person_return_json); // deserializing is not so easy because i use fixed string lengths aka [u8, {n_fixed_length_here}] arrays, which then would get printed as "s_name\":[106,111,110,97,115,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0] 
}