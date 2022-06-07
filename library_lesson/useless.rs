// rustc --crate-type=lib useless.rs


fn f_local_function(
    n_a: u32, 
    n_b: u32
)->u32{
    return n_a + n_b; 
}
pub fn f_pub_sum(
    n_a:u32,
    n_b:u32
)->u32{
    return f_local_function(n_a, n_b);
}

