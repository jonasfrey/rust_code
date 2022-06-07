extern crate useless; // may be required for rust 2015 and earlier

fn main(){
    
    let n_a :u32= 123;
    let n_b :u32= 1234; 

    // let n_sum = useless::f_local_function(n_a, n_b); // not working because f_local_function is not 'pub fun...'
    let n_sum = useless::f_pub_sum(n_a,n_b);

    println!("sum is: {}", n_sum);
}

//rustc is_dependend_of_library.rs --extern useless=libuseless.rlib --edition=2018 