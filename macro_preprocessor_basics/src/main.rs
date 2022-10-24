// run 
// cargo install cargo-expand
// cargo expand 
// to view the expanded code 
// macro_rules! vec {
//     ( $( $x:expr ),* ) => {
//         {
//             let mut temp_vec = Vec::new();
//             $(
//                 temp_vec.push($x);
//             )*
//             temp_vec
//         }
//     };
// }



macro_rules! f_print_varname_and_value{
    // internal rule.

    ($var:ident) => {
        println!("var  name is : {:?}", $var);
        println!("var value is : {:?}", stringify!($var));
    }
}


macro_rules! f_add_one{
    // internal rule.

    ($var:ident) => {
        $var + 1.0;
    }
}


macro_rules! f_assign_value{
    // internal rule.

    (
        $s_property_name:ident, 
        $o_struct:expr, 
        $n_value:expr
    )
     => {
        $o_struct.$s_property_name = $n_value; // not working
        // $o_struct.stringify!($s_property_name) = $n_value; // also not working  
    }
}
struct O_fibonnacci{
    n_1: u32, 
    n_2: u32, 
    n_3: u32, 
}

fn f_assign_value(){
    let mut o_fibonnacci = O_fibonnacci{
        n_1: 1,
        n_2: 1,
        n_3: 0,
    };
    // n_3; <-- this would throw the compiler error //cannot find value `n_3` in this scope
    f_assign_value!(n_3, o_fibonnacci, 2); // but here the token `n_3` is not a variable name , it is just a token , which disappears after the macro has been expanded / converted to source code 
    
    println!("  _fibonnacci.n_1 {:?}", o_fibonnacci.n_1);
    println!("o_fibonnacci.n_2 {:?}", o_fibonnacci.n_2);
    println!("o_fibonnacci.n_3 {:?}", o_fibonnacci.n_3);
}
fn main() {
    let n_tau = 6.28;
    f_print_varname_and_value!(n_tau);

    println!(
        "{}", 
        f_add_one!(n_tau)
    );

    f_assign_value();
    // f_print_varname_and_value!(n_tau);
    // println!("Hello, world!");
}
