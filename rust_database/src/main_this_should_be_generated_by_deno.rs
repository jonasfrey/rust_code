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


#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
struct O_user {
    n_id: u32, // if type == number
    n_age_milliseconds: u32,
    s_name: [u8; 20],
}

static s_object_name: &'static str = "O_user"; // == model name , 'A_'+..== table name /file name
static s_file_name: &'static str = "./A_o_user"; // 
// call it like 
// cargo run '{"object": {"s_name": "jonas"}, "s_function_name": "f_a_o_create"}'
fn main(){

    let a_s_argument: Vec<String> = env::args().collect();
    if(a_s_argument.len() != 2 ){
        eprintln!(r#"a JSON string in format '{{"s_function_name": "...", "object":{{...}}}}' must be passed as the first argument"#);
        process::exit(1);
    }
    let s_first_arg = a_s_argument[1].to_owned();
    // println!("a_s_argument {:?}", a_s_argument);

    let v: Value = serde_json::from_str(&s_first_arg).unwrap();

    if(v["object"] == serde_json::Value::Null){
        eprintln!("s_prop_name 'object' is required");
        process::exit(1);
    }
    if(v["s_function_name"] == serde_json::Value::Null){
        eprintln!("s_prop_name 's_function_name' is required");
        process::exit(1);
    }

    // println!("read as vec");
    let b_path_exists = Path::new(s_file_name).exists();
    let mut a_o_user : Vec<O_user> = Vec::new();

    if(b_path_exists){
        let a = fs::read(s_file_name).expect("Unable to read file");
        a_o_user = bincode::deserialize(&a).unwrap();
    }
    let mut b_write = false;
    let mut a_o_user_return : Vec<O_user> = Vec::new();


    let mut n_n_id: Option<u64> = None;
    if(v["object"]["n_id"] != serde_json::Value::Null){
        n_n_id = v["object"]["n_id"].as_u64();
    }

    let mut s_s_name: Option<&str> = None;
    let mut s_s_name_len: usize = 0;
    let mut s_s_name_padded = [0; 20];
    if(v["object"]["s_name"] != serde_json::Value::Null){
        s_s_name = v["object"]["s_name"].as_str();
        s_s_name_len = s_s_name.clone().unwrap().len();
        s_s_name_padded[..s_s_name_len].copy_from_slice(s_s_name.clone().unwrap().as_bytes());
        // println!("s_s_name_padded {:?}", &s_s_name_padded);
    }

    let mut n_n_age_milliseconds: Option<u64> = None;
    if(v["object"]["n_age_milliseconds"] != serde_json::Value::Null){
        n_n_age_milliseconds = v["object"]["n_age_milliseconds"].as_u64();
    }

    // get highest id 
    let mut n_id_highest = 0;
    let mut o_user_with_n_id : Option<O_user> = None;
    let mut n_index_o_user_with_n_id: Option<usize> = None;
    let mut n_count = 0;
    for o_user in &mut a_o_user {
        if(o_user.n_id > n_id_highest){
            n_id_highest = o_user.n_id;
            if(n_n_id != None){
                // println!("n_n_id {:?}", n_n_id);
                if(n_n_id.unwrap() as u32 == o_user.n_id){
                    o_user_with_n_id = Some(*o_user);
                    n_index_o_user_with_n_id = Some(n_count);
                    // println!("o_user {:?}", o_user);
                }
            }
        }
        n_count+=1;

    }
    let n_id_highest_incremented = n_id_highest+1;


    if(v["s_function_name"] == "f_a_o_create"){ // C

        if(o_user_with_n_id != None){
            eprintln!("cannot create new object with id: {}, already existing: {}", n_n_id.unwrap(), serde_json::to_string(&o_user_with_n_id).unwrap());
            process::exit(1);     
        }
        // check if all required s_prop_name are there 
        if(s_s_name == None){
            eprintln!("s_prop_name object.s_name is required");
            process::exit(1);
        }
        if(n_n_age_milliseconds == None){
            eprintln!("s_prop_name object.n_age_milliseconds is required");
            process::exit(1);
        }

        // ------------ validate s_str_s_name
        if(s_s_name_len > 20){
            eprintln!("object.s_name ({:?}) len bytes {:?}, allowed len bytes {:?}", s_s_name, s_s_name_len, 20);
            process::exit(1);
        }


        // ------------ validate n_num_n_age_milliseconds
        if(n_n_id != None){
            if(n_n_id.unwrap() > u32::MAX.into()){
                eprintln!("object.n_age_milliseconds, value {:?}, is bigger than allowed max {:?}", n_n_id, u32::MAX );
                process::exit(1);
            }
            if(n_n_id.unwrap() < u32::MIN.into()){
                eprintln!("object.n_age_milliseconds, value {:?}, is smaller than allowed min {:?}", n_n_id, u32::MIN );
                process::exit(1);
            }
        }

        let o_user = O_user {
            n_id: n_id_highest_incremented,
            n_age_milliseconds: n_n_age_milliseconds.unwrap() as u32,
            s_name: s_s_name_padded
        };
        a_o_user.push(o_user);

        a_o_user_return.push(o_user.clone());

        b_write = true;
    }

    if(v["s_function_name"] == "f_a_o_read"){   // R
        let mut b_conditions_match;
        for o_user in &mut a_o_user {
            b_conditions_match = true;
            if(n_n_id != None){
                if(o_user.n_id != n_n_id.unwrap() as u32){
                    b_conditions_match = false; 
                }
            }
            if(s_s_name != None){
                if(o_user.s_name.starts_with(&s_s_name_padded) == false){
                    b_conditions_match = false; 
                }
            }
            if(n_n_age_milliseconds != None){
                if(o_user.n_age_milliseconds != n_n_age_milliseconds.unwrap() as u32){
                    b_conditions_match = false; 
                }
            }
            // println!("b_conditions_match {}", b_conditions_match)

            if(b_conditions_match){
                a_o_user_return.push(*o_user)
            }

        }
    }
    
    if(v["s_function_name"] == "f_a_o_update"){ // U

        if(n_n_id == None){
            eprintln!("objec.n_id must be provided in order to update an object");
            process::exit(1);
        }
        if(o_user_with_n_id == None){
            eprintln!("object with n_id ({:?}) was not found and cannot be updated", n_n_id.unwrap());
            process::exit(1);
        }
        if(s_s_name != None){
            if(o_user_with_n_id != None){
                o_user_with_n_id.unwrap().s_name = s_s_name_padded;
                // println!("afsd {:?}", serde_json::to_string(&o_user_with_n_id.unwrap()));
            }
        }
        a_o_user_return.push(o_user_with_n_id.unwrap());
        b_write = true;
    }
    if(v["s_function_name"] == "f_a_o_delete"){// D
        if(o_user_with_n_id == None){
            eprintln!("cannot delete object with n_id: {}, not existing", n_n_id.unwrap());
            process::exit(1);
        }
        a_o_user_return.push(o_user_with_n_id.unwrap());
        a_o_user.remove(n_index_o_user_with_n_id.unwrap());

        b_write = true;
    }

    if(b_write){
        let a_o_user_encoded: Vec<u8> = bincode::serialize(&a_o_user).unwrap();
        // println!("writing file");
        fs::write(s_file_name, &a_o_user_encoded).expect("writing not working"); // the first 8 bytes are used to store the number of elements in the vector
    }
    
    let a_o_user_return_encoded: Vec<u8> = bincode::serialize(&a_o_user_return).unwrap();
    let s_file_name_function_return = format!("{}_{}", s_file_name, v["s_function_name"].as_str().unwrap()); 

    fs::write(s_file_name_function_return, &a_o_user_return_encoded).expect("writing not working"); // the first 8 bytes are used to store the number of elements in the vector
    io::stdout().write_all(&a_o_user_return_encoded).unwrap();
    // let a_o_user_return_json = serde_json::to_string(&a_o_user_return).unwrap();
    // println!("a_o_user_return_json {:?}",a_o_user_return_json); // deserializing is not so easy because i use fixed string lengths aka [u8, {n_fixed_length_here}] arrays, which then would get printed as "s_name\":[106,111,110,97,115,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0] 
}