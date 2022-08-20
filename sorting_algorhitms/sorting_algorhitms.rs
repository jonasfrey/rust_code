use rand::Rng;

extern crate image;

// use std::time::Instant;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;
use image::{ImageBuffer, RgbImage};


use std::fs::File;
use std::io::prelude::*;
use std::process::exit;
use std::convert::TryInto;


const N_IMG_WIDTH: u32 = 1000;
const N_IMG_HEIGTH: u32 = 1000;

const N_FPS = 60.0;


    
// This is the main function
fn f_a_n_randnum(
    n_length_a_n_randnum: u32
)
-> Vec<u8>
// -> Vec<u16>
// -> Vec<u32>
// -> Vec<u64>
{

    let mut rng = rand::thread_rng();

    let mut a_n_randnum: Vec<u8> = vec![]; 
    // let mut a_n_randnum: Vec<u16> = vec![]; 
    // let mut a_n_randnum: Vec<u32> = vec![]; 
    // let mut a_n_randnum: Vec<u64> = vec![]; 
    // let mut a_n_randnum : [u32; n_length_a_n_randnum] = [0; n_length_a_n_randnum]; 

    for n in 0..n_length_a_n_randnum {
        let n_rand: f32 = rng.gen();
        // let n_rand: f32 = 0.1234;
        // a_n_randnum[n] = (
        a_n_randnum.push(
            ((n_rand*(u8::MAX as f32)) as u8)
            // (n_rand*(u16::MAX as f32)) as u16
            // (n_rand*(u32::MAX as f32)) as u32
            // (n_rand*(u64::MAX as f32)) as u64
        ); 
    }

    return a_n_randnum;
}

fn f_draw_image(
    a_n_numbers: Vec<u8>,
    s_folder_path_name: &str, 
    n_index_to_highlight: u32
){

    let mut image: RgbImage = ImageBuffer::new(N_IMG_WIDTH, N_IMG_HEIGTH);
    let n_border_width = 2; 
    let mut n_index = 0; 
    while n_index < a_n_numbers.len(){

        let n_value = a_n_numbers[n_index]; 
        let n_height_val = ((N_IMG_HEIGTH as f32) / (u8::MAX as f32) * (n_value as f32)) as u32;
        
        let n_x_start = ((N_IMG_WIDTH as f32) / (a_n_numbers.len() as f32) * (n_index as f32)) as u32; 
        let n_x_end = ((N_IMG_WIDTH as f32) / (a_n_numbers.len() as f32) * ((n_index+1) as f32)) as u32;
        
        let n_y_start = 0; 
        let n_y_end = n_height_val;

        let mut n_y = 0; 
        while n_y < n_y_end{
            
            n_y = n_y + 1; 

            let mut n_x = n_x_start; 
            while n_x < n_x_end{
                if(
                    n_x <= n_x_start+n_border_width
                    ||
                    n_x >= n_x_end-n_border_width
                    ||
                    n_y <= n_y_start + n_border_width
                    ||
                    n_y >= n_y_end - n_border_width
                ){

                    if(n_index as u32 == n_index_to_highlight){
                        *image.get_pixel_mut(n_x,n_y) = image::Rgb([255,0,0]);
                    }else{
                        //border
                        *image.get_pixel_mut(n_x,n_y) = image::Rgb([0,n_value,0]);
                    }
                
                }else{
                    if(n_index as u32 == n_index_to_highlight){

                        *image.get_pixel_mut(n_x,n_y) = image::Rgb([255,0,0]);
                    }else{
                        *image.get_pixel_mut(n_x,n_y) = image::Rgb([255,255,255]);
                    }
                }
                
                n_x = n_x + 1;
            }
        }

        n_index = n_index + 1;

    }

    let n_ts_ms = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();

    
    let mut owned_string: String = "".to_owned();
    let mut s_file_name = str::replace("output_image_{}.png", "{}", &(n_ts_ms).to_string());
    let borrowed_string = s_folder_path_name; 
    owned_string.push_str(&borrowed_string);
    fs::create_dir_all(s_folder_path_name);

    owned_string.push_str(&s_file_name);

    // write it out to a file
    // let n_ts_ms = Instant::now().elapsed().as_millis();
    image.save(owned_string).unwrap();

    // ffmpeg -i audio.wav -framerate 60 -pattern_type glob -i '*.png' -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p output.mp4


}
fn f_a_selectionsort(
    // a_n_numbers_original: Vec<u8>
    a_n_numbers: &mut Vec<u8>
){

    // get the struct
    let mut o_wav = f_o_wav();

    let n_ts_ms = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();

    let s_folder_path_name = str::replace("./selectionsort_images_{}/", "{}", &(n_ts_ms).to_string());

    let mut n_index = 0; 
    let mut n_value = 0;

    while n_index < a_n_numbers.len(){
        n_value = a_n_numbers[n_index];     
    // for (n_index, n_value) in a_n_numbers.iter().enumerate() {
        println!("index:value {}:{}", n_index, n_value);

        let mut n_min_index = n_index;
        let mut n_min = a_n_numbers[n_min_index];
        
        let n_start = n_index; 
        let mut n_index2 = n_start;
        
        while n_index2 < a_n_numbers.len(){
            // f_draw_image((&mut a_n_numbers).to_vec());
            f_draw_image(a_n_numbers.to_vec(), &s_folder_path_name, n_index2 as u32);

            o_wav = f_add_samples(
                o_wav,// o_wav struct
                (a_n_numbers[n_index2] as f32 / (u8::MAX) as f32) as f32 * 5.0f32, //frequency
                String::from("sawtooth"), //wave type 'sawtooth' , 'sine' 
                (1000.0/N_FPS)as u32, // milliseconds
            );
            if(a_n_numbers[n_index2] < n_min){
                n_min_index = n_index2;
                n_min = a_n_numbers[n_min_index];

            }
            n_index2 = n_index2 + 1;

        }
        let n_tmp = a_n_numbers[n_index]; 
        a_n_numbers[n_index] = n_min;
        a_n_numbers[n_min_index] = n_tmp;

        n_index = n_index + 1; 
    }

    let s_folder_path_name_wavfile = str::replace("./selectionsort_images_{}/audio.wav", "{}", &(n_ts_ms).to_string());

    f_save_o_wav(
        o_wav,
        s_folder_path_name_wavfile
    );
    
    // ffmpeg -i audio.wav -framerate 60 -pattern_type glob -i '*.png' -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p output.mp4

    Command::new("ffmpeg")
        .arg("-i")
        .arg(s_folder_path_name_wavfile)
        .arg("-framerate")
        .arg(N_FPS)
        .arg("-pattern_type")
        .arg("glob")
        .arg("-i")
        .arg("'*.png'")
        .arg("-c:v")
        .arg("libx264")
        .arg("-profile:v")
        .arg("high")
        .arg("-crf")
        .arg("20")
        .arg("-pix_fmt")
        .arg("yuv420p")
        .arg("output.mp4")
        .output()
        .expect("failed to execute process")

    // return a_n_numbers;
}
fn f_a_insertsort(
    a_n_numbers: &mut Vec<u8>
){

    let n_ts_ms = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();

    let s_folder_path_name = str::replace("./insertsort_images{}/", "{}", &(n_ts_ms).to_string());


    let mut n_index = 1; 

    while n_index < a_n_numbers.len(){
        
        let n_to_insert = a_n_numbers[n_index]; //1
        println!("n_to_insert: {:?}", n_to_insert);

        let mut n_i = 0; 
        while n_i < n_index{
            println!("{:?} <? {:?} ", n_to_insert,a_n_numbers[n_i]);

            if(n_to_insert < a_n_numbers[n_i]){
                
                let mut n_index_reverse = n_index; 

                while n_index_reverse > n_i{

                    a_n_numbers[n_index_reverse] = a_n_numbers[n_index_reverse-1];

                    n_index_reverse = n_index_reverse -1;
                }
                println!("shifted: {:?}", a_n_numbers);


                a_n_numbers[n_i] = n_to_insert;
                println!("inserted: {:?}", a_n_numbers);
                break;
            } 
            n_i+=1;
        }

        f_draw_image(a_n_numbers.to_vec(), &s_folder_path_name, 0);

        n_index = n_index +1;
    }

}

fn f_bubblesort(
    a_n_numbers: &mut Vec<u8>
){

    let n_ts_ms = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();
    let s_folder_path_name = str::replace("./bubblesort_images{}/", "{}", &(n_ts_ms).to_string());
    
    let mut b_fullysorted = false; 
    while !b_fullysorted{
        // bubble phase 
        b_fullysorted = true;
        let mut n_index = 0; 
        
        while n_index < (a_n_numbers.len()-1){
        
            if(a_n_numbers[n_index+1] < a_n_numbers[n_index]){
                let n_tmp = a_n_numbers[n_index+1]; 
                println!("{:?}<-swap->{:?}", a_n_numbers[n_index], a_n_numbers[n_index+1]);
                a_n_numbers[n_index+1] = a_n_numbers[n_index];
                a_n_numbers[n_index] = n_tmp;
                b_fullysorted = false;
            }

            n_index +=1;
            f_draw_image(a_n_numbers.to_vec(), &s_folder_path_name, 0);

        }
    }

}


fn f_quicksort(
    a_n_numbers: &mut Vec<u8>
){

        
}
fn main() {

    let mut a_n_randnum = f_a_n_randnum(100);
    println!("{:?}", a_n_randnum);

    // f_a_selectionsort(&mut a_n_randnum);
    // println!("{:?}", a_n_randnum);

    // let mut a_n_static : Vec<u8> = vec![5,9,1,4,6,23,24,25,26,27];

    // let mut a_n_unordered = a_n_static
    let mut a_n_unordered = a_n_randnum;

    // f_a_insertsort(&mut a_n_unordered);
    // println!("{:?}", a_n_unordered);

    // f_bubblesort(&mut a_n_unordered);
    // println!("{:?}", a_n_unordered);


    f_a_selectionsort(&mut a_n_unordered);
    println!("{:?}", a_n_unordered);


    // f_test_wav();

    // f_a_selectionsort(&mut a_n_randnum); 
    // println!("{:?}", a_n_randnum);
    

}



fn  f_test_wav(){
    let mut o_wav = f_o_wav();
    for n in 0..100{

        o_wav = f_add_samples(
            o_wav,// o_wav struct
            (5.0/100.0 * n as f32) as f32, //frequency
            String::from("sawtooth"), //wave type 'sawtooth' , 'sine' 
            (1000.0/N_FPS)as u32, // milliseconds
        );
    }
    let mut rng = rand::thread_rng();

    for n in 0..100{
        let n_rand: f32 = rng.gen();

        o_wav = f_add_samples(
            o_wav,// o_wav struct
            n_rand + 5.0, //frequency
            String::from("sawtooth"), //wave type 'sawtooth' , 'sine' 
            (1000.0/N_FPS)as u32, // milliseconds
        );
    }


    f_save_o_wav(
        o_wav,
        String::from("test.wav")
    ); 
}
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

struct O_wav{
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
fn f_o_wav()-> O_wav {

    let s_riff_mark = [b'R', b'I', b'F', b'F'];  // ChunkID; "RIFF"
    let s_wave_mark = [b'W',b'A',b'V',b'E']; // Format "WAVE"

    let s_fmt_mark = [b'f',b'm',b't', b' ']; // Subchunk1ID "fmt "
    let n_index_now: u32 = 16; //subchunk1 size , index until now

    let n_audio_format_type: u32 = 1; // 1 = PCM , 
    let n_num_channels: u32 = 1; // number of channels, 1 = mono

    let n_samples_per_second_aka_samplerate: u32 = 44100; // 44100 common, 4800 CD quality, 88200 better quality, number of samples per second!
    let n_bits_per_sample: u32 = 16; 
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

fn f_add_samples(
    mut o_wav: O_wav,
    n_freq_normalized: f32, // eg. 432, //frequency
    s_wavetype: String, // eg. 'sawtooth' , 'sine'  
    n_milliseconds: u32, // eg 100
)-> O_wav{

    // calculate how many samples to add 
    let n_samples_to_add = ((o_wav.n_samples_per_second_aka_samplerate as f32) / 1000.0 * (n_milliseconds as f32))as u32;
    let mut n_count_sample = 0;
    let mut n_inc: f32 = n_freq_normalized;
    let mut n_time: f32 = 0.0;
    while n_count_sample < n_samples_to_add {
        n_time = (n_time + n_inc) % (u16::MAX) as f32;
        // n_radians_start = n_radians;
        // let n_u16 = (((n_radians_start.sin() * (u16::MAX) as f32) + ((u16::MAX as f32)/2.0))) as u16;
        // let n_u16 = n_rand_u16;
        let a_n_u16 = f_a_convert_u16_to_2_u8_values(n_time as u16);
        o_wav.a_array.push(a_n_u16[0+0]);
        o_wav.a_array.push(a_n_u16[0+1]);

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


fn f_save_o_wav(
    o_wav: O_wav, 
    s_path_file_name: String, 
)-> bool{
    let mut file = File::create(s_path_file_name).unwrap();
    file.write_all(
        &o_wav.a_array
    ).unwrap();
    return true;


}