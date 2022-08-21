use device_query::{DeviceQuery, DeviceState, Keycode};
use mouse_rs::{types::keys::Keys, Mouse};

use std::time::{Duration, SystemTime};
use std::thread::sleep;

fn main() {
    let now = SystemTime::now();
    let mut n_i = 0; 
    let mouse = Mouse::new();

    let device_state = DeviceState::new();
    loop {
        sleep(Duration::from_millis(100));
        let keys: Vec<Keycode> = device_state.get_keys();
        for key in keys.iter() {
            if matches!(key, Keycode::LShift){
                println!("sfad");
                // println!("{}", elapsed.as_secs());
                mouse.press(&Keys::LEFT).expect("Unable to press button");
                mouse.release(&Keys::LEFT).expect("Unable to release button");
            }else{
                // println!("{}", elapsed.as_secs());
            }
            // if key.  Keycode::Space {
                // println!("fasd");
            // }
            // println!("Pressed key: {:?}", key);
        }
    }
}