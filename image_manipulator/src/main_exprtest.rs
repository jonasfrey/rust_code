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
use eval::{Expr, to_value};
use std::collections::HashMap;


// use woff2::decode::{convert_woff2_to_ttf, is_woff2};

fn f_print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


// // example input json 



#[tokio::main]
async fn main(){

    let mut o_expr = Expr::new("512.0/2");
    // let mut o_expr = Expr::new(String::from("512.0/2"));
    // let mut o_expr = Expr::new(String::from("512.0/2.0"));
    let mut n_translation_x_evaluated = o_expr.exec().unwrap().as_f64().unwrap();
    println!("n_translation_x_evaluated {:?}", n_translation_x_evaluated);

}