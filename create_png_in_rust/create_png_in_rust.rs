
// c file 
// This is a comment, and is ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if you prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

// This is the main function
// fn main() {
//     // Statements here are executed when the compiled binary is called

//     // Print text to the console
//     println!("Hello World!");
// }
extern crate image;

use rand::Rng;
use image::GenericImageView;

fn f_create_fractal_png(){
    let imgx = 800;
    let imgy = 800;

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    // A redundant loop to demonstrate reading image data
    for x in 0..imgx {
        for y in 0..imgy {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;

            let c = num_complex::Complex::new(-0.4, 0.6);
            let mut z = num_complex::Complex::new(cx, cy);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(x, y);
            let image::Rgb(data) = *pixel;
            *pixel = image::Rgb([data[0], i as u8, data[2]]);
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("fractal.png").unwrap();
}

fn f_create_green_png(){
    let n_width = 800;
    let n_height = 800;



    // Create a new ImgBuf with width: imgx and height: imgy
    let mut a_image_buffer = image::ImageBuffer::new(n_width, n_height);

    // Iterate over the coordinates and pixels of the image
    for (n_x, n_y, o_pixel) in a_image_buffer.enumerate_pixels_mut() {
        
        let n_r = 1.0 as f32;
        let n_g = 0.0 as f32;
        let n_b = 0.0 as f32;

        *o_pixel = image::Rgb([(n_r*255.0) as u8, (n_g*255.0) as u8, (n_b*255.0) as u8]);
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    a_image_buffer.save("green.png").unwrap();
}
fn f_create_random_png(){

    let mut rng = rand::thread_rng();

    let n_width = 5000; 
    let n_height = 5000;

    let mut a_image_buffer = image::ImageBuffer::new(n_width, n_height); 

    for(n_x, n_y, o_pixel) in a_image_buffer.enumerate_pixels_mut(){
        let n_r: f32 = rng.gen();
        let n_g: f32 = rng.gen();
        let n_b: f32 = rng.gen();
        *o_pixel = image::Rgb([
            (n_r*255.0) as u8,
            (n_g*255.0) as u8,
            (n_b*255.0) as u8,
        ])
    }
    a_image_buffer.save("random_image.png").unwrap();
}
fn main() {
    // f_create_fractal_png(); 
    // f_create_green_png();
    f_create_random_png();
}