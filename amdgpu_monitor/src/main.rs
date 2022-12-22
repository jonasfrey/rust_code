extern crate sdl2;
use sdl2::image::LoadTexture;
use sdl2::pixels::PixelFormatEnum;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::render::{Canvas, Texture};
use sdl2::surface::Surface;
use std::io::Cursor;
// use image::io::Reader as ImageReader;



pub fn main() {

  // let img = ImageReader::open("./brain.jpg").unwrap().decode().unwrap();
  //let img2 = ImageReader::new(Cursor::new(bytes)).with_guessed_format()?.decode()?;
  // println!("{:?}",img);
  // let a_n_image_data = img.as_bytes();
  // println!("{:?}",a_n_image_data);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .set_window_flags( 
          32768//sdl2::sys::SDL_WindowFlags::SDL_WINDOW_ALWAYS_ON_TOP
          //|
          //16 // borderless
        )
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    let mut o_texture_creator = canvas.texture_creator();
    // let o_text = sdl2::image::LoadTexture();
    let mut o_te = o_texture_creator.load_texture("./brain.jpg").unwrap();
    
    // let surface = Surface::new(512, 512, PixelFormatEnum::RGBA8888).unwrap();
    
    // let o_texture = o_texture_creator.create_texture_from_surface(surface).unwrap();
    // canvas.with_texture_canvas(&mut o_te, |texture_canvas| {
    //   texture_canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
    //   texture_canvas.clear();
    //   texture_canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
    //   texture_canvas.fill_rect( sdl2::rect::Rect::new(50, 50, 50, 50)).unwrap();
  // });
    'running: loop {
        // i = (i + 1) % 255;
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        // canvas.clear();
        canvas.copy(&o_te,None, None);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...


        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}