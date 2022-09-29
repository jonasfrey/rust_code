

// For reading and opening files
mod test_module;
mod constants_chars_a_u8;

use std::str;
use std::path::Path;
use std::fs::File;

use std::time::{SystemTime, UNIX_EPOCH, Instant, Duration};
use std::f64::consts::TAU;



use std::io::BufWriter;
use autopilot::mouse;
// use buttons::Mouse;
// use readmouse::Mouse;
// use ggez::input::mouse;


use show_image::{event, ImageView, ImageInfo, create_window};

fn f_draw_letter(
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
    let n_size_x = constants_chars_a_u8::a_n_size_x[n_char_index as usize] as f32;
    let n_size_y = constants_chars_a_u8::a_n_size_y[n_char_index as usize] as f32;
    let n_channels_char = constants_chars_a_u8::n_channels as u32;
    let n_char_start_index = constants_chars_a_u8::a_n_start_index_chars_information[n_char_index as usize] as usize;
    
    let mut n_x = 0 as f32; 
    let mut n_y = 0 as f32; 
    let mut n_x_normalized = 0 as f32;
    let mut n_y_normalized = 0 as f32;

    let mut n_x_char = 0 as f32;
    let mut n_y_char = 0 as f32;

    let mut n_channel = 0;
    let n_char_target_channel_index = 1;
    while(n_x < (n_size_x * n_font_size_scale)){
        n_x_normalized = n_x / (n_size_x * n_font_size_scale);
        n_x_char = n_x_normalized * n_size_x;
        n_y = 0.0;
        while(n_y < (n_size_y * n_font_size_scale)){
            n_y_normalized = n_y / (n_size_y * n_font_size_scale);
            n_y_char = n_y_normalized * n_size_y;

            n_channel = 0;
            while(n_channel < n_channels){
                // println!("n_pixel_index :{:?}", n_pixel_index);
                let n_index_pixel = 
                ((((n_position_y as i32) + (n_y as i32)) as u32) * n_vector_pixels_x * n_channels) +
                ((((n_position_x as i32) + (n_x as i32)) as u32) * n_channels);
                let n_index_pixel_char = 
                n_char_start_index as u32+ 
                ((n_y_char as u32) * (n_size_x as u32) * n_channels_char) +
                ((n_x_char as u32) * n_channels);
                // println!("x|y char {:?} {:?}", n_x_char, n_y_char);
                // let n_char_rgb_value = constants_chars_a_u8::a_chars_information[(n_index_pixel_char+n_channel) as usize];
                let n_char_rgb_value = constants_chars_a_u8::a_chars_information[(n_index_pixel_char+n_char_target_channel_index) as usize]; // ommiting the +n_channel rgb to achieve monochrome
                let mut n_val = 0;
                if(n_char_rgb_value > 0){
                    n_val = 255;
                }
                a_n_u8_pixel[(n_index_pixel+n_channel) as usize] = n_val; 
                n_channel+=1;
            }

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
            while(n_channel < n_channels){
                a_n_u8_pixel[(n_index_pixel+n_channel) as usize] = a_color[n_channel as usize];
                n_channel+=1;
            }

            n_i_step += 1;
        }

        n_radius+=1;
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

    while(n_y < n_rect_size_y){
        n_x = 0;
        while(n_x < n_rect_size_x){
            let n_index_pixel = 
            ((n_position_y + n_y) * n_vector_pixels_x * n_channels) +
            ((n_position_x + n_x) * n_channels);
            
            n_channel = 0;
            while(n_channel < n_channels){
                // println!("n_pixel_index :{:?}", n_pixel_index);
                a_n_u8_pixel[(n_index_pixel+n_channel) as usize] = a_color[n_channel as usize];
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

    let n_vector_pixels_x: u32 = 1920; 
    let n_vector_pixels_y: u32 = 1080;
    let n_screen_rect_size_x = 1920;
    let n_screen_rect_size_y = 1080;

    let n_channels: u32 = 3;
    let mut a_n_u8_pixel = vec![222; (n_vector_pixels_x * n_vector_pixels_y * n_channels).try_into().unwrap()];
    // let a_n_u8_pixel : [u8; n_pixels_x * n_pixels_y] = [222];

    // f_animate_using_autopilot(
    //     &mut a_n_u8_pixel, 
    //     n_vector_pixels_x,
    //     n_vector_pixels_y,
    //     n_channels, 
    //     n_screen_rect_size_x,
    //     n_screen_rect_size_y
    // );


    f_animate_using_window_event(
        &mut a_n_u8_pixel, 
        n_vector_pixels_x,
        n_vector_pixels_y,
        n_channels, 
        n_screen_rect_size_x,
        n_screen_rect_size_y
    );

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
    let mut a_color = vec![255,0,11];


    let mut image = ImageView::new(ImageInfo::rgb8(n_vector_pixels_x, n_vector_pixels_y), &a_n_u8_pixel); 
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


        image = ImageView::new(ImageInfo::rgb8(n_vector_pixels_x, n_vector_pixels_y), &a_n_u8_pixel);
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
    let mut a_color = vec![255,0,11];
    let mut image = ImageView::new(ImageInfo::rgb8(n_vector_pixels_x, n_vector_pixels_y), &a_n_u8_pixel); 
    let window = create_window("image", Default::default()).unwrap();
    window.set_image("image-001", image).unwrap();
    
    let n_char_min = 65; 
    let n_char_max = 126; 
    let mut n_char = 65;
    // let mut s_a = [65];
    // let mut s_char = String::from_utf8_lossy(&s_a);

    let o_inst_now = Instant::now();

    let mut n_ts_mcs_now = o_inst_now.elapsed().as_micros();
    let mut n_ts_mcs_last = o_inst_now.elapsed().as_micros();
    let mut n_ts_mcs_delta = o_inst_now.elapsed().as_micros();

    let n_fps = 60; 
    let n_microseconds_pre_frame = ((1000.0*1000.0)/n_fps as f32) as f32;

    for event in window.event_channel().unwrap() {
        if let event::WindowEvent::MouseMove(event) = event.clone() {
            // println!("{:#?}", event.position);
            n_ts_mcs_now = o_inst_now.elapsed().as_micros();
            n_ts_mcs_delta = n_ts_mcs_now - n_ts_mcs_last; 
            // println!("n_ts_mcs_now {:?}", n_ts_mcs_now);
            // println!("n_ts_mcs_last {:?}", n_ts_mcs_last);
            // println!("n_ts_mcs_delta {:?}", n_ts_mcs_delta);
            if(n_ts_mcs_delta > (n_microseconds_pre_frame as u128)){
                let n_mouse_x_normalized = ((event.position[0]) as f32 /n_screen_rect_size_x as f32); 
                let n_mouse_y_normalized = ((event.position[1]) as f32 /n_screen_rect_size_y as f32);
                
                let n_pixel_pos_x = (n_mouse_x_normalized * n_screen_rect_size_x as f32) as u32;
                let n_pixel_pos_y = (n_mouse_y_normalized * n_screen_rect_size_y as f32) as u32;
                // println!("n_pixel_pos_x n_pixel_pos_y {:?} {:?}", n_pixel_pos_x,n_pixel_pos_y);
                // println!("n_pixel_pos_x: {:?}", n_pixel_pos_x);
    
                // let now = Instant::now();
                // we sleep for 2 seconds
                // sleep(Duration::new(2, 0));
                // it prints '2'
                // println!("{}", now.elapsed().as_secs());


                // let o_inst_now = Instant::now();
                // println!("{}", now.elapsed().as_secs());
                f_draw_letter(
                    a_n_u8_pixel,
                    n_vector_pixels_x, 
                    n_vector_pixels_y, 
                    n_channels,
                    n_pixel_pos_x, 
                    n_pixel_pos_y, 
                    5.0,
                    &a_color, 
                    n_char
                );
    
                image = ImageView::new(ImageInfo::rgb8(n_vector_pixels_x, n_vector_pixels_y), a_n_u8_pixel);
                window.set_image("image-001", image).unwrap();

                n_ts_mcs_last = n_ts_mcs_now
            }
        }
    if let event::WindowEvent::KeyboardInput(event) = event.clone() {
            println!("{:#?}", event);
            if event.input.key_code == Some(event::VirtualKeyCode::Escape) && event.input.state.is_pressed() {
                break;
            }
            if(event.input.key_code.unwrap() == event::VirtualKeyCode::Space){
                a_n_u8_pixel.fill(0);

                n_char = (n_char + 1) % n_char_max;
            }
            image = ImageView::new(ImageInfo::rgb8(n_vector_pixels_x, n_vector_pixels_y), a_n_u8_pixel);
            window.set_image("image-001", image).unwrap();
        }

      }



}