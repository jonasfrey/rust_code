use std::fs::File;
use std::io::Write;
use serde_json::{Result, Value};
use std::env;
use std::process;
use std::io::Read;
use std::io;
use eval::{Expr, to_value, eval};
use std::collections::HashMap;



fn f_json_to_hashmap(json: &str, keys: Vec<&str>) -> Result<HashMap<String, Value>> {
    let mut lookup: HashMap<String, Value> = serde_json::from_str(json).unwrap();
    let mut map = HashMap::new();
    for key in keys {
        let (k, v) = lookup.remove_entry (key).unwrap();
        map.insert(k, v);
    }
    Ok(map)
}

fn main() {

    let a_s_arg: Vec<String> = env::args().collect();
    let mut s_first_arg = a_s_arg[1].to_owned();
    println!("s_first_arg {:?}", s_first_arg);

    println!("s_first_arg {}", s_first_arg);
    let o_param: Value = serde_json::from_str(&s_first_arg).unwrap();
    println!("o_param {:?}", o_param);


    
    let a_o = o_param["a_o"].as_array().unwrap();
    
    let mut n_i_a_o = 0; 
    while(n_i_a_o < a_o.len()){
        let o_value = &a_o[n_i_a_o];
        for s_property_name in o_value.as_object().unwrap().keys(){
            println!("s_property_name {:?}", s_property_name);
            println!("value {:?}", o_value[s_property_name]);
            if(String::from(s_property_name).starts_with(":s")){
                println!("value evaluated {:?}", eval(o_value[s_property_name].as_str().unwrap()));
            }
        }
        // println!("value {:?}\n", a_o[n_i_a_o]);
        // let m: HashMap<String, String> = serde_json::from_str(&a_o[n_i_a_o]).unwrap();

        n_i_a_o+=1;
    }
    // for value in a_o{
    //     println!("value {:?}\n", value);
    // }

}
