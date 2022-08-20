

extern crate image;
// use std::default::Default;
use std::fs::File;

// use  image::buffer::EnumeratePixelsMut;
use image::ImageBuffer;
// use image::Pixel;
// use Container;
use rand::Rng;
// use image::GenericImageView;
// use image::{GenericImage, ImageFormat, Pixel, Rgba, RgbaImage, DynamicImage, ImageBuffer};
// use image::ImageBuffer;
use image::DynamicImage;
// use image::DynamicImage::EnumeratePixelsMut;

// fn f_create_fractal_png(){
//     let imgx = 800;
//     let imgy = 800;

//     let scalex = 3.0 / imgx as f32;
//     let scaley = 3.0 / imgy as f32;

//     // Create a new ImgBuf with width: imgx and height: imgy
//     let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

//     // Iterate over the coordinates and pixels of the image
//     for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
//         let r = (0.3 * x as f32) as u8;
//         let b = (0.3 * y as f32) as u8;
//         *pixel = image::Rgb([r, 0, b]);
//     }

//     // A redundant loop to demonstrate reading image data
//     for x in 0..imgx {
//         for y in 0..imgy {
//             let cx = y as f32 * scalex - 1.5;
//             let cy = x as f32 * scaley - 1.5;

//             let c = num_complex::Complex::new(-0.4, 0.6);
//             let mut z = num_complex::Complex::new(cx, cy);

//             let mut i = 0;
//             while i < 255 && z.norm() <= 2.0 {
//                 z = z * z + c;
//                 i += 1;
//             }

//             let pixel = imgbuf.get_pixel_mut(x, y);
//             let image::Rgb(data) = *pixel;
//             *pixel = image::Rgb([data[0], i as u8, data[2]]);
//         }
//     }

//     // Save the image as “fractal.png”, the format is deduced from the path
//     imgbuf.save("fractal.png").unwrap();
// }

fn f_create_green_png(){
    let n_width = 800;
    let n_height = 800;



    // Create a new ImgBuf with width: imgx and height: imgy
    let mut a_image_buffer = image::ImageBuffer::new(n_width, n_height);

    // Iterate over the coordinates and pixels of the image
    for (n_x, n_y, o_pixel) in a_image_buffer.enumerate_pixels_mut() {
        
        let n_r = 1.0 as f32;
        let n_g = 0.0 as f32;
        let n_b = 0.0 as f32;

        *o_pixel = image::Rgb([(n_r*255.0) as u8, (n_g*255.0) as u8, (n_b*255.0) as u8]);
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    a_image_buffer.save("green.png").unwrap();
}
fn f_create_random_png(){

    let mut rng = rand::thread_rng();

    let n_width = 4000; 
    let n_height = 4000;

    let mut a_image_buffer = image::ImageBuffer::new(n_width, n_height); 

    for(n_x, n_y, o_pixel) in a_image_buffer.enumerate_pixels_mut(){
        let n_r: f32 = rng.gen();
        let n_g: f32 = rng.gen();
        let n_b: f32 = rng.gen();
        *o_pixel = image::Rgb([
            (n_r*255.0) as u8,
            (n_g*255.0) as u8,
            (n_b*255.0) as u8,
        ])
    }
    
    a_image_buffer.save("random_image.png").unwrap();
}

// fn f_create_image_with_text(){

//     let mut rng = rand::thread_rng();

//     let n_width = 512; 
//     let n_height = 512;

//     // let mut a_image_buffer = ImageBuffer::<Rgb<u8>>::new(n_width, n_height); 
//     let mut a_image_buffer = image::ImageBuffer::new(n_width, n_height); 



//     // for(n_x, n_y, o_pixel) in a_image_buffer.enumerate_pixels_mut(){
//     //     let n_r: f32 = rng.gen();
//     //     let n_g: f32 = rng.gen();
//     //     let n_b: f32 = rng.gen();
//     //     *o_pixel = image::Rgb([
//     //         (n_r*255.0) as u8,
//     //         (n_g*255.0) as u8,
//     //         (n_b*255.0) as u8,
//     //     ])
//     // }

//         f_write_text_to_image(
//         // a_image_buffer.enumerate_pixels_mut(), 
//         a_image_buffer,
//         10, 
//         10, 
//         0xff00ff,
//         String::from("this is a test"),   
//     );
//     a_image_buffer.save("image_with_text.png").unwrap();
// }
use std::path::Path;
use serde; 
use serde_json;
use std::fs;
// extern crate rustc_serialize;
// use rustc_serialize::json::Json;
use std::io::Read;
fn f_write_text_to_vec(
    // a_image: &mut ImageBuffer<P, Container>,
    a_image: Vec<u8>,
    n_image_width: u32, 
    n_image_height: u32, 
    // a_image: &mut ImageBuffer,
    // a_pixels: EnumeratePixelsMut<'a, P: Pixel + 'a> ,
    n_x: u32, 
    n_y: u32,
    n_color: u32,
    // 11111111 10101010 11011101
    s_text: String
) -> Vec<u8>{
    let n_color_r = n_color >> (8*2) & 0b00000000_00000000_11111111;
    let n_color_g = n_color >> (8*1) & 0b00000000_00000000_11111111;
    let n_color_b = n_color >> (8*0) & 0b00000000_00000000_11111111;

    println!("color r,g,b, {},{},{}", n_color_r,n_color_g,n_color_b);

    // let o_file = fs::File::open("./../read_png_image/a.json")
    //     .expect("file should open read only");
    // let o_json: serde_json::Value = serde_json::from_reader(o_file)
    //     .expect("file should be proper JSON");
    // let a_a_b = o_json.get("a_a_b")
    //     .expect("file should have FirstName key");
    // println!("sadf{:?}", a_a_b);
    
    let n_index_channel = 0; 
    
    // while(n_index_channel< a_image.len()){
    //     println!("n_index_channel, n_val {},{}", n_index_channel, a_image[n_index_channel]);  
    // }
    let mut a_o_serde_json_object: Vec<serde_json::Value> = Vec::new();
    
    for (n_index, s_char) in s_text.chars().enumerate() {
        
        // println!("n_index, s_char, s_char.to_lowercase() {},{},{}", n_index, s_char, s_char.to_lowercase());
        let s_path_filename = str::replace("./../read_png_image/{s_char}.json", "{s_char}", &s_char.to_lowercase().to_string());
        let b_file_exists = Path::new(&s_path_filename).exists();
        if(b_file_exists){

            let o_json: Vec<serde_json::Value> = serde_json::Value::Null;

            for o_serde_json_object in &a_o_serde_json_object{
                let s_letter = o_serde_json_object.get("s_letter")
                .expect("s_letter key should exist");
                // println!("asdf {:?}", s_letter);
                if(s_letter == s_char.to_lowercase()){
                    o_json = o_serde_json_object;
                }
            }
            if(o_json != serde_json::Value::Null){

                
                let o_file = fs::File::open(s_path_filename)
                .expect("file should open read only");
                let mut o_json: serde_json::Value = serde_json::from_reader(o_file)
                .expect("file should be proper JSON");
                
                a_o_serde_json_object.push(o_json);
            }
            
            println!("asdf {:?}", o_json);

        }

    }
    return a_image
}

fn main() {
    // f_create_fractal_png(); 
    // f_create_green_png();
    // f_create_random_png();
    // f_create_image_with_text();
    // f_example();
    f_create_png_from_vec();
}

use image::RgbImage;

fn f_create_png_from_vec(){
    let n_width: u32 = 100;
    let n_height: u32 = 100; 
    let n_channels: u32 = 4;
    let mut a_image = vec!(255;0);//(n_width*n_height*n_channels)as usize);
    let mut rng = rand::thread_rng();

    let mut n_index_pixel = 0; 
    while(n_index_pixel < n_width*n_height){
        let n_rand: f32 = rng.gen();
        let n_val: u8 = (n_rand * 255.0) as u8;
        a_image.push(n_val);//r
        a_image.push(n_val);//g
        a_image.push(n_val);//b
        a_image.push(n_val);//a
        n_index_pixel+=1;        
    }
    
    a_image = f_write_text_to_vec(
        a_image,
        n_width, 
        n_height,
        10, 
        10, 
        0xff00ff,
        String::from("this is a TEST"),   
    );

    let a_image_buffer = RgbImage::from_raw(n_width as u32, n_height as u32, a_image)
        .expect("container should have the right size for the image dimensions");

    a_image_buffer.save("image_from_vec.png").unwrap();

}

fn f_a_xy(
    n_image_height: u32, 
    n_image_width: u32, 
    n_channels: u32, 
    n_index: u32,
)-> [u32;2]{
    // convert index to xy , for example in 3 channel , 2x2 image
    // [
    // [
    //     [
    //         [0r,1g,2b], [3r, 4g, 5b], 
    //     ],
    //     [
    //         [6r,7g,8b], [9r,10g,11b], 
    //     ]
    // ]
    //  index                0, 1, 2, 3, 4, 5, 6, 7, 8, 9,10,11, 
    //  chanel_index         r, g, b, r, g, b, r, g, b, r, g, b,
    //  pixel x              0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 
    //  pixel y              0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1
    //  index r,g,b,r,g
    // index 4 -> x: 1, y: 0, channel_index: 1 (g->green) 
    let n_x = (n_index as f32/ n_channels as f32) as u32 % n_image_width;
    let n_y = ((n_index as f32/ n_channels as f32) / n_image_width as f32) as u32;
    
    return [n_x, n_y];
}
fn f_n_index(
    n_image_height: u32, 
    n_image_width: u32, 
    n_channels: u32, 
    n_index_channel: u32, 
    n_x: u32, 
    n_y: u32
)-> u32{

    let n_index = n_y * n_image_width + n_x + n_channels + n_index_channel;

    return n_index;
}

use image::Pixel;
// fn f_example(){
//     let mut o_image = image::ImageBuffer::new(512,512); 
//     f_change_image(&o_image);
//     o_image.save("o_image.png").unwrap();

// }
// fn f_change_image(
//     o_image: &mut image::ImageBuffer<RGBA<u8>, Vec<RGBA<u8>>>,
//     // o_image:&mut ImageBuffer
// ){
//     for(n_x, n_y, o_pixel) in o_image.enumerate_pixels_mut(){
//         let n_r: u8 = n_x % 255;
//         let n_g: u8 = n_x % 122;
//         let n_b: u8 = n_x % 50;
//         *o_pixel = image::Rgb([
//             (n_r) as u8,
//             (n_g) as u8,
//             (n_b) as u8,
//         ])
//     }

// }