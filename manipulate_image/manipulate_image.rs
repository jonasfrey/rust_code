
// c file 
// This is a comment, and is ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if you prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

// This is the main function
fn main() {
    // Statements here are executed when the compiled binary is called

    use std::io::Cursor;
    use image::io::Reader as ImageReader;
    use image::GenericImageView;
    
    // let img = ImageReader::open("naneb.png")?.decode()?;
    let img = image::open("./naneb.jpg").unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());

    // img.brighten(2_i32.pow(12_u32));
    img.adjust_contrast(10.0_f32);
    // Write the contents of this image to the Writer in PNG format.
    img.save("naneb.png").unwrap();
    // println!("{:?}", img);
}
