use std::fs::OpenOptions;
use std::io::prelude::*;
use std::env;
use std::{thread, time};
fn main() {
    let a_s_arg: Vec<String> = env::args().collect();
    let s_string =  a_s_arg[1].clone();
    if(a_s_arg.len() < 2){
        eprintln!("plases provide a character as argument, example: 'cargo run |'");
        std::process::exit(1);
    }
    let s_file_name = "the_file";
    let a_nu8 : [u8; 10] = [s_string.as_bytes()[0] ;10];
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