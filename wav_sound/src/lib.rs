extern crate image;
use std::fs::File;
use std::io::prelude::*;
use std::process::exit;
use std::convert::TryInto;
use std::time::{Duration, SystemTime};

use std::time::UNIX_EPOCH;
use image::GenericImageView;


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

fn f_add_samples_milliseconds_of_freq_sinewave(
    n_milliseconds: u32, 
    n_freq: u32,
    a_array: &mut Vec<u8>,
    n_samples_per_second_aka_samplerate: u32, 
    n_bits_per_sample: u32
){

    // let n_index_of_data_mark = a_array.iter().position(|&r| r == "data").unwrap();
    let mut n_i = 0;
    let mut n_index_of_data_mark = 0;
    while n_i < a_array.len(){
        if
            a_array[n_i+0] == b'd'
            &&
            a_array[n_i+1] == b'a'
            &&
            a_array[n_i+2] == b't'
            &&
            a_array[n_i+3] == b'a'
        {
            n_index_of_data_mark = n_i;
            break;
        }
        n_i = n_i+1;
    }

    println!("n_index_of_data_mark: {}", n_index_of_data_mark);

    let n_samples_to_add = ((n_samples_per_second_aka_samplerate as f32) / 1000.0 * (n_milliseconds as f32))as u32;
    let mut n_count_sample = 0;

    let mut n_radians_start: f32 = 3.14159;
    let n_freq_float = n_freq as f32 / 2000.0_f32;

    while n_count_sample < n_samples_to_add {

        // let n_rand_f32: f32 = rng.gen();
        // let n_rand_u16: u16 = rng.gen();

        // let n_u16: u16 = ((n_i*10) as u16) % (2_u16.pow(16)-1);
        // let n_u16: u16 = (((n_i as f32) * n_sawtooth_freq) as u16) % u16::MAX;
        n_radians_start = n_radians_start + n_freq_float;

        let n_u16 = (((n_radians_start.sin() * (u16::MAX) as f32) + ((u16::MAX as f32)/2.0))) as u16;
        // let n_u16 = n_rand_u16;
        let a_n_u16 = f_a_convert_u16_to_2_u8_values(n_u16);
        a_array.push(a_n_u16[0+0]);
        a_array.push(a_n_u16[0+1]);

        n_count_sample = n_count_sample + 1; 
    }


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

pub struct O_wav{
    s_riff_mark : [u8; 4],
    s_wave_mark : [u8; 4],
    s_fmt_mark  : [u8; 4],
    n_index_now: u32,
    n_audio_format_type: u32 ,
    n_num_channels: u32 ,
    n_samples_per_second_aka_samplerate: u32 ,
    n_bits_per_sample: u32 ,
    n_bits_per_second_all_channels: u32 ,
    n_bytes_per_second_all_channels: u32 ,
    n_block_align: u32 ,
    s_data_mark: [u8; 4],
    n_header_end_index: u32,
    n_filesize_min: u32,
    a_array: Vec<u8>,   
}
pub fn f_o_wav()-> O_wav {

    let s_riff_mark = [b'R', b'I', b'F', b'F'];  // ChunkID; "RIFF"
    let s_wave_mark = [b'W',b'A',b'V',b'E']; // Format "WAVE"

    let s_fmt_mark = [b'f',b'm',b't', b' ']; // Subchunk1ID "fmt "
    let n_index_now: u32 = 16; //subchunk1 size , index until now

    let n_audio_format_type: u32 = 1; // 1 = PCM , 
    let n_num_channels: u32 = 1; // number of channels, 1 = mono

    let n_samples_per_second_aka_samplerate: u32 = 44100; // 44100 common, 4800 CD quality, 88200 better quality, number of samples per second!
    let n_bits_per_sample: u32 = 8; 
    let n_bits_per_second_all_channels: u32 = n_bits_per_sample * n_samples_per_second_aka_samplerate * n_num_channels;
    let n_bytes_per_second_all_channels: u32 = n_bits_per_second_all_channels / 8;
    let n_block_align: u32 = n_num_channels * (n_bits_per_sample / 8);
    let s_data_mark = [b'd',b'a',b't',b'a'];// "data"
    let n_header_end_index: u32 = 44;


    let n_filesize_min = n_header_end_index;
    let mut a_array = vec![0; n_filesize_min as usize];


    // The "RIFF" chunk descriptor 
    a_array[0] = s_riff_mark[0]; // big endian starting with index 0
    a_array[1] = s_riff_mark[1];
    a_array[2] = s_riff_mark[2];
    a_array[3] = s_riff_mark[3];


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
    
    // let a_slice = &a_array[0..(n_index) as usize];

    let n_file_size_bytes: u32 = a_array.len() as u32; // used at the end //ChunkSize
    // ChunkSize
    let a_n_file_size_bytes = f_a_convert_u32_to_4_u8_values(n_file_size_bytes); 
    a_array[4] = a_n_file_size_bytes[3]; // little endian
    a_array[5] = a_n_file_size_bytes[2];
    a_array[6] = a_n_file_size_bytes[1];
    a_array[7] = a_n_file_size_bytes[0];

    let n_data_size_bytes: u32 = n_file_size_bytes - n_header_end_index;
    // file size
    let a_n_data_size_bytes = f_a_convert_u32_to_4_u8_values(n_data_size_bytes); 

    a_array[40] = a_n_data_size_bytes[3];
    a_array[41] = a_n_data_size_bytes[2];
    a_array[42] = a_n_data_size_bytes[1];
    a_array[43] = a_n_data_size_bytes[0];

    return O_wav{
        s_riff_mark : s_riff_mark ,
        s_wave_mark : s_wave_mark ,
        s_fmt_mark  : s_fmt_mark  ,
        n_index_now: n_index_now,
        n_audio_format_type: n_audio_format_type,
        n_num_channels: n_num_channels,
        n_samples_per_second_aka_samplerate: n_samples_per_second_aka_samplerate,
        n_bits_per_sample: n_bits_per_sample,
        n_bits_per_second_all_channels: n_bits_per_second_all_channels,
        n_bytes_per_second_all_channels: n_bytes_per_second_all_channels,
        n_block_align: n_block_align,
        s_data_mark: s_data_mark,
        n_header_end_index: n_header_end_index,
        n_filesize_min: n_filesize_min,
        a_array: a_array,
    };
}

fn f_n_number_of_samples_in_o_wav(
    o_wav: &O_wav 
) -> u32{
    return o_wav.a_array.len() as u32 - o_wav.n_header_end_index; 
}
pub fn f_add_sound(
    mut o_wav: O_wav,
    n_frequency_cycles_per_second: u32, // eg. 432, //frequency
    s_wavetype: String, // eg. 'sawtooth' , 'sine'  
    n_milliseconds: u32, // eg 100
)-> O_wav{

    // calculate how many samples to add 
    let n_samples_to_add = ((o_wav.n_samples_per_second_aka_samplerate as f32) / 1000.0 * (n_milliseconds as f32))as u32;
    let mut n_count_sample = 0;

    let n_samples_per_cycle: u32 = (o_wav.n_samples_per_second_aka_samplerate / n_frequency_cycles_per_second);
    let n_radians_per_sample: f64 = std::f64::consts::TAU / ((n_samples_per_cycle as f64));
    let mut n_radians: f64 = 0.0;
    let mut n_sample_value: u8 = 0;
    let n_amplitude: f32 = 0.5;// the full amplitude somehow does not work
    
    // depends on the bit depth
    let n_max: u8 = 255; 
    println!("(n_amplitude * (u16::MAX as f32)) {:?}", (n_amplitude * (u16::MAX as f32)));
    println!("((u16::MAX as f32)) {:?}", ((u16::MAX as f32)));
    println!("((u16::MAX as f32)) {:?}", ((u16::MAX as f32)));
    println!("(n_amplitude * (u16::MAX as f32)) as u16 {:?}", (n_amplitude * (u16::MAX as f32)) as u16);
    while n_count_sample < n_samples_to_add {
        let n_amplitude_offset: u8 = ((n_amplitude as f64/2.0) * n_max as f64) as u8;

        if(s_wavetype == String::from("square")){
            // if square wave , signal is on or off
            if (n_count_sample % (n_samples_per_cycle)) > n_samples_per_cycle / 2{
                n_sample_value = 0 + n_amplitude_offset;
            }else{
                n_sample_value = (n_amplitude * n_max as f32) as u8 + n_amplitude_offset;// if it is u16 max, it wont work 
            }
        }

        if(s_wavetype == String::from("sine")){
            n_radians += n_radians_per_sample;
            n_sample_value = (((f64::sin(n_radians) * 0.5_f64  + 0.5_f64) * n_amplitude as f64) * 255 as f64) as u8 + n_amplitude_offset;
        }   

        if(s_wavetype == String::from("sawtooth")){
            n_sample_value = (((n_count_sample as u32 % n_samples_per_cycle as u32) as f32 / n_samples_per_cycle as f32) * n_amplitude * n_max as f32) as u8 + n_amplitude_offset;
        }

        if(s_wavetype == String::from("triangle")){

            if (n_count_sample % (n_samples_per_cycle)) > n_samples_per_cycle / 2{
                n_sample_value = (((n_count_sample as u32 % n_samples_per_cycle as u32) as f32 / n_samples_per_cycle as f32) * n_amplitude  * n_max as f32) as u8 + n_amplitude_offset;
            }else{
                n_sample_value = n_max + n_amplitude_offset*2 -(((n_count_sample as u32 % n_samples_per_cycle as u32) as f32 / n_samples_per_cycle as f32) * n_amplitude  * n_max as f32) as u8 + n_amplitude_offset;
            }
        }
        // let a_n_u16 = f_a_convert_u16_to_2_u8_values(n_sample_value as u16);
        // o_wav.a_array.push(a_n_u16[0+0]);
        // o_wav.a_array.push(a_n_u16[0+1]);

        o_wav.a_array.push(n_sample_value);
        n_count_sample = n_count_sample + 1; 
    }

    let n_file_size_bytes: u32 = o_wav.a_array.len() as u32; // used at the end //ChunkSize
    // ChunkSize
    let a_n_file_size_bytes = f_a_convert_u32_to_4_u8_values(n_file_size_bytes); 
    o_wav.a_array[4] = a_n_file_size_bytes[3]; // little endian
    o_wav.a_array[5] = a_n_file_size_bytes[2];
    o_wav.a_array[6] = a_n_file_size_bytes[1];
    o_wav.a_array[7] = a_n_file_size_bytes[0];

    let n_data_size_bytes: u32 = n_file_size_bytes - o_wav.n_header_end_index;
    // file size
    let a_n_data_size_bytes = f_a_convert_u32_to_4_u8_values(n_data_size_bytes); 

    o_wav.a_array[40] = a_n_data_size_bytes[3];
    o_wav.a_array[41] = a_n_data_size_bytes[2];
    o_wav.a_array[42] = a_n_data_size_bytes[1];
    o_wav.a_array[43] = a_n_data_size_bytes[0];

    return o_wav;
}



pub fn f_save_o_wav(
    o_wav: O_wav, 
    s_path_file_name: String, 
)-> bool{
    let mut file = File::create(s_path_file_name).unwrap();
    // create image of wavedata for debugging
    let mut rng = rand::thread_rng();

    let n_width = f_n_number_of_samples_in_o_wav(&o_wav); 
    let n_height = 255;

    let mut a_image_buffer = image::ImageBuffer::new(n_width, n_height); 


    // A redundant loop to demonstrate reading image data
    for n_x in 0..n_width {
        let n_wav_sample_value: u8 = o_wav.a_array[o_wav.n_header_end_index as usize + n_x as usize] as u8; 
        for n_y in 0..n_height {
            let o_pixel = a_image_buffer.get_pixel_mut(n_x, n_y);

            if(n_y == n_wav_sample_value.into()){
                *o_pixel = image::Rgb([255 as u8,255 as u8,255 as u8]);
            }else{
                *o_pixel = image::Rgb([0 as u8,0 as u8,0 as u8]);

            }
        }
    }

    // let s_wav_image_name = str::replace("{s_path_file_name}_waveform_image.png", "{s_path_file_name}", &s_path_file_name);
    // a_image_buffer.save(s_wav_image_name).unwrap();
    a_image_buffer.save("_waveform_image.png").unwrap();

    
    file.write_all(
        &o_wav.a_array
    ).unwrap();
    return true;


}


