
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
