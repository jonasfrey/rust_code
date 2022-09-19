use std::fs::OpenOptions;
use std::io::prelude::*;
use std::env;
use std::{thread, time};
use std::fs::File;
use std::fs;

fn main() {
    let a_s_arg: Vec<String> = env::args().collect();
    let s_string =  a_s_arg[1].clone();
    if(a_s_arg.len() < 2){
        eprintln!("plases provide a character as argument, example: 'cargo run |'");
        std::process::exit(1);
    }
    let mut a_nu8 : [u8; 10] = [s_string.as_bytes()[0] ;10];
    // f_append_to_file(&a_nu8);
    f_read_and_write_file(&a_nu8);
}

fn f_append_to_file(a_nu8: &[u8]){
    
    let s_file_name = "the_file";
    let mut o_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(s_file_name)
        .unwrap();
    
    let mut n_i = 0;
    
    loop {
        thread::sleep(time::Duration::from_millis(10));
        n_i += 1;
        o_file.write_all(&a_nu8).expect("Unable to write data");
    
        if n_i == 1000 {
            println!("OK, that's enough");
            // Exit this loop
            break;
        }
    }
}
fn f_read_and_write_file(a_nu8: &[u8]){
    let s_file_name = "the_file";
    let s_file_content = fs::read_to_string(s_file_name)
        .expect("Should have been able to read the file");

    let mut file = File::open(s_file_name).unwrap();
    let mut a_nu8_file = s_file_content.as_bytes().to_vec();

    for val in a_nu8 {
        a_nu8_file.push(*val);
    }

    file.write_all(&a_nu8_file).unwrap();

}