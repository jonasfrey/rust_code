use std::io::Cursor;
use text_to_png::TextRenderer;
use image::io::Reader as ImageReader;
use image::GenericImageView;
use std::sync::Once;
use std::fs::File;
use std::io::Write;
use serde_json::{Result, Value};
use std::env;
use image::{ImageBuffer, Rgba, RgbaImage};
use std::process;
use std::io::Read;
use std::io;

// use woff2::decode::{convert_woff2_to_ttf, is_woff2};

fn f_print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


// // example input json 



#[tokio::main]
async fn main(){

    // scale == size , how many pixels width
    // translation == position, a fixed point somewhere
    // rotation == rotation

    let a_s_arg: Vec<String> = env::args().collect();
    let mut s_first_arg = a_s_arg[1].to_owned();
    println!("s_first_arg {:?}", s_first_arg);

    println!("s_first_arg {}", s_first_arg);
    let o_param: Value = serde_json::from_str(&s_first_arg).unwrap();


    let a_o_manipulation = o_param["a_o_manipulation"].as_array().unwrap();
    let mut n_i_a_o_manipulation = 0; 
    while(n_i_a_o_manipulation < a_o_manipulation.len()){
        let o_manipulation = &a_o_manipulation[n_i_a_o_manipulation];
        let mut o_image_input = ImageReader::open(o_manipulation["s_path_image_input"].as_str().unwrap()).unwrap().decode().unwrap();
        let mut o_image_output = o_image_input.clone();
        if(o_manipulation["s_operation_name"].as_str().unwrap() == "resize"){

            o_image_output = image::DynamicImage::ImageRgba8(
                image::imageops::resize(
                    &mut o_image_input,
                    o_manipulation["n_scale_x"].as_u64().unwrap() as u32, 
                    o_manipulation["n_scale_y"].as_u64().unwrap() as u32,
                    image::imageops::FilterType::Nearest,
                    // image::imageops::FilterType::Triangle,
                    // image::imageops::FilterType::CatmullRom,
                    // image::imageops::FilterType::Gaussian,
                    // image::imageops::FilterType::Lanczos3,
                )
            );
        }
        
        if(o_manipulation["s_operation_name"].as_str().unwrap() == "overlay"){

            let mut o_image_foreground = ImageReader::open(o_manipulation["s_path_image_foreground"].as_str().unwrap()).unwrap().decode().unwrap();
            o_image_output = o_image_input.clone();
            image::imageops::overlay(
                &mut o_image_output,
                &o_image_foreground,
                o_manipulation["n_translation_x"].as_i64().unwrap() as i64, 
                o_manipulation["n_translation_y"].as_i64().unwrap() as i64
            );
        }

        if(o_manipulation["s_operation_name"].as_str().unwrap() == "crop"){

            o_image_output = image::DynamicImage::ImageRgba8(
                image::imageops::crop(
                    &mut o_image_input,
                    o_manipulation["n_translation_x"].as_i64().unwrap().try_into().unwrap(),
                    o_manipulation["n_translation_y"].as_i64().unwrap().try_into().unwrap(), 
                    o_manipulation["n_scale_x"].as_i64().unwrap().try_into().unwrap(),
                    o_manipulation["n_scale_y"].as_i64().unwrap().try_into().unwrap(),
                ).to_image()
            );
        }
        if(o_manipulation["s_operation_name"].as_str().unwrap() == "blur"){        
            o_image_output = image::DynamicImage::ImageRgba8(
                image::imageops::blur(
                    &mut o_image_input,
                    o_manipulation["n_float_blur_sigma"].as_f64().unwrap() as f32 
                )
            );
        }
        if(o_manipulation["s_operation_name"].as_str().unwrap() == "rotate"){        
            let mut n_rotation_normalized = o_manipulation["n_rotation_normalized"].as_f64().unwrap() as f64;
            let mut n_o_image_input_scale_x = o_image_input.width();
            let mut n_o_image_input_scale_y = o_image_input.height();
                
            let mut o_image_buffer = ImageBuffer::new(
                n_o_image_input_scale_x,
                n_o_image_input_scale_y
            );
        
            let mut n_cos = (std::f64::consts::TAU * n_rotation_normalized).cos();
            let mut n_sin = (std::f64::consts::TAU * n_rotation_normalized).sin();

            let mut n_translation_x = 0 as f64; 
            if(o_manipulation["n_translation_x"]!= serde_json::Value::Null){
                n_translation_x = (o_manipulation["n_translation_x"].as_f64().unwrap())
            }
            let mut n_translation_y = 0 as f64; 
            if(o_manipulation["n_translation_y"]!= serde_json::Value::Null){
                n_translation_y = (o_manipulation["n_translation_y"].as_f64().unwrap())
            }

            for n_y in 0..n_o_image_input_scale_x{
                for n_x in 0..n_o_image_input_scale_y {

                    // p'x = cos(theta) * (px-ox) - sin(theta) * (py-oy) + ox
                    // p'y = sin(theta) * (px-ox) + cos(theta) * (py-oy) + oy
                    let n_x_f64 = n_x as f64;
                    let n_y_f64 = n_y as f64;

                    let n_new_x = n_cos * (n_x_f64-n_translation_x) - n_sin * (n_y_f64-n_translation_y) + n_translation_x;
                    let n_new_y = n_sin * (n_x_f64-n_translation_x) + n_cos * (n_y_f64-n_translation_y) + n_translation_y;

                    if(
                        (n_new_x  >= 0.into() && n_new_x < n_o_image_input_scale_x.into())
                        &&
                        (n_new_y  >= 0.into() && n_new_y < n_o_image_input_scale_y.into())

                    ){
                        let p = o_image_input.get_pixel(n_x as u32 , n_y as u32 );
                        // println!("p {:?}", p);
                        o_image_buffer.put_pixel( n_new_x as u32, n_new_y as u32 ,p);
                    }
                    // else{
                    //      println!("x|y {:?}|{:?}", n_new_x, n_new_y);
                    // }
                }
            }
            o_image_output = image::DynamicImage::ImageRgba8(
                o_image_buffer
            )
        }
        if(o_manipulation["s_operation_name"].as_str().unwrap() == "flip"){        
            let mut n_o_image_input_scale_x = o_image_input.width();
            let mut n_o_image_input_scale_y = o_image_input.height();
                
            let mut o_image_buffer = ImageBuffer::new(
                n_o_image_input_scale_x,
                n_o_image_input_scale_y
            );
            let mut s_axis = String::from("x"); 
            if(o_manipulation["s_axis"]!= serde_json::Value::Null){
                s_axis = String::from(o_manipulation["s_axis"].as_str().unwrap())
            }

            let mut n_translation_x = n_o_image_input_scale_x;
            let mut n_translation_y = 0;

            if(s_axis == "x"){
                n_translation_x = n_o_image_input_scale_x;
                n_translation_y = 0;
            }else{
                n_translation_x = 0;
                n_translation_y = n_o_image_input_scale_y;
            }


            for n_y in 0..n_o_image_input_scale_x{
                for n_x in 0..n_o_image_input_scale_y {

                    // p'x = cos(theta) * (px-ox) - sin(theta) * (py-oy) + ox
                    // p'y = sin(theta) * (px-ox) + cos(theta) * (py-oy) + oy
                    let n_x_f64 = n_x as f64;
                    let n_y_f64 = n_y as f64;
                    let mut n_new_x = 0 as f64; 
                    let mut n_new_y = 0 as f64; 
                    if(s_axis == "x"){
                        n_new_x = n_x_f64*(-1 as f64) + n_translation_x as f64;
                        n_new_y = n_y_f64;
                    }else{
                        n_new_x = n_x_f64;
                        n_new_y = n_y_f64*(-1 as f64) + n_translation_y as f64;
                    }


                    if(
                        (n_new_x  >= 0.into() && n_new_x < n_o_image_input_scale_x.into())
                        &&
                        (n_new_y  >= 0.into() && n_new_y < n_o_image_input_scale_y.into())
                    ){
                        let p = o_image_input.get_pixel(n_x as u32 , n_y as u32 );
                        // println!("p {:?}", p);
                        o_image_buffer.put_pixel( n_new_x as u32, n_new_y as u32 ,p);
                    }
                    // else{
                    //      println!("x|y {:?}|{:?}", n_new_x, n_new_y);
                    // }
                }
            }
            o_image_output = image::DynamicImage::ImageRgba8(
                o_image_buffer
            )
        }   

        if(o_manipulation["s_operation_name"].as_str().unwrap() == "transformation_matrix"){        
            let mut n_o_image_input_scale_x = o_image_input.width();
            let mut n_o_image_input_scale_y = o_image_input.height();
                
            let mut o_image_buffer = ImageBuffer::new(
                n_o_image_input_scale_x,
                n_o_image_input_scale_y
            );
            let mut n_translation_x = 0 as f64; 
            if(o_manipulation["n_translation_x"]!= serde_json::Value::Null){
                n_translation_x = (o_manipulation["n_translation_x"].as_f64().unwrap())
            }
            let mut n_translation_y = 0 as f64; 
            if(o_manipulation["n_translation_y"]!= serde_json::Value::Null){
                n_translation_y = (o_manipulation["n_translation_y"].as_f64().unwrap())
            }

            let mut a_n_f64_x : Vec<f64> = vec![
                o_manipulation["a_a_n"].as_array().unwrap()[0].as_array().unwrap()[0].as_f64().unwrap(),
                o_manipulation["a_a_n"].as_array().unwrap()[0].as_array().unwrap()[1].as_f64().unwrap(),
            ];

            let mut a_n_f64_y : Vec<f64> = vec![
                o_manipulation["a_a_n"].as_array().unwrap()[1].as_array().unwrap()[0].as_f64().unwrap(),
                o_manipulation["a_a_n"].as_array().unwrap()[1].as_array().unwrap()[1].as_f64().unwrap(),
            ];
            // println!("a_n_f64_x {:?}", a_n_f64_x);
            // println!("a_n_f64_y {:?}", a_n_f64_y);
            // std::process::exit(0);
            
            for n_y in 0..n_o_image_input_scale_x{
                for n_x in 0..n_o_image_input_scale_y {

                    let n_x_f64 = n_x as f64;
                    let n_y_f64 = n_y as f64;


                    let n_new_x = a_n_f64_x[0] * (n_x_f64-n_translation_x) - a_n_f64_x[1] * (n_y_f64-n_translation_y) + n_translation_x;
                    let n_new_y = a_n_f64_y[0] * (n_x_f64-n_translation_x) + a_n_f64_y[1] * (n_y_f64-n_translation_y) + n_translation_y;


                    if(
                        (n_new_x  >= 0.into() && n_new_x < n_o_image_input_scale_x.into())
                        &&
                        (n_new_y  >= 0.into() && n_new_y < n_o_image_input_scale_y.into())
                    ){
                        let p = o_image_input.get_pixel(n_x as u32 , n_y as u32 );
                        // println!("p {:?}", p);
                        o_image_buffer.put_pixel( n_new_x as u32, n_new_y as u32 ,p);
                    }
                    // else{
                    //      println!("x|y {:?}|{:?}", n_new_x, n_new_y);
                    // }
                }
            }
            o_image_output = image::DynamicImage::ImageRgba8(
                o_image_buffer
            )
        }   
        
        o_image_output.save(o_manipulation["s_path_image_output"].as_str().unwrap()).unwrap();
        
        n_i_a_o_manipulation+=1;
    }



}