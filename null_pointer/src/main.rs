use std::env;
use std::ptr;


fn main(){
    // f_main_not_working();
    f_main_working();
}
// fn f_main_not_working() {


//     let a_s_argument: Vec<String> = env::args().collect();
//     if(a_s_argument.len() > 1){
//         let n = 1;
//     }else{
//         let n: *const i32 = ptr::null();
//     }
//     println!("{:?}", n);
//     // if(n.is_null()){
//     //     println!("is null");
//     // }else{
//     //     println!("is not null");
//     // }

// }   


fn f_main_working(){

    let a_s_argument: Vec<String> = env::args().collect();
    let mut n: Option<u32> = None;
    // ...
    
    if(a_s_argument.len() > 1){
        n = Some(1);
    }
    // ...
    
    if(n == None){
        println!("is none {:?}", n);
        // println!("n.unwrap() {:?}", n.unwrap()); // this will panic!
    }else{
        println!("is not none {:?}", n);
        println!("n.unwrap() {:?}", n.unwrap());
    }

    println!("n == Some(1) {:?}", n == Some(1));
}