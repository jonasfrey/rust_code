use enigo::*;

use std::{thread, time, io::stdout, time::Duration};



// use crossterm::{
//     cursor::position,
//     event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
//     execute,
//     terminal::{disable_raw_mode, enable_raw_mode},
//     Result,
// };
use macroquad::prelude::*;

// sudo apt install libxdo-dev
fn main()  {
    println!("Hello, test!");

    let mut enigo = Enigo::new();

    // enigo.mouse_move_to(500, 200);



    // enigo.mouse_move_relative(100, 100);

    // enigo.mouse_up(MouseButton::Left);

    // enigo.key_sequence("hello world");

    let mut n_i = 0; 

    while n_i < 1000 {

        thread::sleep(time::Duration::from_millis(10));
        println!("{} asdf", n_i);      
        // enigo.mouse_down(MouseButton::Left);
        n_i += 1;
    }


    // loop {
    //     // Wait up to 1s for another event
    //     if poll(Duration::from_millis(10))? {
    //         // It's guaranteed that read() wont block if `poll` returns `Ok(true)`
    //         let event = read()?;

    //         println!("Event::{:?}\r", event);

    //         if event == Event::Key(KeyCode::Char(' ').into()) {
    //             // println!("Cursor position: {:?}\r", position());
    //             println!("asdf");
    //         }

    //         if event == Event::Key(KeyCode::Esc.into()) {
    //             break;
    //         }
    //     } else {
    //         // Timeout expired, no event for 1s
    //         println!(".\r");
    //     }
    // }

    // Ok(())


}



// use macroquad::prelude::*;

// #[macroquad::main("BasicShapes")]
// async fn main() {
//     loop {
//         clear_background(RED);

//         draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
//         draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
//         draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

//         draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
        
//         if is_key_down(KeyCode::Right) {
//             println!("right");
//         }
//         if is_key_down(KeyCode::Left) {
//             println!("left");
//         }
//         next_frame().await
//     }
// }