
fn f_test_infinity(){

    let a_nu8: [u8;4] = 
        [
        //  0b01111111,0b10000000,0b00000000,0b00000000 // rust::4.6096e-41, js: Infinity
    //   0b00000000,0b01111111,0b11111111,0b11111111 //rust: NaN, js: 1.1754942106924411e-38
        //  0b01111111,0b10000000,0b00000000,0b00000000 //rust:
        0b11000010,0b11100101,0b01000000,0b00000000 // from https://www.youtube.com/watch?v=2dopLI1GZig

        // 0b11000010,0b11100101,0b01000000,0b10101100 // check to see what happens when we fill the padding with random bits
        
            //||          |
            //^|sign bit  |
            // ^ exponent bits
            //            |
            //            ^ mantissa bits
        ];
    println!("{:?}", a_nu8);

    
    let n_f32 = f32::from_be_bytes(a_nu8);
    
    println!("n_f32 :{:?}", n_f32);
    // var a_nu8 = new Uint8Array([127, 128, 0, 0]);
    // var n = new DataView(a_nu8.buffer);
    // var b_little_endian = false;
    // console.log(n.getFloat32(b_little_endian));

    // let n: f32 = (1.0/2.0+ 1.0/4.0+ 1.0/8.0 + 1.0/16.0 + 1.0/32.0 + 1.0/64.0 + 1.0/128.0 + 1.0/256.0 + 1.0/512.0 + 1.0/1024.0);
    // let n: f32 = (1.0/2.0+ 1.0/4.0+ 1.0/8.0);
    let n: f32 = (1.0/2.0+ 1.0/4.0+ 1.0/8.0);
    println!("n{:?}", n);
    // let n: f32 = -114.625;
    println!("{:320b}", n.to_bits())
}

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
    f_test_infinity();
}
