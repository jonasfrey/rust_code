// if not working on linux
//      debian 
//      sudo apt-get install -y libxdo-dev
use mouse_rs::{types::keys::Keys, Mouse};

use std::{io::stdout, time::Duration};

use crossterm::{
    cursor::position,
    event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};

use std::{thread, time};

use inputbot::{KeySequence, KeybdKey::*, MouseButton::*};
use std::{thread::sleep};

fn f_inputbot_demo() {
    // Bind the number 1 key your keyboard to a function that types 
    // "Hello, world!" when pressed.
    EKey.bind(|| KeySequence("Hello, world!").send());

    // // Bind your caps lock key to a function that starts an autoclicker.
    // CapsLockKey.bind(move || {
    //     while CapsLockKey.is_toggled() {
    //         LeftButton.press();
    //         LeftButton.release();

    //         sleep(Duration::from_millis(30));
    //     }
    // });
           
    
    
    // Call this to start listening for bound inputs.
    inputbot::handle_input_events();
}


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


fn move_and_press() {
    let mouse = Mouse::new();
    mouse.move_to(500, 500).expect("Unable to move mouse");
    mouse.press(&Keys::RIGHT).expect("Unable to press button");
    mouse.release(&Keys::RIGHT).expect("Unable to release button");
}

fn main() -> Result<()>  {
    println!("Hello, world!");
    // move_and_press();
    let mouse = Mouse::new();
    let mousepos = mouse.get_position().expect("unable to get mouse position");

    // println!("{:?}", mousepos);
    println!("{:?}", mousepos.x);
    println!("{:?}", mousepos.y);

    let ten_millis = time::Duration::from_millis(10);
    // let now = time::Instant::now();
    
    // let mut n_i = 0; 
    // while n_i < 100000 {
    //     let mousepos = mouse.get_position().expect("unable to get mouse position");

    //     let event = read()?;

    //     println!("Event::{:?}\r", event);

    //     if event == Event::Key(KeyCode::Char('c').into()) {
    //         println!("key c is pressed");
    //         // println!("Cursor position: {:?}\r", position());
    //     }

    //     println!("{:?}", mousepos.x);
    //     println!("{:?}", mousepos.y);
    //     thread::sleep(ten_millis);
    //     n_i+=1;
    // }
    enable_raw_mode()?;

    loop {
        // Wait up to 1s for another event
        if poll(Duration::from_millis(100))? {
            // It's guaranteed that read() wont block if `poll` returns `Ok(true)`
            let event = read()?;

            println!("Event::{:?}\r", event);

            if event == Event::Key(KeyCode::Char(' ').into()) {
                println!("c is pressed");
                // println!("Cursor position: {:?}\r", position());
            }

            if event == Event::Key(KeyCode::Esc.into()) {
                break;
            }
        } else {
            // Timeout expired, no event for 1s
            // println!(".\r");
        }
    }

    Ok(())

    // disable_raw_mode()
    
    // f_inputbot_demo();

    // print_type_of(&mousepos); // &str
}