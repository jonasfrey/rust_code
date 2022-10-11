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
use eval::{Expr, to_value};
use std::collections::HashMap;


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
    let mut a_s_path_file : Vec<String> = Vec::new();
    let mut a_n_scale_x : Vec<f64> = Vec::new();
    let mut a_n_scale_y : Vec<f64> = Vec::new();

    while(n_i_a_o_manipulation < a_o_manipulation.len()){
        let o_manipulation = &a_o_manipulation[n_i_a_o_manipulation];
        let mut s_path_image_input = o_manipulation["s_path_image_input"].as_str().unwrap();
        let mut o_image_input = ImageReader::open(s_path_image_input).unwrap().decode().unwrap();
        let mut n_index_a_s_path_file = a_s_path_file.iter().position(|s| s == s_path_image_input);
        if(n_index_a_s_path_file == None){
            // println!("n_index_a_s_path_file {:?}", n_index_a_s_path_file);
            a_s_path_file.push(String::from(s_path_image_input));
            a_n_scale_x.push(o_image_input.width() as f64);
            a_n_scale_y.push(o_image_input.height() as f64);
            n_index_a_s_path_file = Some((a_s_path_file.len()-1) as usize);
        }else{
            n_index_a_s_path_file = Some(n_index_a_s_path_file.unwrap() as usize);
            a_s_path_file[n_index_a_s_path_file.unwrap()] = String::from(s_path_image_input);
            a_n_scale_x[n_index_a_s_path_file.unwrap()] = (o_image_input.width() as f64);
            a_n_scale_y[n_index_a_s_path_file.unwrap()] = (o_image_input.height() as f64);
        }

        println!("a_s_path_file {:?}", a_s_path_file);

        let mut n_translation_x_evaluated = 0 as f64; 
        if(o_manipulation["s_translation_x"]!= serde_json::Value::Null){
            let mut s_expression = String::from(o_manipulation["s_translation_x"].as_str().unwrap());
            for (n_index, s_path_file) in a_s_path_file.clone().iter().enumerate(){
                let mut s = vec![String::from(s_path_file.clone()), String::from(".n_scale_x")].join("");
                s_expression = s_expression.replace(&s[..], &(a_n_scale_x[n_index as usize].to_string())[..]);
                // println!("replacing {:?} with {:?}", s, &(a_n_scale_x[n_index as usize].to_string()));
                s = vec![String::from(s_path_file.clone()), String::from(".n_scale_y")].join("");
                s_expression = s_expression.replace(&s[..], &(a_n_scale_y[n_index as usize].to_string())[..]);

            }
            
            let mut o_expr = Expr::new(String::from(s_expression.clone()));
            // println!("s_expression {:?}", s_expression.clone());
            n_translation_x_evaluated = o_expr.exec().unwrap().as_f64().unwrap();
            // println!("n_translation_x_evaluated {:?}", n_translation_x_evaluated);
            // std::process::exit(1);
        }
        let mut n_translation_y_evaluated = 0 as f64;
        if(o_manipulation["s_translation_y"]!= serde_json::Value::Null){
            let mut s_expression = String::from(o_manipulation["s_translation_y"].as_str().unwrap());
            for (n_index, s_path_file) in a_s_path_file.clone().iter().enumerate(){
                let mut s = vec![String::from(s_path_file.clone()), String::from(".n_scale_x")].join("");
                s_expression = s_expression.replace(&s[..], &(a_n_scale_y[n_index as usize].to_string())[..]);
                // println!("replacing {:?} with {:?}", s, &(a_n_scale_y[n_index as usize].to_string()));
                s = vec![String::from(s_path_file.clone()), String::from(".n_scale_y")].join("");
                s_expression = s_expression.replace(&s[..], &(a_n_scale_y[n_index as usize].to_string())[..]);
            }
            
            let mut o_expr = Expr::new(String::from(s_expression.clone()));
            // println!("s_expression {:?}", s_expression.clone());
            n_translation_y_evaluated = o_expr.exec().unwrap().as_f64().unwrap();
            // println!("n_translation_y_evaluated {:?}", n_translation_y_evaluated);
            // std::process::exit(1);
        }




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
                n_translation_x_evaluated as i64, 
                n_translation_y_evaluated as i64
            );
        }

        if(o_manipulation["s_operation_name"].as_str().unwrap() == "crop"){

            o_image_output = image::DynamicImage::ImageRgba8(
                image::imageops::crop(
                    &mut o_image_input,
                    n_translation_x_evaluated as u32,
                    n_translation_y_evaluated as u32, 
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


            let mut n_translation_x = n_translation_x_evaluated;
            let mut n_translation_y = n_translation_y_evaluated;

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


            let mut n_translation_x = n_translation_x_evaluated;
            let mut n_translation_y = n_translation_y_evaluated;

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

        let mut s_path_image_output = o_manipulation["s_path_image_output"].as_str().unwrap();
        let mut n_index_a_s_path_file = a_s_path_file.iter().position(|s| s == s_path_image_output);
        if(n_index_a_s_path_file == None){
            // println!("n_index_a_s_path_file {:?}", n_index_a_s_path_file);
            a_s_path_file.push(String::from(s_path_image_output));
            a_n_scale_x.push(o_image_output.width() as f64);
            a_n_scale_y.push(o_image_output.height() as f64);
            n_index_a_s_path_file = Some((a_s_path_file.len()-1) as usize);
        }else{
            n_index_a_s_path_file = Some(n_index_a_s_path_file.unwrap() as usize);
            a_s_path_file[n_index_a_s_path_file.unwrap()] = String::from(s_path_image_output);
            a_n_scale_x[n_index_a_s_path_file.unwrap()] = (o_image_output.width() as f64);
            a_n_scale_y[n_index_a_s_path_file.unwrap()] = (o_image_output.height() as f64);
        }

        // println!("a_n_scale_x {:?}", a_n_scale_x);
        // println!("o_image_output.width() {:?}", o_image_output.width());
        n_i_a_o_manipulation+=1;
    }



}