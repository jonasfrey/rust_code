use rand::Rng;

extern crate image;

// use std::time::Instant;
use std::time::{SystemTime, UNIX_EPOCH};

use image::{ImageBuffer, RgbImage};

const N_IMG_WIDTH: u32 = 512;
const N_IMG_HEIGTH: u32 = 512;

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
    a_n_numbers: Vec<u8>
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
                    //border
                    *image.get_pixel_mut(n_x,n_y) = image::Rgb([0,n_value,0]);
                
                }else{

                    *image.get_pixel_mut(n_x,n_y) = image::Rgb([255,255,255]);
                }
                
                n_x = n_x + 1;
            }
        }

        n_index = n_index + 1;

    }

    let n_ts_ms = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();

    // write it out to a file
    // let n_ts_ms = Instant::now().elapsed().as_millis();
    image.save(str::replace("output_image_{}.png", "{}", &(n_ts_ms).to_string()) ).unwrap();

    ffmpeg -framerate 25 -pattern_type glob -i '*.png' -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p output.mp4


}
fn f_a_selectionsort(
    // a_n_numbers_original: Vec<u8>
    a_n_numbers: &mut Vec<u8>
){
    let mut n_index = 0; 
    let mut n_value =0; 
    while n_index < a_n_numbers.len(){
        n_value = a_n_numbers[n_index];     
    // for (n_index, n_value) in a_n_numbers.iter().enumerate() {
        println!("index:value {}:{}", n_index, n_value);

        let mut n_min_index = n_index;
        let mut n_min = a_n_numbers[n_min_index];
        
        let n_start = n_index; 
        let mut n_index2 = n_start;
        while n_index2 < a_n_numbers.len(){
            if(a_n_numbers[n_index2] < n_min){
                n_min_index = n_index2;
                n_min = a_n_numbers[n_min_index];
            }
            n_index2 = n_index2 + 1;
        }
        let n_tmp = a_n_numbers[n_index]; 
        a_n_numbers[n_index] = n_min;
        a_n_numbers[n_min_index] = n_tmp;
        
        // f_draw_image((&mut a_n_numbers).to_vec());
        f_draw_image(a_n_numbers.to_vec());

        n_index = n_index + 1; 
    }
    // return a_n_numbers;
}
fn f_a_insertsort(){

}

fn main() {

    let mut a_n_randnum = f_a_n_randnum(10);
    println!("{:?}", a_n_randnum);

    f_a_selectionsort(&mut a_n_randnum);
    println!("{:?}", a_n_randnum);

    // f_a_selectionsort(&mut a_n_randnum); 
    // println!("{:?}", a_n_randnum);
    

}
