

fn f_s_my_stupid_float(
    a_nu8: Vec<u8>
) -> String {

    let mut s = String::from("");

    let mut n_byte = 0;
    let n_bits_per_nibble: u8 = 4;
    let n_base: u8 = 2;
    let n_nibble_mask: u8 = (n_base.pow(n_bits_per_nibble as u32)-1);
    
    while(n_byte < a_nu8.len()){
        let mut n_nibble = 0;
        // println!("{:?}", a_nu8[n_byte]);
        while(n_nibble < 2){
            let n = a_nu8[n_byte] >> ((1-n_nibble) * n_bits_per_nibble) & n_nibble_mask;
            // println!("{:?}", n);
            if(n < 10){
                s.push_str(&n.to_string());
            }
            if(n == 10){
                s.push('.')
            }
            if(n > 10){}
            n_nibble+=1;
        }
        n_byte+=1;
    }

    return s;
}

fn main() {


    let s_float = f_s_my_stupid_float(
        vec![
            0b00010010,
            0b00110100,
            0b01010110,
            0b01111000,
            0b10011010,
            0b11001101,
            0b11101111,
        ]
    );
    println!("s_float: {:?}", s_float);

    let s_float = f_s_my_stupid_float(
        vec![
            0b00011000,
            0b10100001,
            0b10001111
        ]
    );
    println!("s_float: {:?}", s_float);

}
