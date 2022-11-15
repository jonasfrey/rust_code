use enigo::*;

use device_query::{DeviceQuery, DeviceState, MouseState, Keycode};

use std::{thread, time};


fn main() {

    let mut o_enigo = Enigo::new();
    // enigo.mouse_move_to(500, 200);
    // o_enigo.mouse_down(MouseButton::Left);
    // enigo.mouse_move_relative(100, 100);
    // o_enigo.mouse_up(MouseButton::Left);
    // enigo.key_sequence("hello world");

    let o_ten_millis = time::Duration::from_millis(5);
    let o_now = time::Instant::now();

    let o_device_state = DeviceState::new();


    loop{
        println!("press and hold the left shift key to click like a maniac!!!");
        let a_o_keycode: Vec<Keycode> = o_device_state.get_keys();
        if a_o_keycode.contains(&Keycode::LShift){            
            o_enigo.mouse_down(MouseButton::Left);
            thread::sleep(o_ten_millis);
            o_enigo.mouse_up(MouseButton::Left);

            println!("test");
        }else{
            o_enigo.mouse_up(MouseButton::Left);
        }
    }

}
