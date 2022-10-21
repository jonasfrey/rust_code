use chrono::Utc;
use std::sync::Arc;



fn f_a_n_u8__color_rgba_mixed(
    src: u32, 
    dst: u32, 
    t: u32, 
    // uint32_t src, uint32_t dst, uint32_t t
)-> u32{
    // assert(t <= 255);
    let s: u32 = 255 - t;
    let n_mixed : u32 = (
        (((((src >> 0)  & 0xff) * s +
           ((dst >> 0)  & 0xff) * t) >> 8)) |
        (((((src >> 8)  & 0xff) * s +
           ((dst >> 8)  & 0xff) * t)     )  & !0xff) |
        (((((src >> 16) & 0xff) * s +
           ((dst >> 16) & 0xff) * t) << 8)  & !0xffff) |
        (((((src >> 24) & 0xff) * s +
           ((dst >> 24) & 0xff) * t) << 16) & !0xffffff)
    );
        println!(" n_mixed  le bytes {:?} ", n_mixed.to_le_bytes());
    println!(" n_mixed  be bytes {:?} ", n_mixed.to_be_bytes());
    return n_mixed
}

pub fn main() {

    let mut n_four_bytes_as_u32 : u32 = 0;
    n_four_bytes_as_u32 = n_four_bytes_as_u32 | 70u32 << 8*0;
    n_four_bytes_as_u32 = n_four_bytes_as_u32 | 217u32 << 8*1;
    n_four_bytes_as_u32 = n_four_bytes_as_u32 | 98u32 << 8*2;
    n_four_bytes_as_u32 = n_four_bytes_as_u32 | 153u32 << 8*3;
    
    let mut n_four_bytes_as_u32__2 : u32 = 0;
    n_four_bytes_as_u32__2 = n_four_bytes_as_u32__2 | 255u32 << 8*0;
    n_four_bytes_as_u32__2 = n_four_bytes_as_u32__2 | 0u32 << 8*1;
    n_four_bytes_as_u32__2 = n_four_bytes_as_u32__2 | 87u32 << 8*2;
    n_four_bytes_as_u32__2 = n_four_bytes_as_u32__2 | 107u32 << 8*3;
    // println!("{}",n_four_bytes_as_u32)
    f_a_n_u8__color_rgba_mixed(
        n_four_bytes_as_u32, 
        n_four_bytes_as_u32__2, 
        107 as u32
    );
    
    f_a_n_u8__color_rgba_mixed(
        n_four_bytes_as_u32, 
        n_four_bytes_as_u32__2, 
        0 as u32
    );
    
        f_a_n_u8__color_rgba_mixed(
        n_four_bytes_as_u32, 
        n_four_bytes_as_u32__2, 
        153u32 as u32
    );
    
        f_a_n_u8__color_rgba_mixed(
        n_four_bytes_as_u32, 
        n_four_bytes_as_u32__2, 
        255 as u32
    );
    
    
    
    let mut n_four_bytes_as_u32 : u32 = 0;
    n_four_bytes_as_u32 = n_four_bytes_as_u32 | 70u32 << 8*3;
    n_four_bytes_as_u32 = n_four_bytes_as_u32 | 217u32 << 8*2;
    n_four_bytes_as_u32 = n_four_bytes_as_u32 | 98u32 << 8*1;
    n_four_bytes_as_u32 = n_four_bytes_as_u32 | 153u32 << 8*0;
    
    let mut n_four_bytes_as_u32__2 : u32 = 0;
    n_four_bytes_as_u32__2 = n_four_bytes_as_u32__2 | 255u32 << 8*3;
    n_four_bytes_as_u32__2 = n_four_bytes_as_u32__2 | 0u32 << 8*2;
    n_four_bytes_as_u32__2 = n_four_bytes_as_u32__2 | 87u32 << 8*1;
    n_four_bytes_as_u32__2 = n_four_bytes_as_u32__2 | 107u32 << 8*0;
    // println!("{}",n_four_bytes_as_u32)
    f_a_n_u8__color_rgba_mixed(
        n_four_bytes_as_u32, 
        n_four_bytes_as_u32__2, 
        107 as u32
    );
    
    f_a_n_u8__color_rgba_mixed(
        n_four_bytes_as_u32, 
        n_four_bytes_as_u32__2, 
        0 as u32
    );
    
        f_a_n_u8__color_rgba_mixed(
        n_four_bytes_as_u32, 
        n_four_bytes_as_u32__2, 
        153u32 as u32
    );
    
        f_a_n_u8__color_rgba_mixed(
        n_four_bytes_as_u32, 
        n_four_bytes_as_u32__2, 
        255 as u32
    );
}