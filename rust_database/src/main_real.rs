
use std::fs;
use std::mem;
use std::io::Write;
use std::fs::File;
use serde::{Serialize, Deserialize};
use std::env;
use serde_json::{Result, Value};
use std::process;
#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct O_user {
    n_id: u32,
    s_name: [u8; 20],
    s_email: [u8; 20],
}

fn main() {
    let s: String = "Greg".into();
    
    match &s[..] {
        "Greg" => func_1(),
        "Dave" => func_2(),
        _ => {},
    }
}  

fn func_1(){
    println!("Function 1");
}

fn func_2() {
    println!("Function 2");
}


// call it like 
// cargo run '{"s_object_name":"O_person", "object": {"name"}, "s_function_name": "f_a_o_create"}'
fn main(){



    let args: Vec<String> = env::args().collect();
    let mut s_first_arg = args[1].to_owned();
    println!("s_first_arg {}", s_first_arg);
    let v: Value = serde_json::from_str(&s_first_arg).unwrap();

    if(v["s_object_name"] == serde_json::Value::Null){
        println!("s_prop_name 's_object_name' is required")
        process::exit(1);
    }
    if(v["object"] == serde_json::Value::Null){
        println!("s_prop_name 'object' is required")
        process::exit(1);
    }
    if(v["s_function_name"] == serde_json::Value::Null){
        println!("s_prop_name 's_function_name' is required")
        process::exit(1);
    }


    if(v["s_function_name"] == "f_a_o_create"){
        f_a_o_create(&v);
    }
    if(v["s_function_name"] == "f_a_o_read"){
        f_a_o_read(&v);
    }
    if(v["s_function_name"] == "f_a_o_update"){
        f_a_o_update(&v);
    }
    if(v["s_function_name"] == "f_a_o_deleted"){
        f_a_o_deleted(&v);
    }

    println!("{:?}", v["s_object_name"]);
    // Parse the string of data into serde_json::Value.
    let mut s_json = args[0].to_owned();

    let s_slice: &str = &s_json[..];  // take a full slice of the string

    f_write_multiple_struct_data();
    f_read_multiple_struct_data();
}

fn f_a_o_create(){
    
}

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
            s_name: s_name,
            s_email: s_name
        };
        a.push(o_user);
        n_i+=1;
    }

    println!("a.len() {:?}", a.len());

    let a_o_person_encoded: Vec<u8> = bincode::serialize(&a).unwrap();

    println!("a_o_person_encoded.len() {:?}", a_o_person_encoded.len());

    // let mut f = File::create("o_user.vtk").expect("Unable to create file");     


    // the first 8 bytes are used to store the number of elements in the vector
    let mut a_first_8_bytes : [u8; 8] = [0;8];
    a_first_8_bytes.clone_from_slice(&a_o_person_encoded[0..8]); 
    // let n :u64 = a_first_8_bytes;

    let n_little_endian = u64::from_be_bytes(a_first_8_bytes);
    let n_big_endian = u64::from_le_bytes(a_first_8_bytes);

    println!("a_first_8_bytes n little endian: {}", n_little_endian);
    println!("a_first_8_bytes n big endian: {}", n_big_endian);

    // fs::write("./a_o_person", &a_o_person_encoded[8..]).expect("writing not working"); // the first 8 bytes are used to store the number of elements in the vector
    fs::write("./a_o_person", &a_o_person_encoded).expect("writing not working"); // the first 8 bytes are used to store the number of elements in the vector

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