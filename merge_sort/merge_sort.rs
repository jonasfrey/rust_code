use rand::Rng;

extern crate image;

// use std::time::Instant;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;
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
    a_n_numbers: Vec<u8>,
    s_folder_path_name: &str
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

    
    let mut owned_string: String = "".to_owned();
    let mut s_file_name = str::replace("output_image_{}.png", "{}", &(n_ts_ms).to_string());
    let borrowed_string = s_folder_path_name; 
    owned_string.push_str(&borrowed_string);
    fs::create_dir_all(s_folder_path_name);

    owned_string.push_str(&s_file_name);

    // write it out to a file
    // let n_ts_ms = Instant::now().elapsed().as_millis();
    image.save(owned_string).unwrap();

    // ffmpeg -framerate 25 -pattern_type glob -i '*.png' -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p output.mp4


}
fn f_a_selectionsort(
    // a_n_numbers_original: Vec<u8>
    a_n_numbers: &mut Vec<u8>
){
    let n_ts_ms = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();

    let s_folder_path_name = str::replace("./selectionsort_images_{}/", "{}", &(n_ts_ms).to_string());

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
        f_draw_image(a_n_numbers.to_vec(), &s_folder_path_name);

        n_index = n_index + 1; 
    }
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

        f_draw_image(a_n_numbers.to_vec(), &s_folder_path_name);

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
            f_draw_image(a_n_numbers.to_vec(), &s_folder_path_name);

        }
    }

}


fn f_quicksort(
    a_n_numbers: &mut Vec<u8>
){

        
}
fn main() {

    let mut a_n_randnum = f_a_n_randnum(200);
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

    // f_a_selectionsort(&mut a_n_randnum); 
    // println!("{:?}", a_n_randnum);
    

}
