use std::fs::File;
use std::io::prelude::*;
use std::process::exit;
use std::convert::TryInto;
use std::time::{Duration, SystemTime};
use std::time::UNIX_EPOCH;
fn f_a_convert_u32_to_4_u8_values(n_u32: u32) -> [u8;4]{
// input:   0b1010 1010       0000 1111       0011 0011       0101 1111  // one number
// output: [0b1010 1010,    0b0000 1111,    0b0011 0011,    0b0101 1111] // array of integers

    // from low to high
    // 0b1010 1010 0000 1111 0011 0011 0101 1111
    //                                 ^--- l1
    //                       ^--- l2
    //             ^--- l3
    //   ^---l4
    let n_8bits_l1: u8 = (n_u32 & 0b11111111) as u8;
    let n_8bits_l2: u8 = ((n_u32 & 0b1111111100000000) >> 8) as u8;
    let n_8bits_l3: u8 = ((n_u32 & 0b111111110000000000000000) >> 8*2) as u8;
    let n_8bits_l4: u8 = ((n_u32 & 0b11111111000000000000000000000000) >> 8*3) as u8;

    // return [
    //     n_8bits_l4,
    //     n_8bits_l3,
    //     n_8bits_l2,
    //     n_8bits_l1
    // ]; 
    let a_array:[u8;4] =  [
        n_8bits_l4,
        n_8bits_l3,
        n_8bits_l2,
        n_8bits_l1
    ]; 

    return a_array; 
    

}
fn main() -> std::io::Result<()> {
    let s_filename = "bitmap_image.bmp";
    {
        
        let mut file = File::create(s_filename)?;
        

        // let mut a_array = vec![0;     0];

        let n_width: u32 = 5000; 
        let n_height: u32 = 5000; 
        let n_channels: u32 = 3;
        let n_bits_per_channel: u32 = 8;  
        let n_bits_per_pixel: u32 = n_bits_per_channel * n_channels; 
        let n_imagedata_size_bytes: u32 = (((n_width )*(n_height ))*((n_bits_per_pixel/8)) ) as u32;
        let n_header_size = 0; // unknown at the beginning 
        let mut n_file_size_bytes: u32 = 0; // unknow at the beginning
        let n_bitmap_fileheader_size: u32 = 14; //signature=2+filesize=4+reservd1=2+reservd2=2+fileoffsettopixelarray=4 => 2+4+2+2+4 = 14
        
        // const N_ENOUGH: usize = n_imagedata_size_bytes+1000;
        // let mut a_array:[u8; N_ENOUGH] = [0; N_ENOUGH];
        let n_enough = n_imagedata_size_bytes + 1000;
        let mut a_array = vec![0; n_enough.try_into().unwrap()];


        // header
        // signature 
        a_array[0] = 'B' as u8; 
        a_array[1] = 'M' as u8; 
        // file size in bytes
        // a_array[2] = n_file_size_bytes; // will be set in the end
        a_array[3] = 0; 
        a_array[4] = 0; 
        a_array[5] = 0; 
        // reserved 1 
        a_array[6] = 0;
        a_array[7] = 0;
        // reserved 2 
        a_array[8] = 0; 
        a_array[9] = 0; 
        // file offset to pixel array
        a_array[10] = 0;
        a_array[11] = 0;
        a_array[12] = 0;
        a_array[13] = 0;

        // DIB HEADER BITMAPV5HEADER (DIB = device independent bitmap)
        // DIB header size 
        // a_array[14] = 0; // will be set later
        a_array[15] = 0;
        a_array[16] = 0;
        a_array[17] = 0;
        // image width
        let a_n_width = f_a_convert_u32_to_4_u8_values(n_width); 
        a_array[18] = a_n_width[3];
        a_array[19] = a_n_width[2];
        a_array[20] = a_n_width[1];
        a_array[21] = a_n_width[0];
        // image height 
        let a_n_height = f_a_convert_u32_to_4_u8_values(n_height); 
        a_array[22] = a_n_height[3];
        a_array[23] = a_n_height[2];
        a_array[24] = a_n_height[1];
        a_array[25] = a_n_height[0];
        // planes
        a_array[26] = 0;
        a_array[27] = 0;
        // bits per pixel 
        a_array[28] = n_bits_per_pixel as u8; // unsigned 8 bit values 0-255 
        a_array[29] = 0;
        // compression
        a_array[30] = 0;
        a_array[31] = 0;
        a_array[32] = 0;
        a_array[33] = 0;
        // image size in bytes
        let a_n_imagedata_size_bytes = f_a_convert_u32_to_4_u8_values(n_imagedata_size_bytes); 
        a_array[34] = a_n_imagedata_size_bytes[3];
        a_array[35] = a_n_imagedata_size_bytes[2];
        a_array[36] = a_n_imagedata_size_bytes[1];
        a_array[37] = a_n_imagedata_size_bytes[0];
        // x pixels per meter (Specifies print resolution, in pixels per meter, of the target device for the bitmap)
        a_array[38] = 0;
        a_array[39] = 0;
        a_array[40] = 0;
        a_array[41] = 1;
        // y pixels per meter (Specifies print resolution, in pixels per meter, of the target device for the bitmap)
        a_array[42] = 0;
        a_array[43] = 0;
        a_array[44] = 0;
        a_array[45] = 1;
        // colors in color table
        a_array[46] = 0;
        a_array[47] = 0;
        a_array[48] = 0;
        a_array[49] = 0;
        // important color count
        a_array[50] = 0;
        a_array[51] = 0;
        a_array[52] = 0;
        a_array[53] = 0;
        //red cahnnel bitmask
        let n_index_start = 54; 
        let mut n_index: u32 = 54;

        while n_index < (n_index_start+(((n_width as u32)*(n_height as u32)*(n_channels as u32)) as u32)) {
            // println!("{:?}", n_index);
            // a_array[n_index as usize] = (n_index%255) as u8;

            let n_unix_ts_millisec = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_micros();
            a_array[n_index as usize] = ((1/(n_unix_ts_millisec%10+1))*255) as u8;
            n_index+=1;
        }

        // file size 
        let a_n_index = f_a_convert_u32_to_4_u8_values(n_index); 
        a_array[2] = a_n_index[3];
        a_array[3] = a_n_index[2];
        a_array[4] = a_n_index[1];
        a_array[5] = a_n_index[0];

        n_file_size_bytes = n_index;
        a_array[14] = (n_file_size_bytes - n_bitmap_fileheader_size - n_imagedata_size_bytes) as u8; // DIB header size!!!

        let a_slice = &a_array[0..(n_index) as usize];

        
        // set header size 

        file.write_all(
            &a_slice
        )?;
    }

    // {
    //     let mut file = File::open(s_filename)?;
    //     // read the same file back into a Vec of bytes
    //     let mut buffer = Vec::<u8>::new();
    //     file.read_to_end(&mut buffer)?;
    //     println!("{:?}", buffer);
    // }
    // {
    //     let mut file = File::open("bitmap.bmp")?;
    //     // read the same file back into a Vec of bytes
    //     let mut buffer = Vec::<u8>::new();
    //     file.read_to_end(&mut buffer)?;
    //     println!("{:?}", buffer);
    // }

    Ok(())
}