#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

use device_query::{DeviceQuery, DeviceState, MouseState, Keycode};
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
    n_x:f64,
    n_y:f64,
}
#[derive(Debug)]
struct O_spatialproperty{

    o_point_2d__current: O_point_2d, 
    o_point_2d__velocity: O_point_2d, 
    o_point_2d__acceleration: O_point_2d,

 }

 fn f_calculate_o_spatialproperty(
    o_spatialproperty: &mut O_spatialproperty
 ){
    o_spatialproperty.o_point_2d__velocity.n_x += o_spatialproperty.o_point_2d__acceleration.n_x;
    o_spatialproperty.o_point_2d__velocity.n_y += o_spatialproperty.o_point_2d__acceleration.n_y;

    o_spatialproperty.o_point_2d__current.n_x += o_spatialproperty.o_point_2d__velocity.n_x;
    o_spatialproperty.o_point_2d__current.n_y += o_spatialproperty.o_point_2d__velocity.n_y;
 }
 #[derive(Debug)]
 struct O_object_2d{
    s_name: String,
    o_spatialproperty__translation: O_spatialproperty,
    o_spatialproperty__rotation: O_spatialproperty,
    o_spatialproperty__scale: O_spatialproperty,
    f_a_n_f64__color: fn(u32, u32) -> Vec<f64>
 }

fn f_calculate_o_object_2d(
    o_object_2d: &mut O_object_2d
){

    f_calculate_o_spatialproperty(&mut o_object_2d.o_spatialproperty__translation);
    f_calculate_o_spatialproperty(&mut o_object_2d.o_spatialproperty__rotation);
    f_calculate_o_spatialproperty(&mut o_object_2d.o_spatialproperty__scale);
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

fn f_b_collision_with_o_object_2d_by_s_name(
    o_game: &O_game, 
    n_index_o_object_2d: usize,
    o_object_2d_s_name: String
)->bool{
    let mut b_collision = false;
    for obj_object_2d in o_game.a_o_object_2d.iter(){
        if(obj_object_2d.s_name == o_object_2d_s_name){
            // println!("o_object_2d.o_spatialproperty__translation.o_point_2d__current.n_y: {:?}", o_object_2d.o_spatialproperty__translation.o_point_2d__current.n_y);
            // println!("n_y: {:?}", n_y);
            // println!("o_object_2d.o_spatialproperty__translation.o_point_2d__current.n_x: {:?}", o_object_2d.o_spatialproperty__translation.o_point_2d__current.n_x);
            // println!("n_x: {:?}", n_x);
            if(
                obj_object_2d.o_spatialproperty__translation.o_point_2d__current.n_y == o_game.a_o_object_2d[n_index_o_object_2d].o_spatialproperty__translation.o_point_2d__current.n_y
                &&
                obj_object_2d.o_spatialproperty__translation.o_point_2d__current.n_x == o_game.a_o_object_2d[n_index_o_object_2d].o_spatialproperty__translation.o_point_2d__current.n_x
            ){
                b_collision = true;
            }
        }
    }
    return b_collision;
}

fn f_a_rect_read_and_optional_write(
    a_n_u8__image: &mut Vec<u8>,
    n_image_scale_x : u32,
    n_image_scale_y : u32,
    n_image_channels: u32,
    n_rect_translation_x : u32,
    n_rect_translation_y : u32,
    n_rect_scale_x : u32,
    n_rect_scale_y : u32,
    // a_n_f64__color : Option<&Vec<f64>>
    f_a_n_f64__color: Option<fn(u32, u32) -> Vec<f64>>
) -> Vec<u8> {

    let mut a_vec: Vec<u8> = vec![
        0;
        n_image_channels as usize *
        n_rect_scale_x as usize *
        n_rect_scale_y as usize 
    ];

    let mut n_rgba_value = 0;
    let mut n_pixel_index = 0;
    let mut n_x = 0;
    let mut n_y = 0;
    let mut n_channel = 0;
    
    let n_index_max = a_n_u8__image.len()-1;

    let mut n_index_a_vec = 0;
    let mut a_n_f64__color :Vec<f64>= vec![0.0,0.0,0.0,0.0];
    while(n_y < n_rect_scale_y){
        n_x = 0;
        while(n_x < n_rect_scale_x){
            let n_index_pixel = 
            ((n_rect_translation_y + n_y) * n_image_scale_x * n_image_channels) +
            ((n_rect_translation_x + n_x) * n_image_channels);
            
            n_channel = 0;
            
            if(f_a_n_f64__color.is_none() == false){
                a_n_f64__color = f_a_n_f64__color.unwrap()(n_x, n_y);
            }
            while(n_channel < n_image_channels){
                let n_index = (n_index_pixel+n_channel);
                if(n_index < 0 || n_index > n_index_max.try_into().unwrap()){
                    n_channel+=1;
                    continue;
                }
                if(
                    f_a_n_f64__color.is_none() == false
                ){
                    // println!("a_n {:?}", (a_n_f64__color[n_channel as usize] * (u8::MAX as f64)) as u8);
                    a_n_u8__image[n_index as usize] =(a_n_f64__color[n_channel as usize] * (u8::MAX as f64)) as u8;
                    // a_n_u8__image[n_index as usize] =255;
                }
                a_vec[n_index_a_vec as usize] = (a_n_u8__image[n_index as usize]);
                n_index_a_vec +=1;
                n_channel+=1;
            }
            // println!("x|y {:?}|{:?}", n_x, n_y);
            n_x+=1;
        }
        n_y+=1;
        // println!("x|y {:?}|{:?}", n_x, n_y);
    }

    return a_vec

}


#[derive(Debug)]
struct O_game<'a>{
    n_pixel_channels: u8, 
    o_object_2d_window: O_object_2d,
    a_o_object_2d: Vec<O_object_2d>,
    s_name: String,
    n_frame_id: u64, 
    a_n_u8__image: &'a mut Vec<u8>,
    n_ts_mic_frame: u128, 
    n_ts_mic_frame_last: u128,
    n_ts_mic_frame_delta: u128,
    n_fps_avg: f64,
    n_boxes_x: u32, 
    n_boxes_y: u32,
    o_object_2d_box: O_object_2d,
    // o_window: T,
    // Result<WindowProxy, error::CreateWindowError>,
    // o_image: T,
}

fn f_a_n_f64__color__green(n_x: u32, n_y:u32)-> Vec<f64>{
    vec![0.0,1.0,0.0,1.0]
}
fn f_a_n_f64__color__red(n_x: u32, n_y:u32)-> Vec<f64>{
    vec![1.0,0.0,0.0,1.0]
}
fn f_a_n_f64__color__random(n_x: u32, n_y:u32)-> Vec<f64>{
    let mut rng = rand::thread_rng();

    vec![
        rng.gen::<f64>(), 
        rng.gen::<f64>(),
        rng.gen::<f64>(),
        rng.gen::<f64>()
    ]
}
fn f_a_n_f64__color__yellow_random(n_x: u32, n_y:u32)-> Vec<f64>{
    let mut rng = rand::thread_rng();
    let n_rand = rng.gen::<f64>();
    vec![
        n_rand,
        n_rand,
        0.0, 
        1.0
    ]
}

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut o_object_2d_window = O_object_2d{
        s_name: String::from("window"),
        o_spatialproperty__translation: O_spatialproperty{
            o_point_2d__current: O_point_2d{n_x: 0.0, n_y: 0.0}, 
            o_point_2d__velocity: O_point_2d{n_x: 0.0, n_y: 0.0}, 
            o_point_2d__acceleration: O_point_2d{n_x: 0.0, n_y: 0.0},
        },
        o_spatialproperty__rotation: O_spatialproperty{
            o_point_2d__current: O_point_2d{n_x: 0.0, n_y: 0.0}, 
            o_point_2d__velocity: O_point_2d{n_x: 0.0, n_y: 0.0}, 
            o_point_2d__acceleration: O_point_2d{n_x: 0.0, n_y: 0.0},
        },
        o_spatialproperty__scale: O_spatialproperty{
            o_point_2d__current: O_point_2d{n_x: 1920.0, n_y: 1080.0}, 
            o_point_2d__velocity: O_point_2d{n_x: 0.0, n_y: 0.0}, 
            o_point_2d__acceleration: O_point_2d{n_x: 0.0, n_y: 0.0},
        },
        f_a_n_f64__color: f_a_n_f64__color__red
    };
    let n_pixel_channels = 4;

    let mut a_n_u8__image: Vec<u8> = 
    vec![
        0;
        (
            o_object_2d_window.o_spatialproperty__scale.o_point_2d__current.n_x *  
            o_object_2d_window.o_spatialproperty__scale.o_point_2d__current.n_y * 
            (n_pixel_channels as f64)
        ) as usize
    ];
    let a_o_object_2d : Vec<O_object_2d> = Vec::new();
    let s_name = String::from("labyrinth");
    let s_name_copye = String::from("labyrinth");
    let n_frame_id = 0;

    let o_window = create_window(s_name, Default::default()).unwrap();
    let mut o_image = ImageView::new(
        ImageInfo::rgba8(
            o_object_2d_window.o_spatialproperty__scale.o_point_2d__current.n_x as u32,  
            o_object_2d_window.o_spatialproperty__scale.o_point_2d__current.n_y as u32
        ),
        &a_n_u8__image
    ); 

    let n_boxes_x = 32;
    let n_boxes_y = 20;

    let n_scale_box_x = (o_object_2d_window.o_spatialproperty__scale.o_point_2d__current.n_x as f32/ n_boxes_x as f32) as f64;
    let n_scale_box_y = (o_object_2d_window.o_spatialproperty__scale.o_point_2d__current.n_y as f32/ n_boxes_y as f32) as f64;

    let mut o_object_2d_box = O_object_2d{
        s_name: String::from("box"),
        o_spatialproperty__translation: O_spatialproperty{
            o_point_2d__current: O_point_2d{n_x: 0.0, n_y: 0.0}, 
            o_point_2d__velocity: O_point_2d{n_x: 0.0, n_y: 0.0}, 
            o_point_2d__acceleration: O_point_2d{n_x: 0.0, n_y: 0.0},
        },
        o_spatialproperty__rotation: O_spatialproperty{
            o_point_2d__current: O_point_2d{n_x: 0.0, n_y: 0.0}, 
            o_point_2d__velocity: O_point_2d{n_x: 0.0, n_y: 0.0}, 
            o_point_2d__acceleration: O_point_2d{n_x: 0.0, n_y: 0.0},
        },
        o_spatialproperty__scale: O_spatialproperty{
            o_point_2d__current: O_point_2d{n_x: n_scale_box_x, n_y: n_scale_box_y}, 
            o_point_2d__velocity: O_point_2d{n_x: 0.0, n_y: 0.0}, 
            o_point_2d__acceleration: O_point_2d{n_x: 0.0, n_y: 0.0},
        },
        f_a_n_f64__color: f_a_n_f64__color__red
    };

    let mut o_game = O_game{
        n_pixel_channels: n_pixel_channels,
        o_object_2d_window: o_object_2d_window,
        a_n_u8__image: &mut a_n_u8__image,
        a_o_object_2d: a_o_object_2d,
        s_name: s_name_copye,
        n_frame_id: n_frame_id,
        n_ts_mic_frame: 0,
        n_ts_mic_frame_last: 0,
        n_ts_mic_frame_delta: 0,
        n_fps_avg: 0.0,
        n_boxes_x: 32, 
        n_boxes_y: 20,
        o_object_2d_box: o_object_2d_box
        // o_image: o_image,
        // o_window: o_image,
    };

   

    f_detect_labyrinth_from_image(&mut o_game);
   
    let o_object_2d_wall = &o_game.a_o_object_2d[0];

    let mut o_object_2d_player = O_object_2d{
        s_name: String::from("player"),
        o_spatialproperty__translation: O_spatialproperty{
            o_point_2d__current: O_point_2d{
                n_x : 0.0,
                n_y : 14.0 * o_game.o_object_2d_box.o_spatialproperty__scale.o_point_2d__current.n_y
            }, 
            o_point_2d__velocity: O_point_2d{
                n_x : o_object_2d_wall.o_spatialproperty__translation.o_point_2d__velocity.n_x,
                n_y : o_object_2d_wall.o_spatialproperty__translation.o_point_2d__velocity.n_y,
            }, 
            o_point_2d__acceleration: O_point_2d{
                n_x : o_object_2d_wall.o_spatialproperty__translation.o_point_2d__acceleration.n_x,
                n_y : o_object_2d_wall.o_spatialproperty__translation.o_point_2d__acceleration.n_y,
            },
        },
        o_spatialproperty__scale: O_spatialproperty{
            o_point_2d__current: O_point_2d{
                n_x : o_object_2d_wall.o_spatialproperty__scale.o_point_2d__current.n_x,
                n_y : o_object_2d_wall.o_spatialproperty__scale.o_point_2d__current.n_y
            }, 
            o_point_2d__velocity: O_point_2d{
                n_x : o_object_2d_wall.o_spatialproperty__scale.o_point_2d__velocity.n_x,
                n_y : o_object_2d_wall.o_spatialproperty__scale.o_point_2d__velocity.n_y,
            }, 
            o_point_2d__acceleration: O_point_2d{
                n_x : o_object_2d_wall.o_spatialproperty__scale.o_point_2d__acceleration.n_x,
                n_y : o_object_2d_wall.o_spatialproperty__scale.o_point_2d__acceleration.n_y,
            },
        },
        o_spatialproperty__rotation: O_spatialproperty{
            o_point_2d__current: O_point_2d{
                n_x : o_object_2d_wall.o_spatialproperty__rotation.o_point_2d__current.n_x,
                n_y : o_object_2d_wall.o_spatialproperty__rotation.o_point_2d__current.n_y
            }, 
            o_point_2d__velocity: O_point_2d{
                n_x : o_object_2d_wall.o_spatialproperty__rotation.o_point_2d__velocity.n_x,
                n_y : o_object_2d_wall.o_spatialproperty__rotation.o_point_2d__velocity.n_y,
            }, 
            o_point_2d__acceleration: O_point_2d{
                n_x : o_object_2d_wall.o_spatialproperty__rotation.o_point_2d__acceleration.n_x,
                n_y : o_object_2d_wall.o_spatialproperty__rotation.o_point_2d__acceleration.n_y,
            },
        },

        f_a_n_f64__color: f_a_n_f64__color__yellow_random
    };
    let n_index_o_object_2d_player = o_game.a_o_object_2d.len();
    o_game.a_o_object_2d.push(o_object_2d_player);

    let n_screen_rect_size_x = 1920;
    let n_screen_rect_size_y = 1080;
    
    let device_state = DeviceState::new();


    let mut a_o_point_2d__player_movements_cached : Vec<O_point_2d> = Vec::new();

    let mut a_o_keycode: Vec<Keycode> = device_state.get_keys();
    let mut a_o_keycode_last: Vec<Keycode> = device_state.get_keys();

    let o_instant_now = Instant::now();
    while(true){
        // clear

        o_game.a_n_u8__image.fill(0);
        o_game.n_frame_id +=1;
        o_game.n_ts_mic_frame = o_instant_now.elapsed().as_micros();
        o_game.n_ts_mic_frame_delta = o_game.n_ts_mic_frame - o_game.n_ts_mic_frame_last;
        // println!("delta microseconds {:?}", o_game.n_ts_mic_frame_delta);
        // println!("delta milliseconds {:?}", o_game.n_ts_mic_frame_delta / 1000);
        o_game.n_fps_avg = 1000.0 / (o_game.n_ts_mic_frame_delta / 1000) as f64;
        // println!("o_game.n_fps_avg {:?}", o_game.n_fps_avg);

        // let o_mouse_loc = mouse::location();
        // let n_mouse_x_normalized = ((o_mouse_loc.x) as f32 /n_screen_rect_size_x as f32); 
        // let n_mouse_y_normalized = ((o_mouse_loc.y) as f32 /n_screen_rect_size_y as f32);
        // let n_pixel_pos_x = (n_mouse_x_normalized * n_screen_rect_size_x as f32) as f64;
        // let n_pixel_pos_y = (n_mouse_y_normalized * n_screen_rect_size_y as f32) as f64;

        // println!("o_game.n_frame_id: {:?}", o_game.n_frame_id);
        a_o_keycode = device_state.get_keys();
        // println!("Is Right pressed? {}", keys.contains(&Keycode::Right));
        let n_x_cached = o_game.a_o_object_2d[n_index_o_object_2d_player].o_spatialproperty__translation.o_point_2d__current.n_x;
        let n_y_cached = o_game.a_o_object_2d[n_index_o_object_2d_player].o_spatialproperty__translation.o_point_2d__current.n_y;

        let mut n_x = n_x_cached; 
        let mut n_y = n_y_cached; 
        
        if(a_o_keycode.contains(&Keycode::Right) && !a_o_keycode_last.contains(&Keycode::Right)){
            n_x = o_game.a_o_object_2d[n_index_o_object_2d_player].o_spatialproperty__translation.o_point_2d__current.n_x + o_game.o_object_2d_box.o_spatialproperty__scale.o_point_2d__current.n_x; 
            n_y = o_game.a_o_object_2d[n_index_o_object_2d_player].o_spatialproperty__translation.o_point_2d__current.n_y;
        }
        if(a_o_keycode.contains(&Keycode::Left) && !a_o_keycode_last.contains(&Keycode::Left)){
            n_x = o_game.a_o_object_2d[n_index_o_object_2d_player].o_spatialproperty__translation.o_point_2d__current.n_x - o_game.o_object_2d_box.o_spatialproperty__scale.o_point_2d__current.n_x; 
            n_y = o_game.a_o_object_2d[n_index_o_object_2d_player].o_spatialproperty__translation.o_point_2d__current.n_y;
        }
        if(a_o_keycode.contains(&Keycode::Up) && !a_o_keycode_last.contains(&Keycode::Up)){
            n_y = o_game.a_o_object_2d[n_index_o_object_2d_player].o_spatialproperty__translation.o_point_2d__current.n_y - o_game.o_object_2d_box.o_spatialproperty__scale.o_point_2d__current.n_y; 
            n_x = o_game.a_o_object_2d[n_index_o_object_2d_player].o_spatialproperty__translation.o_point_2d__current.n_x;
        }
        if(a_o_keycode.contains(&Keycode::Down) && !a_o_keycode_last.contains(&Keycode::Down)){
            n_y = o_game.a_o_object_2d[n_index_o_object_2d_player].o_spatialproperty__translation.o_point_2d__current.n_y + o_game.o_object_2d_box.o_spatialproperty__scale.o_point_2d__current.n_y; 
            n_x = o_game.a_o_object_2d[n_index_o_object_2d_player].o_spatialproperty__translation.o_point_2d__current.n_x;
        }



        o_game.a_o_object_2d[n_index_o_object_2d_player].o_spatialproperty__translation.o_point_2d__current.n_y = n_y;
        o_game.a_o_object_2d[n_index_o_object_2d_player].o_spatialproperty__translation.o_point_2d__current.n_x = n_x;
        let mut b_collision = f_b_collision_with_o_object_2d_by_s_name(
            &o_game, 
            n_index_o_object_2d_player, 
            String::from("wall")
        );
        if(b_collision){
            o_game.a_o_object_2d[n_index_o_object_2d_player].o_spatialproperty__translation.o_point_2d__current.n_y = n_y_cached;
            o_game.a_o_object_2d[n_index_o_object_2d_player].o_spatialproperty__translation.o_point_2d__current.n_x = n_x_cached;
        }else{
            if(n_x != n_x_cached || n_y != n_y_cached){
                
                o_game.a_o_object_2d.push(
                    O_object_2d{
                        s_name: String::from("player_visited"),
                        o_spatialproperty__translation: O_spatialproperty{
                            o_point_2d__current: O_point_2d{
                                n_x: n_x_cached,
                                n_y: n_y_cached
                            }, 
                            o_point_2d__velocity: O_point_2d{n_x: 0.0, n_y: 0.0}, 
                            o_point_2d__acceleration: O_point_2d{n_x: 0.0, n_y: 0.0},
                        },
                        o_spatialproperty__rotation: O_spatialproperty{
                            o_point_2d__current: O_point_2d{n_x: 0.0, n_y: 0.0}, 
                            o_point_2d__velocity: O_point_2d{n_x: 0.0, n_y: 0.0}, 
                            o_point_2d__acceleration: O_point_2d{n_x: 0.0, n_y: 0.0},
                        },
                        o_spatialproperty__scale: O_spatialproperty{
                            o_point_2d__current: O_point_2d{
                                n_x: o_game.a_o_object_2d[n_index_o_object_2d_player].o_spatialproperty__scale.o_point_2d__current.n_x,
                                n_y: o_game.a_o_object_2d[n_index_o_object_2d_player].o_spatialproperty__scale.o_point_2d__current.n_y
                            }, 
                            o_point_2d__velocity: O_point_2d{n_x: 0.0, n_y: 0.0}, 
                            o_point_2d__acceleration: O_point_2d{n_x: 0.0, n_y: 0.0},
                        },
                        f_a_n_f64__color: f_a_n_f64__color__green
                    }
                );
            }
        }
        // else{
        //     a_o_point_2d__player_movements_cached.push(
        //         O_point_2d{
        //             n_x: o_game.a_o_object_2d[n_index_o_object_2d_player].o_spatialproperty__translation.o_point_2d__current.n_x,
        //             n_y: o_game.a_o_object_2d[n_index_o_object_2d_player].o_spatialproperty__translation.o_point_2d__current.n_y
        //         }
        //     );
        // }
        
        // println!("o_object_2d_scanner_box.o_spatialproperty__translation.o_point_2d__current.n_y: {:?}",o_object_2d_scanner_box.o_spatialproperty__translation.o_point_2d__current.n_y);
        for o_object_2d in o_game.a_o_object_2d.iter(){
            f_a_rect_read_and_optional_write(
                o_game.a_n_u8__image,
                o_game.o_object_2d_window.o_spatialproperty__scale.o_point_2d__current.n_x as u32,  
                o_game.o_object_2d_window.o_spatialproperty__scale.o_point_2d__current.n_y as u32,
                o_game.n_pixel_channels as u32, 
                (o_object_2d.o_spatialproperty__translation.o_point_2d__current.n_x) as u32,
                (o_object_2d.o_spatialproperty__translation.o_point_2d__current.n_y) as u32,
                (o_object_2d.o_spatialproperty__scale.o_point_2d__current.n_x) as u32,
                (o_object_2d.o_spatialproperty__scale.o_point_2d__current.n_y) as u32,
                Some(o_object_2d.f_a_n_f64__color),
            );
        }

        o_image = ImageView::new(
            ImageInfo::rgba8(
                o_game.o_object_2d_window.o_spatialproperty__scale.o_point_2d__current.n_x as u32,  
                o_game.o_object_2d_window.o_spatialproperty__scale.o_point_2d__current.n_y as u32
            ),
            o_game.a_n_u8__image
        ); 

        
        o_window.set_image(o_game.s_name.clone(), o_image).unwrap();

        a_o_keycode_last = a_o_keycode;

        o_game.n_ts_mic_frame_last = o_game.n_ts_mic_frame;

    }
    

  Ok(())
}


fn f_detect_labyrinth_from_image(
    o_game: &mut O_game
){
     // println!("o{:?}", o_object_2d.s_name);

    // The decoder is a build for reader and can be used to set various decoding options
    // via `Transformations`. The default output transformation is `Transformations::IDENTITY`.
    let decoder = png::Decoder::new(File::open("./labyrinth_contrast.png").unwrap());
    let mut reader = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; reader.output_buffer_size()];
    // Read the next frame. An APNG might contain multiple frames.
    let info = reader.next_frame(&mut buf).unwrap();
    // Grab the bytes of the image.
    let bytes = &buf[..info.buffer_size()];
    // Inspect more details of the last read frame.
    // let in_animation = reader.info().frame_control.is_some();


    let n_channels: u32 = 4;
    // let mut a_n_u8__image = vec![0; (n_image_pixels_x * n_image_pixels_y * n_channels).try_into().unwrap()];
    // let a_n_u8_pixel : [u8; n_pixels_x * n_pixels_y] = [222];
    let mut a_n_u8__image: Vec<u8> = bytes.iter().cloned().collect();


    println!("a : {:?}", info);
    let n_image_pixels_x = info.width; 
    let n_image_pixels_y = info.height; 
    
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
    
    let n_scale_x = (n_image_pixels_x as f32/ o_game.n_boxes_x as f32) as f64;
    let n_scale_y = (n_image_pixels_y as f32/ o_game.n_boxes_y as f32) as f64;

    let mut n_i = 0; 
    let mut n_x = 0; 
    let mut n_y = 0; 
    while(n_i < o_game.n_boxes_x*o_game.n_boxes_y){
        n_x = ( n_x + 1 ) % o_game.n_boxes_x;
        n_y = ((n_i as f32) / o_game.n_boxes_x as f32) as u32;

        let mut o_object_2d_wall = O_object_2d{
            s_name: String::from("wall"),
            o_spatialproperty__translation: O_spatialproperty{
                o_point_2d__current: O_point_2d{
                    n_x : n_x as f64 * n_scale_x,
                    n_y : n_y as f64 * n_scale_y
                }, 
                o_point_2d__velocity: O_point_2d{n_x: 0.0, n_y: 0.0}, 
                o_point_2d__acceleration: O_point_2d{n_x: 0.0, n_y: 0.0},
            },
            o_spatialproperty__rotation: O_spatialproperty{
                o_point_2d__current: O_point_2d{n_x: 0.0, n_y: 0.0}, 
                o_point_2d__velocity: O_point_2d{n_x: 0.0, n_y: 0.0}, 
                o_point_2d__acceleration: O_point_2d{n_x: 0.0, n_y: 0.0},
            },
            o_spatialproperty__scale: O_spatialproperty{
                o_point_2d__current: O_point_2d{
                    n_x: n_scale_x,
                    n_y: n_scale_y
                }, 
                o_point_2d__velocity: O_point_2d{n_x: 0.0, n_y: 0.0}, 
                o_point_2d__acceleration: O_point_2d{n_x: 0.0, n_y: 0.0},
            },
            // a_n_f64__color: vec![(n_avg / 255.0).into(), 0.0,0.0, 1.0]

            f_a_n_f64__color: f_a_n_f64__color__red

        };

        let a_n_u8__subframe = f_a_rect_read_and_optional_write(
            &mut a_n_u8__image,
            n_image_pixels_x as u32, 
            n_image_pixels_y as u32, 
            n_channels as u32,
            o_object_2d_wall.o_spatialproperty__translation.o_point_2d__current.n_x as u32,
            o_object_2d_wall.o_spatialproperty__translation.o_point_2d__current.n_y as u32,
            o_object_2d_wall.o_spatialproperty__scale.o_point_2d__current.n_x as u32,
            o_object_2d_wall.o_spatialproperty__scale.o_point_2d__current.n_y as u32,
            None
        );

        let mut n_sum: u32 = 0; 
        let mut n_i_a_n_u8__subframe = 0;
        // while(n_i_a_n_u8__subframe < a_n_u8__subframe.len()){
        //     n_sum += a_n_u8__subframe[n_i_a_n_u8__subframe] as u32;
        //     n_i_a_n_u8__subframe+=1;
        // }
        for n_u8 in a_n_u8__subframe.iter(){
            n_sum += *n_u8 as u32
        }
        let n_avg = n_sum as f32 / a_n_u8__subframe.len() as f32;

         // let sum: u32 = a_n_u8__subframe.iter().sum();
        
        println!("sum a_n_u8__subframe {:?}", n_sum);
        println!("n_avg a_n_u8__subframe {:?}", n_avg);

        if(n_avg < 110.0){        
            o_game.a_o_object_2d.push(
                O_object_2d{
                    s_name: String::from("wall"),
                    o_spatialproperty__translation: O_spatialproperty{
                        o_point_2d__current: O_point_2d{
                            n_x : n_x as f64 * o_game.o_object_2d_box.o_spatialproperty__scale.o_point_2d__current.n_x,
                            n_y : n_y as f64 * o_game.o_object_2d_box.o_spatialproperty__scale.o_point_2d__current.n_y
                        }, 
                        o_point_2d__velocity: O_point_2d{n_x: 0.0, n_y: 0.0}, 
                        o_point_2d__acceleration: O_point_2d{n_x: 0.0, n_y: 0.0},
                    },
                    o_spatialproperty__rotation: O_spatialproperty{
                        o_point_2d__current: O_point_2d{n_x: 0.0, n_y: 0.0}, 
                        o_point_2d__velocity: O_point_2d{n_x: 0.0, n_y: 0.0}, 
                        o_point_2d__acceleration: O_point_2d{n_x: 0.0, n_y: 0.0},
                    },
                    o_spatialproperty__scale: O_spatialproperty{
                        o_point_2d__current: O_point_2d{
                            n_x: o_game.o_object_2d_box.o_spatialproperty__scale.o_point_2d__current.n_x,
                            n_y: o_game.o_object_2d_box.o_spatialproperty__scale.o_point_2d__current.n_y
                        }, 
                        o_point_2d__velocity: O_point_2d{n_x: 0.0, n_y: 0.0}, 
                        o_point_2d__acceleration: O_point_2d{n_x: 0.0, n_y: 0.0},
                    },
                    // a_n_f64__color: vec![(n_avg / 255.0).into(), 0.0,0.0, 1.0]
                    // a_n_f64__color: vec![0.0,0.0,0.0,0.0]
                    f_a_n_f64__color: f_a_n_f64__color__red
                    // a_n_f64__color:  vec![(n_avg / 255.0).into(), 0.0,0.0, 1.0]
                }
            );
        }
        n_i+=1;
    }

    
    // let a_o_object_2d__wall: Vec<O_object_2d> = a_o_object_2d
    // .into_iter()
    // .filter(|obj_object_2d| obj_object_2d.s_name == "wall")
    // .collect();
    // let o_object_2d_wall = &a_o_object_2d__wall[0];

    
}