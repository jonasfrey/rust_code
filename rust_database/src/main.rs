
use std::fs;
use std::mem;
use std::io::Write;
use std::fs::File;
use serde::{Serialize, Deserialize};
use std::env;
use serde_json::{Result, Value};
use std::process;
use std::io::{self};
use std::path::Path;
use std::ptr;
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
    if(v["object"]["s_name"] != serde_json::Value::Null){
        s_s_name = v["object"]["s_name"].as_str();
    }
    let s_s_name_len = s_s_name.clone().unwrap().len();
    let mut s_s_name_padded = [0; 20];
    s_s_name_padded[..s_s_name_len].copy_from_slice(s_s_name.clone().unwrap().as_bytes()); 

    let mut n_n_age_milliseconds: Option<u64> = None;
    if(v["object"]["n_age_milliseconds"] != serde_json::Value::Null){
        n_n_age_milliseconds = v["object"]["n_age_milliseconds"].as_u64();
    }

    // get highest id 
    let mut n_id_highest = 0;
    let mut o_user_with_n_id : Option<O_user> = None;
    for o_user in &mut a_o_user {
        if(o_user.n_id > n_id_highest){
            n_id_highest = o_user.n_id;
            if(n_n_id != None){
                // println!("n_n_id {:?}", n_n_id);
                if(n_n_id.unwrap() as u32 == o_user.n_id){
                    o_user_with_n_id = Some(*o_user);
                    // println!("o_user {:?}", o_user);
                }
            }
        }
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
                if(o_user.s_name.starts_with(&s_s_name.clone().unwrap().as_bytes()[0..20])){
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
                println!("afsd {:?}", serde_json::to_string(&o_user_with_n_id.unwrap()));
            }
        }
        a_o_user_return.push(o_user_with_n_id.unwrap());
        b_write = true;
    }
    if(v["s_function_name"] == "f_a_o_delete"){// D

        
        b_write = true;
    }
    if(b_write){
        let a_o_user_encoded: Vec<u8> = bincode::serialize(&a_o_user).unwrap();
        println!("writing file");
        fs::write(s_file_name, &a_o_user_encoded).expect("writing not working"); // the first 8 bytes are used to store the number of elements in the vector
    }
    let a_o_user_return_encoded: Vec<u8> = bincode::serialize(&a_o_user_return).unwrap();
    io::stdout().write_all(&a_o_user_return_encoded).unwrap();
    let a_o_user_return_json = serde_json::to_string(&a_o_user_return).unwrap();
    println!("a_o_user_return_json {:?}",a_o_user_return_json);
}

// fn f_a_o_create(v: &Value, a_o_user :&mut Vec<O_user>, a_o_user_return : &mut Vec<O_user>){

// }

// fn f_a_o_read(v: &Value, a_o_user :&mut Vec<O_user>, a_o_user_return : &mut Vec<O_user>){
    
// }

// fn f_a_o_delete(v: &Value, a_o_user :&mut Vec<O_user>, a_o_user_return : &mut Vec<O_user>){
    
// }

// fn f_a_o_update(v: &Value, a_o_user :&mut Vec<O_user>, a_o_user_return : &mut Vec<O_user>){
    
// }

fn f_write_bytes_with_padding(mut bytes: &mut [u8], s: &str) {
    bytes.write(s.as_bytes()).unwrap();
}

fn f_write_multiple_struct_data(){

    let mut n_i = 0; 
    let n_max = 18;
    let mut a = Vec::new();
    while(n_i < n_max){
        let mut s_name : [u8; 20] = [0; 20];
        f_write_bytes_with_padding(&mut s_name, "abcdefghijklmnopqrstuvwxyz");
        
        let o_user = O_user {
            n_id: n_i,
            n_age_milliseconds: 10,
            s_name: s_name,
        };
        a.push(o_user);
        n_i+=1;
    }

    println!("a.len() {:?}", a.len());

    let a_o_user_encoded: Vec<u8> = bincode::serialize(&a).unwrap();

    println!("a_o_user_encoded.len() {:?}", a_o_user_encoded.len());

    // let mut f = File::create("o_user.vtk").expect("Unable to create file");     


    // the first 8 bytes are used to store the number of elements in the vector
    let mut a_first_8_bytes : [u8; 8] = [0;8];
    a_first_8_bytes.clone_from_slice(&a_o_user_encoded[0..8]); 
    // let n :u64 = a_first_8_bytes;

    let n_little_endian = u64::from_be_bytes(a_first_8_bytes);
    let n_big_endian = u64::from_le_bytes(a_first_8_bytes);

    println!("a_first_8_bytes n little endian: {}", n_little_endian);
    println!("a_first_8_bytes n big endian: {}", n_big_endian);

    // fs::write("./a_o_person", &a_o_user_encoded[8..]).expect("writing not working"); // the first 8 bytes are used to store the number of elements in the vector
    fs::write("./a_o_person", &a_o_user_encoded).expect("writing not working"); // the first 8 bytes are used to store the number of elements in the vector

}

fn f_read_multiple_struct_data(){

    println!("read as vec");
    let a_o_person = fs::read("./a_o_person").expect("Unable to read file");

    let mut n_i = 0; // somehow the data starts after the 8th byte
    let n_size_of_o_person = mem::size_of::<O_user>();
    println!("n_size_of_o_person {:?}",n_size_of_o_person);
    println!("a_o_person.len() {:?}", a_o_person.len());


    let a_o_person: Vec<O_user> = bincode::deserialize(&a_o_person).unwrap();

    println!(" a_o_person[0] {:?}", a_o_person[2]);

    // // let n_real_len = n_size_of_o_person * (a_o_person.len()/n_size_of_o_person);
    // let n_real_len = n_size_of_o_person * (a_o_person.len()/n_size_of_o_person);
    // // println!("a_o_person.count() {:?}", a_o_person.count());
    // while(n_i < a_o_person.len()){
    //     // println!(" n_i {:?}", n_i);
        
    //     let a_o_person_subarray = &a_o_person[n_i..n_i+n_size_of_o_person];
    //     let o_user_decoded: O_user = bincode::deserialize(&a_o_person_subarray).unwrap();
    //     // println!("{:?}", o_user_decoded);
    //     println!("{:?}", String::from_utf8_lossy(&o_user_decoded.s_email));
    //     n_i+=n_size_of_o_person;
    // }
    
    // println!("{:?}", o_user_decoded);
    // println!("{:?}", a_o_person);

    // ...
}