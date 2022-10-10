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
use std::path::Path;

pub async fn f_text_to_png(
    s_url_or_path_to_font_file: String, 
    s_text: String, 
    n_size: f32, 
    s_path_file_png: String
){
    let mut s_path_file_urlfont = 

    if(
        Path::new("/etc/hosts").exists() == false
    ){
        
    }

    if(
        s_url_or_path_to_font_file.contains("http://")
        ||
        s_url_or_path_to_font_file.contains("https://")
    ){
        //download font
        let s_name_file_urlfont = // "https://fonts.cdnfonts.com/s/88767/Nollasans.ttf"
            String::from(s_url_or_path_to_font_file.clone().rsplit_once('/').unwrap().1);

        let s_url_font_file_extension = // "ttf"
            String::from(s_name_file_urlfont.rsplit_once('.').unwrap().1);

        let s_name_file_urlfont_without_extension =  // "Nollasans"
            String::from(s_name_file_urlfont.rsplit_once('.').unwrap().0);
        
        let mut s_path_file_urlfont = // "./Nollasans.ttf"
            vec![String::from("./"), s_name_file_urlfont.clone()].join("");
        
        // println!("s_name_file_urlfont {:?}", s_name_file_urlfont);
        // println!("s_url_font_file_extension {:?}", s_url_font_file_extension);
        // println!("s_name_file_urlfont_without_extension {:?}", s_name_file_urlfont_without_extension);
        // println!("s_path_file_urlfont {:?}", s_path_file_urlfont);
    
        let resp = reqwest::get(String::from(s_url_or_path_to_font_file)).await.expect("request failed");
        let body = resp.text().await.expect("body invalid");
        let mut out = File::create(s_path_file_urlfont.clone()).expect("failed to create file");
        io::copy(&mut body.as_bytes(), &mut out).expect("failed to copy content");

    }

    println!("calllllleeeed!!!")
}