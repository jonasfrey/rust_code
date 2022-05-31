use std::fs::File;
use std::io::prelude::*;
use std::process::exit;
use std::convert::TryInto;
use std::time::{Duration, SystemTime};
use std::time::UNIX_EPOCH;

use rand::Rng;



// creates a new "instance"" 
fn f_a_convert_u16_to_2_u8_values(n_u16:u16) -> [u8;2]{
    // input:   0b1010 1010       0000 1111   // one number
    // output: [0b1010 1010,    0b0000 1111 ] // array of integers

    // from low to high
    // 0b0011 0011 0101 1111
    //             ^--- l1
    //   ^--- l2
    let n_8bits_l1: u8 = (n_u16 & 0b11111111) as u8;
    let n_8bits_l2: u8 = ((n_u16 & 0b1111111100000000) >> 8) as u8;
    
    let a_array:[u8;2] =  [
        n_8bits_l2,
        n_8bits_l1
    ]; 

    return a_array; 
}

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
//ffmpeg -i 'img-%03d.jpeg' -r 10 out.mkv
fn main() -> std::io::Result<()> {

        // println!("{}", n_i);

    let n_ts_milsec =SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis(); 
    let s_filename = str::replace("wav_noise{n_ts_milsec}.wav", "{n_ts_milsec}", &n_ts_milsec.to_string());
    // let s_filename = "bitmap_image_"..".bmp";
    {
        
        let mut file = File::create(s_filename)?;
        

        // let mut a_array = vec![0;     0];

        let s_riff_mark = [b'R', b'I', b'F', b'F'];  // ChunkID; 
        let n_file_size_bytes: u32 = 50000; // used at the end //ChunkSize
        let s_wave_mark = [b'W',b'A',b'V',b'E']; // Format

        let s_fmt_mark = [b'f',b'm',b't', b' ']; // Subchunk1ID
        let n_index_now: u32 = 16; //subchunk1 size , index until now

        let n_audio_format_type: u32 = 1; // 1 = PCM , 
        let n_num_channels: u32 = 1; // number of channels, 1 = mono

        let n_samples_per_second_aka_samplerate: u32 = 44100; // 44100 common, 4800 CD quality, 88200 better quality, number of samples per second!
        let n_bits_per_sample: u32 = 16; 
        let n_bits_per_second_all_channels: u32 = n_bits_per_sample * n_samples_per_second_aka_samplerate * n_num_channels;
        let n_bytes_per_second_all_channels: u32 = n_bits_per_second_all_channels / 8;
        let n_block_align: u32 = n_num_channels * (n_bits_per_sample / 8);
        let s_data_mark = [b'd',b'a',b't',b'a'];
        let n_header_end_index: u32 = 44;
        let n_data_size_bytes: u32 = n_file_size_bytes - n_header_end_index;


        let mut a_array = vec![0; n_file_size_bytes as usize];
        // let mut a_array: [u8; n_file_size_bytes] = [0; 3];
        


        // The "RIFF" chunk descriptor 
        a_array[0] = s_riff_mark[0]; // big endian starting with index 0
        a_array[1] = s_riff_mark[1];
        a_array[2] = s_riff_mark[2];
        a_array[3] = s_riff_mark[3];
        // ChunkSize
        let a_n_file_size_bytes = f_a_convert_u32_to_4_u8_values(n_file_size_bytes); 
        a_array[4] = a_n_file_size_bytes[3]; // little endian
        a_array[5] = a_n_file_size_bytes[2];
        a_array[6] = a_n_file_size_bytes[1];
        a_array[7] = a_n_file_size_bytes[0];

        //Format
        a_array[8] = s_wave_mark[0]; //big endian
        a_array[9] = s_wave_mark[1];
        a_array[10] = s_wave_mark[2];
        a_array[11] = s_wave_mark[3];
        //Subchunk1ID
        a_array[12] = s_fmt_mark[0]; // big endian
        a_array[13] = s_fmt_mark[1]; 
        a_array[14] = s_fmt_mark[2]; 
        a_array[15] = s_fmt_mark[3];
        
        //
        let a_n_index_now = f_a_convert_u32_to_4_u8_values(n_index_now); 
        a_array[16] = a_n_index_now[3];
        a_array[17] = a_n_index_now[2];
        a_array[18] = a_n_index_now[1];
        a_array[19] = a_n_index_now[0];

        // 
        let a_n_audio_format_type = f_a_convert_u32_to_4_u8_values(n_audio_format_type); 
        a_array[20] = a_n_audio_format_type[3];
        a_array[21] = a_n_audio_format_type[2];

        //  
        let a_n_num_channels = f_a_convert_u32_to_4_u8_values(n_num_channels);
        println!("{:?}", a_n_num_channels);
        a_array[22] = a_n_num_channels[3];
        a_array[23] = a_n_num_channels[2];

        //
        let a_n_samples_per_second_aka_samplerate = f_a_convert_u32_to_4_u8_values(n_samples_per_second_aka_samplerate); 
        a_array[24] = a_n_samples_per_second_aka_samplerate[3];
        a_array[25] = a_n_samples_per_second_aka_samplerate[2];
        a_array[26] = a_n_samples_per_second_aka_samplerate[1];
        a_array[27] = a_n_samples_per_second_aka_samplerate[0];
        
        //
        let a_n_bytes_per_second_all_channels = f_a_convert_u32_to_4_u8_values(n_bytes_per_second_all_channels); 
        a_array[28] = a_n_bytes_per_second_all_channels[3]; 
        a_array[29] = a_n_bytes_per_second_all_channels[2];
        a_array[30] = a_n_bytes_per_second_all_channels[1];
        a_array[31] = a_n_bytes_per_second_all_channels[0];

        //
        let a_n_block_align = f_a_convert_u32_to_4_u8_values(n_block_align); 
        a_array[32] = a_n_block_align[3];
        a_array[33] = a_n_block_align[2];

        // 
        let a_n_bits_per_sample = f_a_convert_u32_to_4_u8_values(n_bits_per_sample); 
        a_array[34] = a_n_bits_per_sample[3];
        a_array[35] = a_n_bits_per_sample[2];


        a_array[36] = s_data_mark[0];//big endian
        a_array[37] = s_data_mark[1];
        a_array[38] = s_data_mark[2];
        a_array[39] = s_data_mark[3];

        // file size
        let a_n_data_size_bytes = f_a_convert_u32_to_4_u8_values(n_data_size_bytes); 

        a_array[40] = a_n_data_size_bytes[3];
        a_array[41] = a_n_data_size_bytes[2];
        a_array[42] = a_n_data_size_bytes[1];
        a_array[43] = a_n_data_size_bytes[0];

        // n_header_end_index // is 44
        println!("{}", n_header_end_index);
        println!("{}", n_file_size_bytes);
        let mut rng = rand::thread_rng();
        let mut n_i = n_header_end_index;
        let n_sawtooth_freq = 1.2; 
        
        while n_i < n_file_size_bytes - 10 {

            // let n_rand_f32: f32 = rng.gen();
            // let n_rand_u16: u16 = rng.gen();

            // let n_u16: u16 = ((n_i*10) as u16) % (2_u16.pow(16)-1);
            let n_u16: u16 = (((n_i as f32) * n_sawtooth_freq) as u16) % u16::MAX;
            // let n_u16 = n_rand_u16;
            let a_n_rand_u16 = f_a_convert_u16_to_2_u8_values(n_u16);
            a_array[(n_i+0) as usize] = a_n_rand_u16[0+0];
            a_array[(n_i+1) as usize] = a_n_rand_u16[0+1];

            n_i = n_i + 2; 
        }

        // let a_slice = &a_array[0..(n_index) as usize];

        file.write_all(
            &a_array
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

