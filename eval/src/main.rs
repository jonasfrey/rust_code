
use serde_json::{Value};
use eval::{Expr,eval, to_value};
use serde::{Serialize, Deserialize};
use std::process;
use std::env;

use evalexpr::*;

#[derive(Serialize)]
struct O_person{
    n_id: u32,
    s_name: String, 
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn eval_dynamic(st:&str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    std::fs::write("/tmp/eval.rs",format!("#[no_mangle]fn eval() -> Vec<u8> {{ { }}}",st))?;
    ::std::process::Command::new("rustc")
        .args(&[
            "-o","/tmp/eval.so",
            "--crate-type","dylib",
            "/tmp/eval.rs"
        ]).spawn()?.wait_with_output()?;
    unsafe {
        let lib = libloading::Library::new("/tmp/eval.so")?;
        let func: libloading::Symbol<unsafe extern fn() -> Vec<u8>> = lib.get(b"eval")?;
        Ok(func())
    }
}
    

fn f_b_filter_function(
    o_person: &O_person, 
    v: &Value
) -> bool {

    // this text would come from an ascii text
    // println!("{:?}", serde_json::to_string(o_person).unwrap());
    let b = o_person.s_name.to_lowercase() == v["s_name"].as_str().unwrap();
    return b;
}
fn f_example_3(){
    let a_s_argument: Vec<String> = env::args().collect();
    if(a_s_argument.len() != 2 ){
        eprintln!(r#"a JSON string in format '{{"s_function_name": "...", "object":{{...}}}}' must be passed as the first argument"#);
        process::exit(1);
    }

    
    let s_first_arg = a_s_argument[1].to_owned();
    // println!("a_s_argument {:?}", a_s_argument);
    
    let v: Value = serde_json::from_str(&s_first_arg).unwrap();


    let mut a_o_person : Vec<O_person> = vec![];
    let mut n_i = 0;
    a_o_person.push(
        O_person {
            n_id: n_i, 
            s_name: String::from("HaLLo")
        }
    );
    a_o_person.push(
        O_person {
            n_id: n_i, 
            s_name: String::from("hallo")
        }
    );
    while(n_i < 100){
        a_o_person.push(
            O_person {
                n_id: n_i, 
                s_name: String::from("halLO")
            }
        );
        n_i+=1;
    }

    let mut a_o_person_filtered = vec![];
    for o_person in a_o_person{

        if(f_b_filter_function(&o_person, &v)){
            a_o_person_filtered.push(o_person)
        }
        
    }

    let s_json = serde_json::to_string(&a_o_person_filtered).unwrap();
    println!("a_o_person_filtered: {:?}", s_json);

}

fn f_example_1(){

    let a_s_argument: Vec<String> = env::args().collect();
    if(a_s_argument.len() != 2 ){
        eprintln!(r#"a JSON string in format '{{"s_function_name": "...", "object":{{...}}}}' must be passed as the first argument"#);
        process::exit(1);
    }
    let s_first_arg = a_s_argument[1].to_owned();
    // println!("a_s_argument {:?}", a_s_argument);
    
    let v: Value = serde_json::from_str(&s_first_arg).unwrap();
    
    
    let mut a_o_person : Vec<O_person> = vec![];
    let mut n_i = 0;
    a_o_person.push(
        O_person {
            n_id: n_i, 
            s_name: String::from("HaLLo")
        }
    );
    a_o_person.push(
        O_person {
            n_id: n_i, 
            s_name: String::from("hallo")
        }
    );
    while(n_i < 100){
        a_o_person.push(
            O_person {
                n_id: n_i, 
                s_name: String::from("halLO")
            }
        );
        n_i+=1;
    }
    
    
    for o_person in a_o_person{
        // let o_expression = Expr::new("o.s_name.to_uppercase() == String::from(\"AF\") || o.n_id == 23") 
        // let o_expression = Expr::new("f_touppercase(o.s_name) == \"ha\"") 
        let o_expression = Expr::new("f_touppercase(o.n_id) == \"ha\"") 
        // .function("f_touppercase", |o| { println!("asfd {:?}", o);print_type_of(&o.first());Ok(to_value(o)) } )
        .function("f_touppercase", |o| { println!("asfd {:?}", o);print_type_of(&o.first());Ok(to_value(o.first().unwrap().as_str().unwrap().to_uppercase())) } )
            .value("o", &o_person)
            .exec().expect("error");
        println!("eval: {:?}",o_expression);

        let b_test = o_person.s_name.to_uppercase() == "AF";
        println!("not eval: {:?}",b_test);
        
    }
    
    // assert_eq!(Expr::new("foo == bar")
    //             .value("foo", true)
    //             .value("bar", true)
    //             .exec(),
    //         Ok(to_value(true)));
}
// fn f_example_2(){

//     let a_s_argument: Vec<String> = env::args().collect();
//     if(a_s_argument.len() != 2 ){
//         eprintln!(r#"a JSON string in format '{{"s_function_name": "...", "object":{{...}}}}' must be passed as the first argument"#);
//         process::exit(1);
//     }
//     let s_first_arg = a_s_argument[1].to_owned();
//     // println!("a_s_argument {:?}", a_s_argument);
    
//     let v: Value = serde_json::from_str(&s_first_arg).unwrap();
    
    
//     let mut a_o_person : Vec<O_person> = vec![];
//     let mut n_i = 0; 
//     while(n_i < 100){
//         a_o_person.push(
//             O_person {
//                 n_id: n_i, 
//                 s_name: String::from("Af")
//             }
//         );
//         n_i+=1;
//     }
    
    
    
//     // let mut context = HashMapContext::new();



    
//     for o_person in a_o_person{
//         let context = context_map! {
//             "n_id" => o_person.n_id,
//             "s_name" => o_person.s_name
//         }.unwrap();
    
//         // assert_eq!(eval_empty_with_context_mut("n_id = 5", &mut context), Ok(EMPTY_VALUE));
//         // let o_expression = Expr::new("o.s_name.to_uppercase() == String::from(\"AF\") || o.n_id == 23") 
//         // let o_expression =  eval("o.s_name::to_uppercase() == \"AF\"");
//         // let o_expression =  eval("o.s_name == \"Af\"");
//         let o_expression =  eval_boolean_with_context_mut("n_id == 5", &mut context);
//         println!(" eval: {:?}",o_expression);


//         let b_test = o_person.s_name.to_uppercase() == "AF";
//         println!("not eval: {:?}",b_test);
        
//     }
    



// }

fn main() {
    // f_example_1();
    // f_example_2();
    f_example_3();

}
