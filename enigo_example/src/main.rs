use enigo::*;
use std::{thread, time::Duration};
// sudo apt install libxdo-dev
use std::env; 


use serde_json::{Result, Value};

fn main(){
    // println!("Hello, test!");
    let a_s_arg: Vec<String> = env::args().collect();

    let s_data = a_s_arg[1].clone();
    println!("s_data {:?}", &s_data);
    // Parse the string of data into serde_json::Value.
    // let o_data: Value = serde_json::from_str(&s_data);

    let o_data: serde_json::Value =
    serde_json::from_str(&s_data).expect("JSON was not well-formatted");

    println!("o_data {:?}", o_data["s_function_name"]);

    let s_function_name = o_data["s_function_name"].clone();
    if s_function_name == Null{
        println!("please pass one of the following arguments");
        println!("{'s_function_name':'f_move', 'x':20, 'y':40}");
        println!("{'s_function_name':'f_click'}");
        println!("{'s_function_name':'f_mouse_down'}");
        println!("{'s_function_name':'f_mouse_up'}");
    }
    
    println!("{:?}",s_function_name);

}
fn f_move(){

}

fn f_click(){

    let mut o_enigo = Enigo::new();
    o_enigo.mouse_down(MouseButton::Left);
    thread::sleep(Duration::from_millis(10));
    o_enigo.mouse_up(MouseButton::Left);

}