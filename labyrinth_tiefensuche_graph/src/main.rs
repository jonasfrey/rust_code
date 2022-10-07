


#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// For reading and opening files
mod test_module;
mod constants_chars_a_u8;

use image::{ImageBuffer, Rgb};


use std::str;
use std::path::Path;
use std::fs::File;

use std::time::{SystemTime, UNIX_EPOCH, Instant, Duration};
use std::f64::consts::TAU;

use rand::Rng;
use std::io::BufWriter;
use autopilot::mouse;
// use buttons::Mouse;
// use readmouse::Mouse;
// use ggez::input::mouse;


use show_image::{event, ImageView, ImageInfo, create_window};


#[derive(Debug)]
struct O_point_2d{ 
    n_x:u32,
    n_y:u32,
}
#[derive(Debug)]
struct O_spatialproperty{

    o_point_2d_current: O_point_2d, 
    o_point_2d_velocity: O_point_2d, 
    o_point_2d_acceleration: O_point_2d,

 }

 fn f_calculate_o_spatialproperty(
    o_spatialproperty: &mut O_spatialproperty
 ){
    o_spatialproperty.o_point_2d_velocity.n_x += o_spatialproperty.o_point_2d_acceleration.n_x;
    o_spatialproperty.o_point_2d_velocity.n_y += o_spatialproperty.o_point_2d_acceleration.n_y;

    o_spatialproperty.o_point_2d_current.n_x += o_spatialproperty.o_point_2d_velocity.n_x;
    o_spatialproperty.o_point_2d_current.n_y += o_spatialproperty.o_point_2d_velocity.n_y;
 }
 #[derive(Debug)]
 struct O_object_2d{
    s_name: String,
    o_translation: O_spatialproperty,
    o_rotation: O_spatialproperty,
    o_scale: O_spatialproperty,
 }

fn f_calculate_o_object_2d(
    o_object_2d: &mut O_object_2d
){

    f_calculate_o_spatialproperty(&mut o_object_2d.o_translation);
    f_calculate_o_spatialproperty(&mut o_object_2d.o_rotation);
    f_calculate_o_spatialproperty(&mut o_object_2d.o_scale);
}

fn f_a_color_rgba_mixed(
    a_color_rgba_1: Vec<u8>,
    a_color_rgba_2: Vec<u8>,
) -> Vec<u8>{

    let n_r_1 = a_color_rgba_1[0 as usize] as f32; //rA
    let n_g_1 = a_color_rgba_1[1 as usize] as f32; //gA
    let n_b_1 = a_color_rgba_1[2 as usize] as f32; //bA
    let n_a_1 = a_color_rgba_1[3 as usize] as f32; //n_a_1
    let n_r_2 = a_color_rgba_2[0 as usize] as f32; //rB
    let n_g_2 = a_color_rgba_2[1 as usize] as f32; //gB
    let n_b_2 = a_color_rgba_2[2 as usize] as f32; //bB
    let n_a_2 = a_color_rgba_2[3 as usize] as f32; //aB

    let n_max : f32 = 255.0;

    // let n_r_mixed = (n_r_1 * n_a_1 / n_max) + (n_r_2 * n_b_2 * (n_max - n_a_1) / (n_max*n_max));
    // let n_g_mixed = (n_g_1 * n_a_1 / n_max) + (n_g_2 * n_b_2 * (n_max - n_a_1) / (n_max*n_max));
    // let n_b_mixed = (n_b_1 * n_a_1 / n_max) + (n_b_2 * n_b_2 * (n_max - n_a_1) / (n_max*n_max));
    // let n_a_mixed = n_a_1 + (n_b_2 * (n_max - n_a_1) / n_max);


    let n_a_mixed = n_a_1 + (n_a_2 * (n_max - n_a_1) / n_max);
    let n_r_mixed = (n_r_1 * n_a_1 + n_r_2 * n_a_2 * (n_max - n_a_1) / n_max)/n_a_mixed ;
    let n_g_mixed = (n_g_1 * n_a_1 + n_g_2 * n_a_2 * (n_max - n_a_1) / n_max)/n_a_mixed ;
    let n_b_mixed = (n_b_1 * n_a_1 + n_b_2 * n_a_2 * (n_max - n_a_1) / n_max)/n_a_mixed ;

    return vec![
        n_r_mixed as u8,
        n_g_mixed as u8,
        n_b_mixed as u8,
        n_a_mixed as u8
    ]
    // return a_color_rgba_mixed;
}
fn f_draw_string(
    a_n_u8_pixel: &mut Vec<u8>,
    n_vector_pixels_x : u32,
    n_vector_pixels_y : u32,
    n_channels: u32,
    n_position_x : u32,
    n_position_y : u32,
    n_font_size_scale : f32,
    a_color : &Vec<u8>, 
    s_string: String, 
){

    let a_n_char = s_string.as_bytes();
    let mut n_position_x_char = n_position_x;


    // f_draw_char(
    //     a_n_u8_pixel,
    //     n_vector_pixels_x,
    //     n_vector_pixels_y ,
    //     n_channels,
    //     n_position_x + 0 * 50, 
    //     n_position_y ,
    //     n_font_size_scale, 
    //     a_color ,
    //     a_n_char[0],
    // );
    // f_draw_char(
    //     a_n_u8_pixel,
    //     n_vector_pixels_x,
    //     n_vector_pixels_y ,
    //     n_channels,
    //     n_position_x + 1 * 50, 
    //     n_position_y ,
    //     n_font_size_scale, 
    //     a_color ,
    //     a_n_char[1],
    // );



    let mut n_len = a_n_char.len();
    let mut n_i = 0; 
    while(n_i < n_len){
        let n_char = a_n_char[n_i];
        let n_char_index = n_char - 32;
        let n_size_x_char = constants_chars_a_u8::a_n_size_x[n_char_index as usize];
        // let n_size_y_char = constants_chars_a_u8::a_n_size_y[n_char_index as usize];
        f_draw_char(
            a_n_u8_pixel,
            n_vector_pixels_x,
            n_vector_pixels_y ,
            n_channels,
            n_position_x_char, 
            n_position_y ,
            n_font_size_scale, 
            a_color ,
            n_char,
        );
        // n_position_x_char += n_size_x_char;
        n_position_x_char += 50;
        n_i+=1;
    }
    // for n_char in a_n_char.iter() {
    //     println!("n_char {:?}", n_char);
    //     let n_char_index = n_char - 32;
    //     // let n_size_x_char = constants_chars_a_u8::a_n_size_x[n_char_index as usize];
    //     // let n_size_y_char = constants_chars_a_u8::a_n_size_y[n_char_index as usize];
    //     f_draw_char(
    //         a_n_u8_pixel,
    //         n_vector_pixels_x,
    //         n_vector_pixels_y ,
    //         n_channels,
    //         n_position_x_char, 
    //         n_position_y ,
    //         n_font_size_scale, 
    //         a_color ,
    //         *n_char,
    //     );
    //     // n_position_x_char += n_size_x_char;
    //     n_position_x_char += 10;
    // }
}

fn f_draw_char(
    a_n_u8_pixel: &mut Vec<u8>,
    n_vector_pixels_x : u32,
    n_vector_pixels_y : u32,
    n_channels: u32,
    n_position_x : u32,
    n_position_y : u32,
    n_font_size_scale : f32,
    a_color : &Vec<u8>, 
    // s_char: String, 
    n_char: u8, 
){


    // let n_char = s_char.as_bytes()[0];
    // println!("nchar {:?}", n_char);
    let n_char_index = n_char - 32;
    let n_size_x_char = constants_chars_a_u8::a_n_size_x[n_char_index as usize] as f32;
    let n_size_y_char = constants_chars_a_u8::a_n_size_y[n_char_index as usize] as f32;
    let n_channels_char = constants_chars_a_u8::n_channels as u32;
    let n_char_start_index = constants_chars_a_u8::a_n_start_index_chars_information[n_char_index as usize] as usize;
    
    let mut n_x = 0 as f32; 
    let mut n_y = 0 as f32; 
    let mut n_x_normalized = 0 as f32;
    let mut n_y_normalized = 0 as f32;

    let mut n_x_char = 0 as f32;
    let mut n_y_char = 0 as f32;

    let mut n_channel = 0;
    let n_char_target_channel_index = 0;
    while(n_x < (n_size_x_char * n_font_size_scale)){
        n_x_normalized = n_x / (n_size_x_char * n_font_size_scale);
        n_x_char = n_x_normalized * n_size_x_char;
        n_y = 0.0;
        while(n_y < (n_size_y_char * n_font_size_scale)){
            n_y_normalized = n_y / (n_size_y_char * n_font_size_scale);
            n_y_char = n_y_normalized * n_size_y_char;

            // println!("n_pixel_index :{:?}", n_pixel_index);
            let n_index_pixel = 
            ((((n_position_y as i32) + (n_y as i32)) as u32) * n_vector_pixels_x * n_channels) +
            ((((n_position_x as i32) + (n_x as i32)) as u32) * n_channels);
            let n_index_pixel_char = 
            n_char_start_index as u32+ 
            ((n_y_char as u32) * (n_size_x_char as u32) * n_channels_char) +
            ((n_x_char as u32) * n_channels_char);

            let a_color_rgba_1 = vec![
                a_n_u8_pixel[(n_index_pixel+0) as usize],
                a_n_u8_pixel[(n_index_pixel+1) as usize],
                a_n_u8_pixel[(n_index_pixel+2) as usize],
                a_n_u8_pixel[(n_index_pixel+3) as usize]
            ];
            let mut n_val = 0; 
            if(
                constants_chars_a_u8::a_chars_information[(n_index_pixel_char+0) as usize] > 0
            ){
                n_val = 255;
            }

            let a_color_char = vec![
                // constants_chars_a_u8::a_chars_information[(n_index_pixel_char+0) as usize],
                // constants_chars_a_u8::a_chars_information[(n_index_pixel_char+1) as usize],
                // constants_chars_a_u8::a_chars_information[(n_index_pixel_char+2) as usize],
                // constants_chars_a_u8::a_chars_information[(n_index_pixel_char+3) as usize]
                n_val,
                n_val,
                n_val,
                constants_chars_a_u8::a_chars_information[(n_index_pixel_char+3) as usize],
            ];
            
            let a_color_rgba_mixed = f_a_color_rgba_mixed(a_color_rgba_1, a_color_char);

            a_n_u8_pixel[(n_index_pixel+0) as usize] = a_color_rgba_mixed[0];
            a_n_u8_pixel[(n_index_pixel+1) as usize] = a_color_rgba_mixed[1];
            a_n_u8_pixel[(n_index_pixel+2) as usize] = a_color_rgba_mixed[2];
            a_n_u8_pixel[(n_index_pixel+3) as usize] = a_color_rgba_mixed[3];

            

            n_y+=1.0;
        }

        n_x+=1.0;
    }
}

fn f_draw_circle(
    a_n_u8_pixel: &mut Vec<u8>,
    n_vector_pixels_x : u32,
    n_vector_pixels_y : u32,
    n_channels: u32,
    n_position_x : u32,
    n_position_y : u32,
    n_size_x : u32,
    n_size_y : u32,
    a_color : &Vec<u8>
){
    let mut n_radius_x = 0; 
    let mut n_radius_y = 0;
    
    // simple 
    let mut n_radius = 0;
    let mut n_radians_per_step: f64 = 0.0;
    let mut n_steps = 0;
    // x        /radius 1 / steps 1 

    //  x       /radius 2 / steps 4
    // x x
    //  x

    //  xx     /radius 2.5 / steps 8
    // x  x
    // x  x
    //  xx


    // lets do steps = radius*radius
    
    while(n_radius < n_size_x){
        let n_number_of_pixels_for_circumfence = 2.0 as f64 * n_radius as f64 * (TAU/2.0);
        // n_steps = n_radius * (n_radius/3); 
        n_steps = n_number_of_pixels_for_circumfence as u32;

        n_radians_per_step = TAU / (( n_steps ) as f64);

        let mut n_i_step = 0; 
        while(n_i_step < n_steps){
            let n_radians = (n_radians_per_step * (n_i_step as f64) as f64);
            let n_x = n_radius as f64 * ((n_radians)as f64).sin();
            let n_y = n_radius as f64 * ((n_radians)as f64).cos();
                        
            let n_index_pixel = 
            ((((n_position_y as i32) + (n_y as i32)) as u32) * n_vector_pixels_x * n_channels) +
            ((((n_position_x as i32) + (n_x as i32)) as u32) * n_channels);

            let mut n_channel = 0;
            let a_color_rgba_1 = vec![
                a_n_u8_pixel[(n_index_pixel+0) as usize],
                a_n_u8_pixel[(n_index_pixel+1) as usize],
                a_n_u8_pixel[(n_index_pixel+2) as usize],
                a_n_u8_pixel[(n_index_pixel+3) as usize]
            ];
            let a_color_rgba_2 = a_color;
            let a_color_rgba_mixed = f_a_color_rgba_mixed(a_color_rgba_1, a_color.clone());
            while(n_channel < n_channels){
                
                a_n_u8_pixel[(n_index_pixel+n_channel) as usize] = a_color_rgba_mixed[n_channel as usize];

                // a_n_u8_pixel[(n_index_pixel+n_channel) as usize] = a_color[n_channel as usize];
                n_channel+=1;
            }

            n_i_step += 1;
        }

        n_radius+=1;
    }

}
fn f_a_rect(
    a_n_u8__image: &mut Vec<u8>,
    n_vector_pixels_x : u32,
    n_vector_pixels_y : u32,
    n_channels: u32,
    n_position_x : u32,
    n_position_y : u32,
    n_rect_size_x : u32,
    n_rect_size_y : u32,
){

    let mut a_vec: Vec<u8> = Vec::new();

    let mut n_rgba_value = 0;
    let mut n_pixel_index = 0;
    let mut n_x = 0;
    let mut n_y = 0;
    let mut n_channel = 0;
    
    let n_index_max = a_n_u8_pixel.len()-1;

    while(n_y < n_rect_size_y){
        n_x = 0;
        while(n_x < n_rect_size_x){
            let n_index_pixel = 
            ((n_position_y + n_y) * n_vector_pixels_x * n_channels) +
            ((n_position_x + n_x) * n_channels);
            
            n_channel = 0;
            while(n_channel < n_channels){
                let n_index = (n_index_pixel+n_channel) as usize;
                if(n_index < 0 || n_index > n_index_max){
                    n_channel+=1;
                    continue;
                }
                a_vec.push(a_n_u8__image[n_index]);
                n_channel+=1;
            }
            // println!("x|y {:?}|{:?}", n_x, n_y);
            n_x+=1;
        }
        n_y+=1;
        // println!("x|y {:?}|{:?}", n_x, n_y);
    }

}

fn f_draw_rect(
    a_n_u8_pixel: &mut Vec<u8>,
    n_vector_pixels_x : u32,
    n_vector_pixels_y : u32,
    n_channels: u32,
    n_position_x : u32,
    n_position_y : u32,
    n_rect_size_x : u32,
    n_rect_size_y : u32,
    a_color : &Vec<u8>
){
    
    let mut n_rgba_value = 0;
    let mut n_pixel_index = 0;
    let mut n_x = 0;
    let mut n_y = 0;
    let mut n_channel = 0;
    
    let n_index_max = a_n_u8_pixel.len()-1;

    while(n_y < n_rect_size_y){
        n_x = 0;
        while(n_x < n_rect_size_x){
            let n_index_pixel = 
            ((n_position_y + n_y) * n_vector_pixels_x * n_channels) +
            ((n_position_x + n_x) * n_channels);
            
            n_channel = 0;
            while(n_channel < n_channels){
                let n_index = (n_index_pixel+n_channel) as usize;
                if(n_index < 0 || n_index > n_index_max){
                    n_channel+=1;
                    continue;
                }
                // println!("n_pixel_index :{:?}", n_pixel_index);
                a_n_u8_pixel[n_index] = a_color[n_channel as usize];
                n_channel+=1;
            }
            // println!("x|y {:?}|{:?}", n_x, n_y);

            n_x+=1;
        }
        n_y+=1;
        // println!("x|y {:?}|{:?}", n_x, n_y);
    }
    // for n_val in a_n_u8_pixel.iter_mut(){
    //     n_rgba_value+=1;
    //     n_pixel_index += ((n_rgba_value%n_channels == 0) * 1);
    //     n_x = n_pixel_index % n_vector_pixels_x;
    //     n_y = (n_pixel_index as f64 / n_vector_pixels_x as f64) as u32
    //     if(n_rgba_value%n_channels == 0){

    //         if(
    //             n_x > n_position_x
    //             &&
    //             n_x < n_position_x + n_pixels_x
    //             &&
    //             n_y > n_position_y
    //             &&
    //             n_y < n_position_y + n_pixels_y
    //         ){
    //             let n_i2 = 0; 
    //             while(n_i2 < n_channels){

    //                 n_i2+=1;
    //             }
    //         }
    //     }
    // }

}

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    f_generate_labyrinth();

    // let n_vector_pixels_x: u32 = 1000; 
    // let n_vector_pixels_y: u32 = 1000;
    // let n_screen_rect_size_x = 1920;
    // let n_screen_rect_size_y = 1080;

    // let n_channels: u32 = 4;
    // let mut a_n_u8_pixel = vec![0; (n_vector_pixels_x * n_vector_pixels_y * n_channels).try_into().unwrap()];
    // // let a_n_u8_pixel : [u8; n_pixels_x * n_pixels_y] = [222];

    // // f_animate_using_autopilot(
    // //     &mut a_n_u8_pixel, 
    // //     n_vector_pixels_x,
    // //     n_vector_pixels_y,
    // //     n_channels, 
    // //     n_screen_rect_size_x,
    // //     n_screen_rect_size_y
    // // );


    // f_animate_using_window_event(
    //     &mut a_n_u8_pixel, 
    //     n_vector_pixels_x,
    //     n_vector_pixels_y,
    //     n_channels, 
    //     n_screen_rect_size_x,
    //     n_screen_rect_size_y
    // );

  Ok(())
}

fn f_animate_using_autopilot(

    a_n_u8_pixel: &mut Vec<u8>,
    n_vector_pixels_x : u32,
    n_vector_pixels_y : u32,
    n_channels: u32,
    n_screen_rect_size_x: u32,
    n_screen_rect_size_y: u32,
)  {
    let mut a_color = vec![255,255,255, 1];


    let mut image = ImageView::new(ImageInfo::rgba8(n_vector_pixels_x, n_vector_pixels_y), &a_n_u8_pixel); 
    let window = create_window("image", Default::default()).unwrap();
    window.set_image("image-001", image).unwrap();

    let mut n_i = 0;
    loop{
        n_i+=1;
        // println!(" ggez::input::mouse::button_pressed {:?}",  ggez::input::mouse::button_pressed());
        // println!("curosorpos {:?}", mouse::location());
        // println!("Left button pressed? {:?}", Mouse::Left.is_pressed());
        let o_mouse_loc = mouse::location();
        let mut n_index = 0; 

        let n_mouse_x_normalized = ((o_mouse_loc.x) as f32 /n_screen_rect_size_x as f32); 
        let n_mouse_y_normalized = ((o_mouse_loc.y) as f32 /n_screen_rect_size_y as f32);

        let n_pixel_pos_x = (n_mouse_x_normalized * n_screen_rect_size_x as f32) as u32;
        let n_pixel_pos_y = (n_mouse_y_normalized * n_screen_rect_size_y as f32) as u32;
        // println!("n_pixel_pos_x: {:?}", n_pixel_pos_x);

        // f_draw_rect(
        f_draw_circle(
            a_n_u8_pixel,
            n_vector_pixels_x, 
            n_vector_pixels_y, 
            n_channels,
            n_pixel_pos_x, 
            n_pixel_pos_y, 
            50, 
            50, 
            &a_color, 
        );


        image = ImageView::new(ImageInfo::rgba8(n_vector_pixels_x, n_vector_pixels_y), &a_n_u8_pixel);
        window.set_image("image-001", image);
        // let cursor_location: (i32, i32) = Enigo::mouse_location();
        // !println("curosorpos {:?}", cursor_location);
    }

}



fn f_animate_using_window_event(
    a_n_u8_pixel: &mut Vec<u8>,
    n_vector_pixels_x : u32,
    n_vector_pixels_y : u32,
    n_channels: u32,
    n_screen_rect_size_x: u32,
    n_screen_rect_size_y: u32,
) {
    let mut o_rand_thread_rng = rand::thread_rng();
    let mut a_color = vec![255,0,11, 122];
    let mut image = ImageView::new(ImageInfo::rgba8(n_vector_pixels_x, n_vector_pixels_y), &a_n_u8_pixel); 
    let o_window = create_window("image", Default::default()).unwrap();
    // window.set_image("image-001", image).unwrap();
    // println!("window {:?}", window.a_inner_size());
    
    let mut a_inner_size = o_window.run_function_wait(|o_window| o_window.inner_size()).unwrap();
    let mut n_window_size_x = a_inner_size[0];
    let mut n_window_size_y = a_inner_size[1];
   
    let mut b_mouse_down = false;

    let n_char_min = 65; 
    let n_char_max = 126; 
    let mut n_char = 65;
    // let mut s_a = [65];
    // let mut s_char = String::from_utf8_lossy(&s_a);

    let o_inst_now = Instant::now();

    let mut n_ts_mcs_now = o_inst_now.elapsed().as_micros();
    let mut n_ts_mcs_last = o_inst_now.elapsed().as_micros();
    let mut n_ts_mcs_delta = o_inst_now.elapsed().as_micros();

    let mut n_ts_ms_now = o_inst_now.elapsed().as_millis();
    let mut n_ts_ms_last = o_inst_now.elapsed().as_millis();
    let mut n_ts_ms_delta = o_inst_now.elapsed().as_millis();

    let n_fps = 60; 
    let n_milliseconds_per_frame = ((1000.0)/n_fps as f32) as u128;
    let n_microseconds_pre_frame = ((1000.0*1000.0)/n_fps as f32) as u128;

    let mut b_frame_is_being_calculated = false;


    for event in o_window.event_channel().unwrap() {
        if let event::WindowEvent::MouseMove(event) = event.clone() {
           
            // println!("{:#?}", event);
            n_ts_mcs_now = o_inst_now.elapsed().as_micros();
            n_ts_ms_now = o_inst_now.elapsed().as_millis();
            n_ts_mcs_delta = n_ts_mcs_now - n_ts_mcs_last;
            n_ts_ms_delta = n_ts_ms_now - n_ts_ms_last;
            // println!("n_ts_mcs_now {:?}", n_ts_mcs_now);
            // println!("n_ts_mcs_last {:?}", n_ts_mcs_last);
            // println!("n_ts_mcs_delta {:?}", n_ts_mcs_delta);
            // println!("n_ts_ms_delta {:?}", n_ts_ms_delta);
            // if(n_ts_mcs_delta > (n_microseconds_pre_frame as u128)){
            if(n_ts_ms_delta > (n_milliseconds_per_frame as u128)){
                if(b_mouse_down){

                        let n_mouse_x_normalized = ((event.position[0]) as f32 /n_window_size_x as f32); 
                        let n_mouse_y_normalized = ((event.position[1]) as f32 /n_window_size_y as f32);
                        
                        let n_pixel_pos_x = (n_mouse_x_normalized * n_screen_rect_size_x as f32) as u32;
                        let n_pixel_pos_y = (n_mouse_y_normalized * n_screen_rect_size_y as f32) as u32;
                        // println!("n_pixel_pos_x n_pixel_pos_y {:?} {:?}", n_pixel_pos_x,n_pixel_pos_y);
                        // println!("n_pixel_pos_x: {:?}", n_pixel_pos_x);
            
                        // let now = Instant::now();
                        // we sleep for 2 seconds
                        // sleep(Duration::new(2, 0));
                        // it prints '2'
                        // println!("{}", now.elapsed().as_secs());

                        a_color = vec![
                            255 as u8,
                            255 as u8,
                            255 as u8,
                            // (o_rand_thread_rng.gen::<u8>()),
                            // (o_rand_thread_rng.gen::<u8>()),
                            // (o_rand_thread_rng.gen::<u8>()),
                            // (o_rand_thread_rng.gen::<u8>()),
                            122 as u8
                        ];
                        // let o_inst_now = Instant::now();
                        // println!("{}", now.elapsed().as_secs());
                        // f_draw_char(
                        //     a_n_u8_pixel,
                        //     n_vector_pixels_x, 
                        //     n_vector_pixels_y, 
                        //     n_channels,
                        //     n_pixel_pos_x, 
                        //     n_pixel_pos_y, 
                        //     5.0,
                        //     &a_color, 
                        //     n_char
                        // );

                        // f_draw_rect(
                        //     a_n_u8_pixel,
                        //     n_vector_pixels_x, 
                        //     n_vector_pixels_y,
                        //     n_channels,
                        //     n_pixel_pos_x+10, 
                        //     n_pixel_pos_y+10, 
                        //     10, 
                        //     10,
                        //     &a_color, 
                        // );
                        // f_draw_circle(
                        //     a_n_u8_pixel,
                        //     n_vector_pixels_x, 
                        //     n_vector_pixels_y,
                        //     n_channels,
                        //     n_pixel_pos_x+20, 
                        //     n_pixel_pos_y+20, 
                        //     200, 
                        //     200,
                        //     &a_color,
                        //     );

                        // clear 
                        a_n_u8_pixel.fill(0);

                        f_draw_string(
                            a_n_u8_pixel,
                            n_vector_pixels_x, 
                            n_vector_pixels_y, 
                            n_channels,
                            n_pixel_pos_x, 
                            n_pixel_pos_y, 
                            2.0,
                            &a_color, 
                            String::from("hello")
                        );

                        f_draw_string(
                            a_n_u8_pixel,
                            n_vector_pixels_x, 
                            n_vector_pixels_y, 
                            n_channels,
                            10, 
                            10, 
                            2.0,
                            &a_color,
                            format!("ms: {}", n_ts_ms_delta)
                        );

                        // println!("s {:?}", n_ts_ms_delta.to_string());
                        // f_draw_string(
                        //     a_n_u8_pixel,
                        //     n_vector_pixels_x, 
                        //     n_vector_pixels_y, 
                        //     n_channels,
                        //     20, 
                        //     20, 
                        //     2.0,
                        //     &a_color, 
                        //     // String::from("a")
                        //     // (n_ts_ms_delta as u32).to_string()
                        //     String::from("ms:")
                        //     // String::from(format!("ms:{}", n_ts_ms_delta))
                        // );

                        n_ts_ms_now = o_inst_now.elapsed().as_millis();
                        n_ts_ms_delta = n_ts_ms_now - n_ts_ms_last;

                        if(n_ts_ms_delta> n_milliseconds_per_frame){
                            image = ImageView::new(ImageInfo::rgba8(n_vector_pixels_x, n_vector_pixels_y), a_n_u8_pixel);
                            o_window.set_image("image-001", image).unwrap();
                        }

                        n_ts_mcs_last = n_ts_mcs_now;
                        n_ts_ms_last = n_ts_ms_now;


                }

            }
        }
        if let event::WindowEvent::KeyboardInput(event) = event.clone() {


            // println!("{:#?}", event);
            if event.input.key_code == Some(event::VirtualKeyCode::Escape) && event.input.state.is_pressed() {
                break;
            }
            if(event.input.key_code.unwrap() == event::VirtualKeyCode::Space){
                a_n_u8_pixel.fill(0);

                n_char = (n_char + 1) % n_char_max;
            }
            image = ImageView::new(ImageInfo::rgba8(n_vector_pixels_x, n_vector_pixels_y), a_n_u8_pixel);
            o_window.set_image("image-001", image).unwrap();
        }

        if let event::WindowEvent::MouseButton(event) = event.clone() {
            // println!("bt {:?}", event);
            if(event.state == show_image::event::ElementState::Pressed){
                b_mouse_down = true;
            }else{
                b_mouse_down = false;
            }
        }
    

      
        // loop{
        //     image = ImageView::new(ImageInfo::rgba8(n_vector_pixels_x, n_vector_pixels_y), &a_n_u8_pixel);
        //     o_window.set_image("image-001", image);
        // }


  }




}


fn f_generate_labyrinth(

){
    let n_screen_rect_size_x = 1920;
    let n_screen_rect_size_y = 1080;

    // println!("o{:?}", o_object_2d.s_name);

    // The decoder is a build for reader and can be used to set various decoding options
    // via `Transformations`. The default output transformation is `Transformations::IDENTITY`.
    let decoder = png::Decoder::new(File::open("./labyrinth.png").unwrap());
    let mut reader = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; reader.output_buffer_size()];
    // Read the next frame. An APNG might contain multiple frames.
    let info = reader.next_frame(&mut buf).unwrap();
    // Grab the bytes of the image.
    let bytes = &buf[..info.buffer_size()];
    // Inspect more details of the last read frame.
    // let in_animation = reader.info().frame_control.is_some();

    let n_image_pixels_x = info.width; 
    let n_image_pixels_y = info.height; 

    let n_boxes_x = 32; 
    let n_boxes_y = 20;
    let n_scale_x = (n_image_pixels_x as f32/ n_boxes_x as f32) as u32;
    let n_scale_y = (n_image_pixels_y as f32/ n_boxes_y as f32) as u32;
    
    let mut o_object_2d_scanner_box = O_object_2d{
        s_name: String::from("scanner_box"),
        o_translation: O_spatialproperty{
            o_point_2d_current: O_point_2d{n_x: 0, n_y: 0}, 
            o_point_2d_velocity: O_point_2d{n_x: 0, n_y: 0}, 
            o_point_2d_acceleration: O_point_2d{n_x: 0, n_y: 0},
        },
        o_rotation: O_spatialproperty{
            o_point_2d_current: O_point_2d{n_x: 0, n_y: 0}, 
            o_point_2d_velocity: O_point_2d{n_x: 0, n_y: 0}, 
            o_point_2d_acceleration: O_point_2d{n_x: 0, n_y: 0},
        },
        o_scale: O_spatialproperty{
            o_point_2d_current: O_point_2d{n_x: n_scale_x, n_y: n_scale_y}, 
            o_point_2d_velocity: O_point_2d{n_x: 0, n_y: 0}, 
            o_point_2d_acceleration: O_point_2d{n_x: 0, n_y: 0},
        },
    };

    let n_channels: u32 = 4;
    // let mut a_n_u8__image = vec![0; (n_image_pixels_x * n_image_pixels_y * n_channels).try_into().unwrap()];
    // let a_n_u8_pixel : [u8; n_pixels_x * n_pixels_y] = [222];
    let mut a_n_u8__image: Vec<u8> = bytes.iter().cloned().collect();


    println!("a : {:?}", info);


    let mut a_n_u8__rgba_color_red = vec![255,0,0,255];
    //      a_
    //      ^ array
    //      ..n_u8__
    //        ^number as item
    //      ......__rgba_color_red
    //              ^ descriptive name of the array

    let mut o_image = ImageView::new(ImageInfo::rgba8(n_image_pixels_x, n_image_pixels_y), &a_n_u8__image); 
    let o_window = create_window("image", Default::default()).unwrap();
    o_window.set_image("image-001", o_image).unwrap();
    
    let mut a_inner_size = o_window.run_function_wait(|o_window| o_window.inner_size()).unwrap();
    let mut n_window_size_x = a_inner_size[0];
    let mut n_window_size_y = a_inner_size[1];
   
    let mut b_mouse_down = false;


    let o_inst_now = Instant::now();
    let mut n_ts_ms_now = o_inst_now.elapsed().as_millis();
    let mut n_ts_ms_last = o_inst_now.elapsed().as_millis();
    let mut n_ts_ms_delta = o_inst_now.elapsed().as_millis();

    let n_fps = 60; 
    let n_milliseconds_per_frame = ((1000.0)/n_fps as f32) as u128;
    let n_microseconds_per_frame = ((1000.0*1000.0)/n_fps as f32) as u128;

    let n_i = 0; 
    while(n_i < n_boxes_x*n_boxes_y){
        let a_n_u8__subframe = f_a_rect(
            &a_n_u8__image,
            o_object_2d_scanner_box.o_translation
            n_vector_pixels_x : u32,
            n_vector_pixels_y : u32,
            n_channels: u32,
            n_position_x : u32,
            n_position_y : u32,
            n_rect_size_x : u32,
            n_rect_size_y : u32,
        )
        
        
        n_i+=1;
    }
    let mut n_frame_id = 0;
    while(n_frame_id < 10000){
        n_frame_id +=1;
        println!("o_object_2d_scanner_box.o_translation.o_point_2d_current.n_y: {:?}",o_object_2d_scanner_box.o_translation.o_point_2d_current.n_y);

        a_n_u8__image = bytes.iter().cloned().collect();

        let o_mouse_loc = mouse::location();

        let n_mouse_x_normalized = ((o_mouse_loc.x) as f32 /n_screen_rect_size_x as f32); 
        let n_mouse_y_normalized = ((o_mouse_loc.y) as f32 /n_screen_rect_size_y as f32);
          
        let n_pixel_pos_x = (n_mouse_x_normalized * n_screen_rect_size_x as f32) as u32;
        let n_pixel_pos_y = (n_mouse_y_normalized * n_screen_rect_size_y as f32) as u32;

        o_object_2d_scanner_box.o_translation.o_point_2d_current.n_x = n_pixel_pos_x;
        o_object_2d_scanner_box.o_translation.o_point_2d_current.n_y = n_pixel_pos_y;

        println!("o_object_2d_scanner_box.o_translation.o_point_2d_current.n_y: {:?}",o_object_2d_scanner_box.o_translation.o_point_2d_current.n_y);
        f_draw_rect(
            &mut a_n_u8__image,
            n_image_pixels_x, 
            n_image_pixels_y, 
            n_channels,
            o_object_2d_scanner_box.o_translation.o_point_2d_current.n_x,
            o_object_2d_scanner_box.o_translation.o_point_2d_current.n_y,
            o_object_2d_scanner_box.o_scale.o_point_2d_current.n_x,
            o_object_2d_scanner_box.o_scale.o_point_2d_current.n_y,
            &a_n_u8__rgba_color_red,
        );
        o_image = ImageView::new(ImageInfo::rgba8(n_image_pixels_x, n_image_pixels_y), &a_n_u8__image);
        o_window.set_image("image-001", o_image).unwrap();
    }
    // for event in o_window.event_channel().unwrap() {
    //     if let event::WindowEvent::MouseMove(event) = event.clone() {
           

    //         n_ts_ms_now = o_inst_now.elapsed().as_millis();
    //         n_ts_ms_delta = n_ts_ms_now - n_ts_ms_last;

    //         if(n_ts_ms_delta > (n_milliseconds_per_frame as u128)){
    //                 // println!("render now!");
                    
    //                 let n_mouse_x_normalized = ((event.position[0]) as f32 /n_window_size_x as f32); 
    //                 let n_mouse_y_normalized = ((event.position[1]) as f32 /n_window_size_y as f32);
                    
    //                 let n_pixel_pos_x = (n_mouse_x_normalized * n_screen_rect_size_x as f32) as u32;
    //                 let n_pixel_pos_y = (n_mouse_y_normalized * n_screen_rect_size_y as f32) as u32;

    //                 o_object_2d_scanner_box.o_translation.o_point_2d_current.n_x = n_pixel_pos_x;
    //                 o_object_2d_scanner_box.o_translation.o_point_2d_current.n_y = n_pixel_pos_y;

    //                 n_ts_ms_now = o_inst_now.elapsed().as_millis();
    //                 n_ts_ms_delta = n_ts_ms_now - n_ts_ms_last;
                    
    //                 n_ts_ms_last = n_ts_ms_now;


    //         }
    //     }
    //     if let event::WindowEvent::KeyboardInput(event) = event.clone() {


    //         // println!("{:#?}", event);
    //         if event.input.key_code == Some(event::VirtualKeyCode::Escape) && event.input.state.is_pressed() {
    //             break;
    //         }
    //         if(event.input.key_code.unwrap() == event::VirtualKeyCode::Space){
    //             a_n_u8__image.fill(0);
    //         }
    //         o_image = ImageView::new(ImageInfo::rgba8(n_image_pixels_x, n_image_pixels_y), &a_n_u8__image);
    //         o_window.set_image("image-001", o_image).unwrap();
    //     }

    //     if let event::WindowEvent::MouseButton(event) = event.clone() {
    //         // println!("bt {:?}", event);
    //         if(event.state == show_image::event::ElementState::Pressed){
    //             b_mouse_down = true;
    //         }else{
    //             b_mouse_down = false;
    //         }
    //     }
    // }
}
