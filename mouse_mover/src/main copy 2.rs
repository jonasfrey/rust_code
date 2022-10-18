
use device_query::{DeviceQuery, DeviceState, MouseState, Keycode};
use std::time::{Duration, Instant};
use std::{thread, time};

use enigo::*;


fn main() {
    
    println!("Hello, world!");
    
    let device_state = DeviceState::new();

    let mouse: MouseState = device_state.get_mouse();
    println!("Current Mouse Coordinates: {:?}", mouse.coords);
    
    let mut o_enigo = Enigo::new();
    // o_enigo.mouse_move_to(500, 200);
    // o_enigo.mouse_down(MouseButton::Left);
    // o_enigo.mouse_move_relative(100, 100);
    // o_enigo.mouse_up(MouseButton::Left);
    // o_enigo.key_sequence("hello world");

    let mut n_acceleration = 0.5;
    let mut n_decelleration = n_acceleration*2.0;
    let mut b_accelerate = false;
    // let mut n_speed_initial = 1.0; // 
    let mut n_speed_initial = 2.0; // 
    let mut n_speed = n_speed_initial;

    let mut n_update_fps = 60.0;
    let mut n_update_milliseconds_delta = 1000.0/n_update_fps;

    let o_instant_now = Instant::now();
    let mut n_ts_milliseconds : u128 = 0;
    let mut n_ts_milliseconds_last : u128 = 0;
    let mut n_ts_milliseconds_delta : u128 = 0;

    let o_ts_sleep_milliseconds = time::Duration::from_millis(n_update_milliseconds_delta as u64);

    let mut b_mouse_down_left = false;
    let mut b_mouse_down_left_last = false;
    let mut b_mouse_down_right = false;
    let mut b_mouse_down_right_last = false;

    loop{
        n_ts_milliseconds = o_instant_now.elapsed().as_millis();
        n_ts_milliseconds_delta = n_ts_milliseconds - n_ts_milliseconds_last;
        let a_o_keycode: Vec<Keycode> = device_state.get_keys();
        b_accelerate = false;

        thread::sleep(o_ts_sleep_milliseconds);

        if(
            a_o_keycode.contains(&Keycode::Grave)
        ){
            if(
                a_o_keycode.contains(&Keycode::J)
            ){ 
                b_accelerate = true;
                n_speed+=n_acceleration; //linear
                // n_speed*=n_speed*0.005+n_speed_initial;// exponential
                println!("speed : {:?}", n_speed);
                o_enigo.mouse_move_relative(-n_speed as i32,0);

            }
            if(
                a_o_keycode.contains(&Keycode::L)
            ){ 
                b_accelerate = true;
                n_speed+=n_acceleration;
                o_enigo.mouse_move_relative(n_speed as i32,0);
                
            }
            if(
                a_o_keycode.contains(&Keycode::I)
            ){ 
                b_accelerate = true;
                n_speed+=n_acceleration;
                o_enigo.mouse_move_relative(0,-n_speed as i32);
                
            }
            if(
                a_o_keycode.contains(&Keycode::K)
            ){ 
                b_accelerate = true;
                n_speed+=n_acceleration;
                o_enigo.mouse_move_relative(0,n_speed as i32);
                
            }
            if(
                a_o_keycode.contains(&Keycode::Semicolon)
            ){ 
                o_enigo.mouse_scroll_y(1);
            }
            if(
                a_o_keycode.contains(&Keycode::P)
            ){ 
                o_enigo.mouse_scroll_y(-1);
            }
            if(
                a_o_keycode.contains(&Keycode::U)
            ){ 
                b_mouse_down_left = true;
            }else{
                b_mouse_down_left = false;
            }
            if(
                a_o_keycode.contains(&Keycode::O)
            ){ 
                b_mouse_down_right = true;
            }else{
                b_mouse_down_right = false;
            }
            if(b_mouse_down_left && !b_mouse_down_left_last){
                o_enigo.mouse_down(MouseButton::Left);
            }
            if(!b_mouse_down_left && b_mouse_down_left_last){
                o_enigo.mouse_up(MouseButton::Left);
            }
            if(b_mouse_down_right && !b_mouse_down_right_last){
                o_enigo.mouse_down(MouseButton::Right);
            }
            if(!b_mouse_down_right && b_mouse_down_right_last){
                o_enigo.mouse_up(MouseButton::Right);
            }
            println!("key pressed");
        }
        if(!b_accelerate){
            // if(n_speed >= n_speed_initial){
            //     n_speed -= n_decelleration;
            // }
            n_speed = n_speed_initial
        }

        n_ts_milliseconds_last = n_ts_milliseconds;

        b_mouse_down_left_last = b_mouse_down_left;
        b_mouse_down_right_last = b_mouse_down_right;
    }

}
