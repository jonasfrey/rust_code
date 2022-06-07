
use std::fs::File;
use std::io::Write; 
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

macro_rules! f_create_function_by_identifier_and_expression {

    // ($s_function_name:expr) => { //this is not working we have to use "ident" (identifier)
    ($i_function_name:ident, $s_function_body:literal) => {
        // creating a new function 
        fn $i_function_name(){
            let evaluated_result = eval($s_function_body);
            println!("evaluated_result is : {}", evaluated_result);
            println!("function created by f_create_function was called!");
        }
    }
}




// This is the main function
fn main() {

    let n = f_return_without_return();
    println!("{:?}", n);

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

    
    f_println_macro_tutorial();

    // the following is not working, in rust we cannot convert runtime values such a strings to a function
    // f_create_function_by_identifier_and_expression!(f_another, "1+(1)+(1+(1))+(1+(1)+(1+(1)))+(1+(1)+(1+(1))+(1+(1)+(1+(1))))");
    // f_another();

    f_type_casting();
    // f_address_and_value_aka_index_and_value();
    // f_datatypes();

    f_struct();


    f_save_struct_array_as_file();

    
    f_rounding_nums();

    f_compare_strings();
    
    f_datatypes_minmax();

    f_datatypes_convert_negative_u8_to_i8();
}

fn f_println_macro_tutorial(){
    let n_num_five = 5; 
    let n_num_two = 2;

    let n_for_debugging = 12341234; 
    let s_for_debugging = "the variable s_for_debugging holds this string"; 
    let a_for_debugging = [1,2,3,4,5];

    println!("we can print a string an brakets get substituated with variables such as : {}", n_num_five);
    
    
    println!("we can print multiple vars like this {}, {}", n_num_five, n_num_two);
    
    println!("we can set the order by adding the index in the bracket {1}, {0}", n_num_five, n_num_two);
    
    
    println!("{{:?}} is for debugging, n_for_debugging: {:?}", n_for_debugging);
    println!("{{:?}} is for debugging, s_for_debugging: {:?}", s_for_debugging);
    // note {} wont print quotes around the string variable 
    println!("{{}} is for printing any var, s_for_debugging: {}", s_for_debugging);
    
    
    // pretty print {:#?}
    // println!("a_for_debugging : {}", a_for_debugging); // a_for_debugging cannot be printed with {}, we need to use {:#?}

    println!("{{:#?}} is for pretty print n_for_debugging: {:#?} ", n_for_debugging);

    println!("{{:#?}} is for pretty print s_for_debugging: {:#?} ", s_for_debugging);
    
    println!("{{:#?}} is for pretty print a_for_debugging: {:#?} ", a_for_debugging);

    let n_address = &n_num_five;

    println!("n_address : {:?}", (&n_num_five)); // wtf ds isch ja huere schlecht wtf i schwoere wtf scheiss rust huere fcking hipster scheiss sprach , nid besser aus c wtf huere behbinderti scheisse ohni witz ey
    println!("n_address : {:#?}", (&n_num_five)); // wttttfff wtf

    // 1 und 1 = 2 und 1 = 3
    
    
    // actually println! is noting more than a print of a fromated string 

    let s_formated = format!("this is a formated string, n_num_five: {}", n_num_five); 
    println!("{}", s_formated);

}

fn f_address_and_value_aka_index_and_value(){
    
    // since rust is low level, it has, as expected, address and value of any variable 

    let n_num = 10; 

    println!("value('n_num'):{}", n_num);
    // println!("address('&n_num'):{}", (&n_num)); // println! wont work when we dont use {:p} 
    println!("address('&n_num'):{:p}", &n_num);
    

    // we can also print as integer 
    // println!("address('&n_num'):{}", ((&n_num) as i64) );

    let n_address = &n_num;
}
// use std::char;
fn f_type_casting(){

    let n_num_ascii_a = 65; 
    println!("n_num_ascii_a: {}", n_num_ascii_a);

    let s_num_ascii_a_as_char = char::from_u32(n_num_ascii_a).unwrap();
    println!("s_num_ascii_a_as_char: {}", s_num_ascii_a_as_char);

    // println!("s_num_ascii_a_as_char: {}", n_num_ascii_a);

   
    let n_num_ascii_a: u8 = 65; 
    println!("n_num_ascii_a: {}", n_num_ascii_a);

    let s_num_ascii_a_as_char = n_num_ascii_a as char; 
    println!("s_num_ascii_a_as_char: {}", s_num_ascii_a_as_char);

    // let n_basesixteen_address_of_n_num_ascii_a: *const u64 = &n_num_ascii_a;
    // println!("n_basesixteen_address_of_n_num_ascii_a {:p}", n_basesixteen_address_of_n_num_ascii_a);

    // // let n_baseten_address_of_n_num_ascii_a = n_basesixteen_address_of_n_num_ascii_a as u64;
    // let n_baseten_address_of_n_num_ascii_a_u64 = n_basesixteen_address_of_n_num_ascii_a as u64;
    // println!("n_baseten_address_of_n_num_ascii_a_u64 {}", n_baseten_address_of_n_num_ascii_a_u64);

    // let n_baseten_address_of_n_num_ascii_a_usize = n_basesixteen_address_of_n_num_ascii_a as usize;
    // println!("n_baseten_address_of_n_num_ascii_a_usize {}", n_baseten_address_of_n_num_ascii_a_usize);

    // println!("n_num_ascii_a: {:c}", n_num_ascii_a);

}

fn f_datatypes(){

    // rust guesses the type by default 

    // let n_num_int = 10; 
    // println!(n_num_int);
    // let n_num_float = 1.1235; 
    
}

fn f_bitwise_operators(){
        
    let n_test = 0b011101010101; 
    
    println!("{:#b}", n_test>>8); 
    
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



// struct definition
struct O_test{
    b_test : bool, 
    n_test : u8, 
    s_test : String
}

#[derive(Debug)]
struct O_test_derived_from_debug{
    b_test : bool, 
    n_test : u8, 
    s_test : String
}


fn f_struct(){

    // create a new "instance" of the struct
    let o_test1 = O_test{
        b_test: true,
        n_test: 128u8, 
        s_test: String::from("this is a test string yeey")
    };

    // access a property / attribute / "member?"
    println!("{:?}", o_test1.b_test); 


    // changing a prop  only possible if the entire struct is mut
    let mut o_test_mut = O_test{
        b_test: true, 
        n_test: 123u8,
        s_test: String::from("asdf")
    };
    println!("{:?}", o_test_mut.s_test); 

    o_test_mut.s_test = String::from("new string is assigned as value");

    println!("{:?}", o_test_mut.s_test); 
    // println!("{:?}", o_test_mut); //this wont work , we need to add '#[derive(Debug)]' one line before the start of the struct definition 'struct O_structname'  


    let mut o_test_from_fun = f_o_test();
    println!("{:?}", o_test_from_fun.s_test);
    // can we modify it ? 
    o_test_from_fun.s_test = String::from("this struct came from a function and a property on it was changed in another scope!!! huray");
    
    println!("{:?}", o_test_from_fun.s_test);

    // can we modify it in another scope ? 
    f_modify_struct(o_test_from_fun); 
    
    // 
    let mut o_test_derived_from_debug = O_test_derived_from_debug{
        b_test: true, 
        n_test: 123u8,
        s_test: String::from("asdf")
    };
    println!("{:?}", o_test_derived_from_debug);



    
}

fn f_modify_struct(
    mut o_test: O_test // we need the 'mut' keyword
){
    o_test.s_test = String::from("the string was modified in the function f_modify_struct"); 
    println!("{:?}", o_test.s_test);
}

fn f_o_test() 
    -> O_test
{   
    // return O_test{
    O_test{ // wtf we do not need to have a 'return' expression to return the struct ?!?!?!?!?
        b_test: true, 
        n_test: 23u8, 
        s_test: String::from("this struct was created in the function f_o_test")
    }
    // this 
}

// fn f_n_withoutreturn() -> u8 {
//     10u8;
// }

// fn f_return_without_return(){ //function definition needs a return type
fn f_return_without_return() -> u8{
    // the last statement in a scope will return from the function, but only if there is no semicolon ";"!!!
    (10+11)
}

struct O_foo{
    n_num: u8
}

fn f_o_foo() -> O_foo
{
    O_foo{
        n_num:12
    }
}

fn f_n_test() -> u8 {
    42u8
}

struct O_txtfile{
    a_bytes: Vec<u8>
}
fn f_o_txtfile() -> O_txtfile{
    let mut a_bytes = vec![];

    let mut o_txtfile = O_txtfile{ a_bytes : a_bytes};
    o_txtfile.a_bytes.push(65);
    o_txtfile.a_bytes.push(66);
    o_txtfile.a_bytes.push(67);
    o_txtfile.a_bytes.push(68);
    o_txtfile.a_bytes.push(69);
    return o_txtfile;
}
fn f_modify_o_txtfile_by_address(
    o_txtfile: &mut O_txtfile
){
    o_txtfile.a_bytes[0] = 70;
    o_txtfile.a_bytes[1] = 71;
}
fn f_modify_o_txtfile_by_borrowing(
    mut o_txtfile: O_txtfile
)-> O_txtfile{
    o_txtfile.a_bytes[2] = 80;
    o_txtfile.a_bytes[3] = 81;
    return o_txtfile;
}
fn f_save_struct_array_as_file(){

    let mut o_txtfile = f_o_txtfile();
    f_modify_o_txtfile_by_address(&mut o_txtfile);
    o_txtfile = f_modify_o_txtfile_by_borrowing(o_txtfile);

    let mut file = File::create("o_txtfile.txt").unwrap();
    file.write_all(
        &o_txtfile.a_bytes
    ).unwrap();

}
fn f_rounding_nums(){

    println!("(1.0_f32 / 1.5_f32) as f32: {:?}", (1.0_f32 / 1.5_f32) as f32);
    println!("(1.0_f32 / 1.5_f32) as u8: {:?}", (1.0_f32 / 1.5_f32) as u8);

    println!("(1.0_f32 / 2.0_f32) as f32: {:?}", (1.0_f32 / 2.0_f32) as f32);
    println!("(1.0_f32 / 2.0_f32) as u8: {:?}", (1.0_f32 / 2.0_f32) as u8);

    println!("(1.0_f32 / 3.0_f32) as f32: {:?}", (1.0_f32 / 3.0_f32) as f32);
    println!("(1.0_f32 / 3.0_f32) as u8: {:?}", (1.0_f32 / 3.0_f32) as u8);

    println!("(1.0_f32 / 4.0_f32) as f32: {:?}", (1.0_f32 / 4.0_f32) as f32);
    println!("(1.0_f32 / 4.0_f32) as u8: {:?}", (1.0_f32 / 4.0_f32) as u8);

    println!("(1000.0_f32 / 60.0_f32) as f32: {:?}", (1000.0_f32 / 60.0_f32) as f32);
    println!("(1000.0_f32 / 60.0_f32) as u8: {:?}", (1000.0_f32 / 60.0_f32) as u8);

    // keep f32 ronding down 
    println!("(((5.0_f32 / 3.0_f32) as u32) as f32): {:?}",(((5.0_f32 / 3.0_f32) as u32) as f32));
    // rounding down with floor
    println!("(5.0_f32 / 3.0_f32).floor(): {:?}",(5.0_f32 / 3.0_f32).floor());
    // rounding up with ceil
    println!("(5.0_f32 / 3.0_f32).ceil(): {:?}",(5.0_f32 / 3.0_f32).ceil());
    // roundiing up if >= x.5 , rounding down if <= x.5 
    println!("(1.666_f32).round(): {:?}",(1.666_f32).round());
    println!("(0.333_f32).round(): {:?}",(0.333_f32).round());

}

fn f_datatypes_minmax(){
    println!(" i8 has the min value of {}.", i8::min_value());
    println!(" i8 has the max value of {}.", i8::max_value());
    println!(" i16 has the min value of {}.", i16::min_value());
    println!(" i16 has the max value of {}.", i16::max_value());
    println!(" i32 has the min value of {}.", i32::min_value());
    println!(" i32 has the max value of {}.", i32::max_value());
    println!(" i64 has the min value of {}.", i64::min_value());
    println!(" i64 has the max value of {}.", i64::max_value());
    println!(" i128 has the min value of {}.", i128::min_value());
    println!(" i128 has the max value of {}.", i128::max_value());
    
}

fn f_datatypes_convert_negative_u8_to_i8(){
    let n_i8_positive_127: i8 = 127;
    let n_i8_negative_127: i8 = -127;
    let n_i8_positive_1: i8 = 1;
    let n_i8_negative_1: i8 = -1;
    println!("n_i8_positive_127: {} | {:#08b}",n_i8_positive_127,n_i8_positive_127);
    println!("-n_i8_negative_127: {} | {:#08b}",n_i8_negative_127,n_i8_negative_127);
    println!("-n_i8_negative_127 as u8: {} | {:#08b}",n_i8_negative_127 as u8, n_i8_negative_127 as u8);

    println!("n_i8_positive_1: {} | {:#08b}",n_i8_positive_1,n_i8_positive_1);
    println!("-n_i8_negative_1: {} | {:#08b}",n_i8_negative_1,n_i8_negative_1);
    println!("-n_i8_negative_1 as u8: {} | {:#08b}",n_i8_negative_1 as u8, n_i8_negative_1 as u8);


    println!("-1_f64: {} | {:#64b}",-1_f64, -1_f64);

    println!("-1_f64 as i8: {} | {:#08b}",-1_f64 as i8,-1_f64 as i8);
    println!("-1_f64 as u8: {} | {:#08b}",-1_f64 as u8,-1_f64 as u8);
    println!("(-1_f64 as i8) as u8: {} | {:#08b}",(-1_f64 as i8) as u8,(-1_f64 as i8) as u8);



}
fn f_compare_strings(){

    let s_test_a = String::from("test1");
    let s_test_b = String::from("test1");

    println!("s_test_a == s_test_b: {}", s_test_a == s_test_b);

    f_compare_strings_in_different_scope(s_test_a);
}
fn f_compare_strings_in_different_scope(
    s: String
){
    println!("s_test_a == s_test_b: {}", String::from("test1") == s);

}