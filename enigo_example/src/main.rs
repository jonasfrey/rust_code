// sudo apt install libxdo-dev
use enigo::*;
use std::{thread, time::Duration};
use std::env; 
use std::collections::HashMap;
use serde_json::{Result, Value};
use std::process::exit;

fn f_move(){

    let a_s_arg: Vec<String> = env::args().collect();
    let s_data = a_s_arg[1].clone();
    let o_data: serde_json::Value = serde_json::from_str(&s_data).expect("JSON was not well-formatted");
    let n_x = o_data["n_x"].as_u64().unwrap();
    let n_y = o_data["n_y"].as_u64().unwrap();
    // println!("n_x {}", n_x);
    let mut o_enigo = Enigo::new();
    o_enigo.mouse_move_to(n_x as i32, n_y as i32);

    println!("f_move called");
}

fn f_click(){
    println!("f_click called");
    let mut o_enigo = Enigo::new();
    o_enigo.mouse_down(MouseButton::Left);
    thread::sleep(Duration::from_millis(10));
    o_enigo.mouse_up(MouseButton::Left);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
fn f_error_argument(){
    println!("please call the binary with a JSON string as its first argument");
    println!("binary_name '{{...}}'");
    println!("please pass one of the following arguments");
    println!("{{'s_function_name':'f_move', 'n_x':1235, 'n_y':5813}}");
    println!("{{'s_function_name':'f_click'}}");
    println!("{{'s_function_name':'f_mouse_down'}}");
    println!("{{'s_function_name':'f_mouse_up'}}");
    exit(1);
}

fn main(){

    let a_s_arg: Vec<String> = env::args().collect();
    if a_s_arg.len() < 2 {
        f_error_argument();
    }

    let s_data = a_s_arg[1].clone();
    let o_data: serde_json::Value = serde_json::from_str(&s_data).expect("JSON was not well-formatted");
    // println!("s_data {:?}", &s_data);
    // // Parse the string of data into serde_json::Value.
    // let o_data: Value = serde_json::from_str(&s_data);
    // println!("o_data {:?}", o_data["s_function_name"]);

    // let s_function_name = o_data["s_function_name"].clone().to_string();
    let s_function_name = o_data["s_function_name"].as_str().unwrap();
    // print_type_of(&s_function_name::String);
    // println!("s_func {}", s_function_name);
    // exit(1);
    if s_function_name == serde_json::Value::Null{
        f_error_argument();
    }

    let mut a_functions: HashMap<_, fn()> = HashMap::new();

    a_functions.insert("f_move", f_move);
    a_functions.insert("f_click", f_click);


    let f_fun = a_functions.get(&*s_function_name);

    match f_fun {
        Some(f_fun) => f_fun(),
        None => {
            println!("invalid op");
        }
    };
    exit(0);

}