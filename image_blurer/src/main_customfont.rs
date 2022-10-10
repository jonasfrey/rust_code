use std::io::Cursor;
use text_to_png::TextRenderer;
use image::io::Reader as ImageReader;
use image::GenericImageView;
use std::sync::Once;
use std::fs::File;
use std::io::Write;
use serde_json::{Result, Value};
use std::env;
use image::{ImageBuffer, Rgba, RgbaImage};
use std::process;
use std::io::Read;
use std::io;
mod f_text_to_png;

// use woff2::decode::{convert_woff2_to_ttf, is_woff2};

fn f_print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


#[tokio::main]
async fn main(){

    let s_url = String::from("https://fonts.cdnfonts.com/s/88767/Nollasans.ttf");
    let s_text = String::from("lol xD");
    let n_size = 20.0;
    let s_path_file_png = String::from("./f_text_to_png_test.png");

    f_text_to_png::f_text_to_png(
        s_url,
        s_text,
        n_size,
        s_path_file_png
    ).await;

}