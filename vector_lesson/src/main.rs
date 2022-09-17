fn f_n_u8_vector_as_u64(a_u8: &[u8], b_big: bool)-> u64{

    let mut n_u64 = 0;

    let mut n_i = 0; 
    let mut n_i_used = 0;
    while(n_i < 8){
        if(b_big){
            n_i_used = n_i;
        }else{
            n_i_used = (8-1)-n_i;
        }
        n_u64 = n_u64 | u64::from(a_u8[n_i]) << (8 * n_i_used);

        n_i+=1;
    }

    return n_u64;
}
fn main() {

    // interpret the first 8 bytes of this vector as a u64 number

    let a : Vec<u8> = vec![10,10,0,0,0,0,0,0,0,0,0,0]; 

    let mut a_first_8_bytes : [u8; 8] = [0;8];
    a_first_8_bytes.clone_from_slice(&a[0..8]); 
    // let n :u64 = a_first_8_bytes;

    let n_little_endian = u64::from_be_bytes(a_first_8_bytes);
    let n_big_endian = u64::from_le_bytes(a_first_8_bytes);
    // u64::from_be_bytes

    println!("n little endian: {}", n_little_endian);
    println!("n big endian: {}", n_big_endian);

    let n = f_n_u8_vector_as_u64(&a[0..8], true);
    println!("n: {}", n);

    let n_little_endian = f_n_u8_vector_as_u64(&a[0..8], false);
    println!("n_little_endian: {}", n_little_endian);

}
