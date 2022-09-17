
use std::fs;
use std::mem;
use std::io::Write;                                                                                                                                                                  
use std::io::prelude::*;                                                                                                                                            



use std::fs::File;     

use serde::{Serialize, Deserialize};
// use bincode;

// struct O_user {
//     n_id: u32,
//     s_name: [u8; 20],
//     s_email: [u8; 20],
// }
#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct O_user {
    n_id: u32,
    s_name: [u8; 20],
    s_email: [u8; 20],
}

fn main() {

    // f_read_as_string();
    // f_read_as_vec();
    // f_write_struct_data();
    f_write_multiple_struct_data();
    f_read_multiple_struct_data();
}

fn f_read_as_string(){
    println!("read as string");
    let data = fs::read_to_string("/etc/hosts").expect("Unable to read file");
    println!("{}", data);
}

fn f_read_as_vec(){
    println!("read as vec");
    let data = fs::read("/etc/hosts").expect("Unable to read file");
    println!("{}", data.len());
    println!("{:?}", data);
}


fn f_write_string_data(){

    let data = "Some data!";
    fs::write("/tmp/foo", data).expect("Unable to write file");

}

fn f_write_bytes_with_padding(mut bytes: &mut [u8], s: &str) {
    bytes.write(s.as_bytes()).unwrap();
}

fn f_write_struct_data(){

    // let mut bytes: [u8; 10] = [0; 10];
    // fill_from_str(&mut bytes, "hello");
    // println!("{:?}", bytes);

    let mut s_name : [u8; 20] = [0; 20];
    f_write_bytes_with_padding(&mut s_name, "abcdefghijklmnopqrstuvwxyz");
    f_write_bytes_with_padding(&mut s_name, "汪汪汪汪汪汪汪汪汪汪汪汪汪汪汪汪汪汪汪汪");// 20 times the character '汪' wont fit into 20 times u8 since the cahr '汪' is utf8 and needs more bytes than just one
    
    // s_name.write("aasdf".as_bytes()).unwrap("overflow ?");
    let o_user = O_user {
        n_id: 12341234,
        s_name: s_name,
        s_email: s_name
    };

    let a_o_person_encoded: Vec<u8> = bincode::serialize(&o_user).unwrap();
    println!("a_o_person_encoded.len() {:?}", a_o_person_encoded.len());

    // let mut f = File::create("o_user.vtk").expect("Unable to create file");     

    fs::write("./o_person", a_o_person_encoded).expect("writing not working");

    // let decoded: Entity = bincode::deserialize(&o_user[..]).unwrap();

    // fs::write("./o_user", encoded ).expect("Unable to write file");

    // let mut f = File::create("o_user.vtk").expect("Unable to create file");     
    // let vec_as_u8 =  &encoded[..];                  
    // f.write_all(vec_as_u8); 

    // for i in &encoded{                                                                                                                                                                  
    //     f.write_all((*i)).expect("Unable to write data");                                                                                                                            
    // }   

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


    fs::write("./a_o_person", a_o_person_encoded).expect("writing not working");

}

fn f_read_multiple_struct_data(){

    println!("read as vec");
    let a_o_person = fs::read("./a_o_person").expect("Unable to read file");

    let mut n_i = 8; // somehow the data starts after the 8th byte
    let n_size_of_o_person = mem::size_of::<O_user>();
    println!("n_size_of_o_person {:?}",n_size_of_o_person);
    println!("a_o_person.len() {:?}", a_o_person.len());


    let mut a_first_8_bytes : [u8; 8] = [0;8];
    a_first_8_bytes.clone_from_slice(&a_o_person[0..8]); 
    // let n :u64 = a_first_8_bytes;

    let n_little_endian = u64::from_be_bytes(a_first_8_bytes);
    let n_big_endian = u64::from_le_bytes(a_first_8_bytes);

    println!("a_first_8_bytes n little endian: {}", n_little_endian);
    println!("a_first_8_bytes n big endian: {}", n_big_endian);

    let n_real_len = n_size_of_o_person * (a_o_person.len()/n_size_of_o_person);
    // println!("a_o_person.count() {:?}", a_o_person.count());
    while(n_i < a_o_person.len()){
        // println!(" n_i {:?}", n_i);
        
        if(n_i+n_size_of_o_person < n_real_len){
            let a_o_person_subarray = &a_o_person[n_i..n_i+n_size_of_o_person];
            let o_user_decoded: O_user = bincode::deserialize(&a_o_person_subarray).unwrap();
            // println!("{:?}", o_user_decoded);
            println!("{:?}", String::from_utf8_lossy(&o_user_decoded.s_email));
        }
        n_i+=n_size_of_o_person;
    }
    
    // println!("{:?}", o_user_decoded);
    // println!("{:?}", a_o_person);
}