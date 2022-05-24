
fn f_macros(){
    //check below     
}
//macros are called with a ! at the end so for example 'println!'
macro_rules! a_macro_function {
    () => (
        println!("a_macro_function was called !");
    )
}

// the arguments need a $ as prefix 
macro_rules! f_this_macro_prints_a_literal {
    ($var_a:literal) => (
        println!("var_a:literal is : {}", $var_a);
    )
}
macro_rules! f_this_macro_prints_an_expression {
    // the arguments need a $ as prefix
    // ant as type annotation with a designator of the following  
    // valid fragment specifiers are `ident`, `block`, `stmt`, `expr`, `pat`, `ty`, `lifetime`, `literal`, `path`, `meta`, `tt`, `item` and `vis`
    ($var_a:expr) => (
        println!("expression string: {}", stringify!($var_a));
        println!("expression evaluated: {}", $var_a);
    )
}

macro_rules! f_dd {
    // the arguments need a $ as prefix
    // ant as type annotation with a designator of the following  
    // valid fragment specifiers are `ident`, `block`, `stmt`, `expr`, `pat`, `ty`, `lifetime`, `literal`, `path`, `meta`, `tt`, `item` and `vis`
    ($var_a:expr) => (
        println!("{}:{}", stringify!($var_a), $var_a);
    )
}
macro_rules! f_create_function_by_identifier {

    // ($s_function_name:expr) => { //this is not working we have to use "ident" (identifier)
    ($i_function_name:ident) => {
        // creating a new function 
        fn $i_function_name(){
            println!("function created by f_create_function was called!");
        }
    }
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
macro_rules! f_create_function_by_identifier_and_expression {

    // ($s_function_name:expr) => { //this is not working we have to use "ident" (identifier)
    ($i_function_name:ident, $s_function_body:literal) => {
        // creating a new function 
        fn $i_function_name(){
            let evaluated_result = eval_dynamic($s_function_body);
            println!("evaluated_result is : {}", evaluated_result);
            println!("function created by f_create_function was called!");
        }
    }
}




// This is the main function
fn main() {

    f_can_this_be_used_in_main_although_it_was_declared_below_the_main_fn();

    f_variables();


    a_macro_function!();
    
    f_this_macro_prints_a_literal!(18);
    f_this_macro_prints_a_literal!("hello");
    f_this_macro_prints_a_literal!(1.1235);

    let n_num = 10; 
    f_this_macro_prints_an_expression!(n_num);

    //
    // f_pretty_print!(18);
    let n_number = 11235; 
    f_dd!(n_number);

    // let s_function_name = "f_test"; 
    // f_create_function_by_identifier!(s_function_name); 

    f_create_function_by_identifier!(f_test); // f_test is now a identifier! 
    f_test();
    f_create_function_by_identifier_and_expression!(f_another, "1+(1)+(1+(1))+(1+(1)+(1+(1)))+(1+(1)+(1+(1))+(1+(1)+(1+(1))))");
    f_another();

}


fn f_variables(){

    let n_immutable = 10; 

    println!("n_immutable:{}", n_immutable);

    //      n_test cannot be mutated 

    // n_test = 20; //cannot assign twice to immutable variable


    let mut n_mutable = 10; 

    println!("n_mutable:{}", n_mutable);
    
    n_mutable = 4321;
    println!("n_mutable:{}", n_mutable);

}

// fn f_datatypes(){
//     let n_num_int : int = 10; 

//     println!()
// }

fn f_can_this_be_used_in_main_although_it_was_declared_below_the_main_fn(){
    // yes functions can be declared after they are called!
    println!("if you see f_can_this_be_used_in_main_although_it_was_declared_below_the_main_fn, then yes")
}
