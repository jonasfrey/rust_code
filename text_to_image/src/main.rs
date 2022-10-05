use text_to_png::TextRenderer;
use std::fs::File;
use std::io::prelude::*;
use image::io::Reader as ImageReader;
use image::GenericImageView;

fn f_create_text_png_with_outline(
    s_text: String, 
    s_color_inside: String, 
    s_color_outside: String,
    n_size: u32
){

    let o_renderer_outside = TextRenderer::try_new_with_ttf_font_data(include_bytes!("OpenSans-Semibold.ttf"))
    .expect("Example font is definitely loadable");

    let o_renderer_inside = TextRenderer::try_new_with_ttf_font_data(include_bytes!("OpenSans-Regular.ttf"))
    .expect("Example font is definitely loadable");

    let o_text_png = o_renderer_outside.render_text_to_png_data(
        s_text.as_str(),
        n_size,
        s_color_outside.as_str(),
    ).unwrap();
    let s_path_image_outside = "o_text_png_outside.png";
    let mut o_file_outside = File::create(s_path_image_outside).unwrap();
    let a: &[u8] = &o_text_png.data;
    o_file_outside.write_all(a).unwrap();

    let o_text_png_inside = o_renderer_inside.render_text_to_png_data(
        s_text.as_str(),
        n_size,
        s_color_inside.as_str(),
    ).unwrap();
    
    let s_path_image_inside = "o_text_png_inside.png";
    let mut o_file_inside = File::create(s_path_image_inside).unwrap();
    let a: &[u8] = &o_text_png_inside.data;
    o_file_inside.write_all(a).unwrap();


    let mut o_image_outside = ImageReader::open(s_path_image_outside).unwrap().decode().unwrap();
    let mut o_image_inside = ImageReader::open(s_path_image_inside).unwrap().decode().unwrap();


    let mut o_image_overlayed = o_image_outside.clone();
    let n_overlay_translation_x = ((o_image_outside.width() as f32 /2.0) - (o_image_inside.width() as f32 / 2.0) ) as i64;
    let n_overlay_translation_y = ((o_image_outside.height() as f32 /2.0) - (o_image_inside.height() as f32 / 2.0) ) as i64;
    image::imageops::overlay(
        &mut o_image_overlayed,
        &o_image_inside,
        n_overlay_translation_x,
        n_overlay_translation_y
    );
    o_image_overlayed.save("o_text_png_overlayed.png").unwrap();


}
fn main() {

    // let renderer = TextRenderer::default();
    
    // let s_path_font = String::from("./OpenSans-Light.ttf");
    
    f_create_text_png_with_outline(
        String::from("T"), 
        String::from("black"), 
        String::from("red"), 
        100
    );

}
