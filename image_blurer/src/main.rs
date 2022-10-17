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


// use woff2::decode::{convert_woff2_to_ttf, is_woff2};

fn f_print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


// // example input json 



#[tokio::main]
async fn main(){



    let o_param_default: Value = serde_json::from_str(r#"
    {
        "o_image_output": {
            "n_scale_x": 1920, 
            "n_scale_y": 1080,
            "n_float_blur_sigma": 100.0, 
            "s_file_extension": ".png"
        },
        "o_image_input":{
            "s_path_file": "./default.png"
        },
        "a_o_text": {
            "n_translation_y": 10,
            "n_translation_x": 10,
            "n_scale_x": 10,
            "n_scale_y": 10,
            "n_rotation_x": 10,
            "n_rotation_y": 10,
            "s": "this is an example",
            "a_n_f64__color_rgba": [0,0,0,0],
            "a_n_f64__bgcolor_rgba": [0,0,0,0],
            "n_padding_text_to_background": 20
        }
    }
    "#).unwrap();
    
    // scale == size , how many pixels width
    // translation == position, a fixed point somewhere
    // rotation == rotation

    let a_s_arg: Vec<String> = env::args().collect();
    let mut s_first_arg = a_s_arg[1].to_owned();
    println!("s_first_arg {:?}", s_first_arg);

    println!("s_first_arg {}", s_first_arg);
    let o_param: Value = serde_json::from_str(&s_first_arg).unwrap();

    // i am to lazy for full validaton...
    if(o_param["o_image_input"] == serde_json::Value::Null){
        println!("s_prop_name 'o_image_input' is required");
        process::exit(1);
    }

    let mut s_outpupt_file_extension = String::from(".png");// defaultt value
    if(o_param["o_image_output"]["s_file_extension"] != serde_json::Value::Null){ 
        s_outpupt_file_extension = o_param["o_image_output"]["s_file_extension"].as_str().unwrap().to_string();
    }
    let mut n_o_image_output_n_scale_x = 1920 as u64;// defaultt value
    if(o_param["o_image_output"]["n_o_image_output_n_scale_x"] != serde_json::Value::Null){ 
        n_o_image_output_n_scale_x = o_param["o_image_output"]["n_o_image_output_n_scale_x"].as_u64().unwrap();
    }
    let mut n_o_image_output_n_scale_y = 1080 as u64;// defaultt value
    if(o_param["o_image_output"]["n_o_image_output_n_scale_y"] != serde_json::Value::Null){ 
        n_o_image_output_n_scale_y = o_param["o_image_output"]["n_o_image_output_n_scale_y"].as_u64().unwrap();
    }


    let n_ratio: f64 = (n_o_image_output_n_scale_x as f64 / n_o_image_output_n_scale_y as f64);


    let s_name_file_without_extension = String::from(o_param["o_image_input"]["s_path_file"].as_str().unwrap().rsplit_once('.').unwrap().0);
    println!("s_name_file_without_extension {:?}", s_name_file_without_extension);

    // std::process::exit(1);

    let mut o_image = ImageReader::open(o_param["o_image_input"]["s_path_file"].as_str().unwrap()).unwrap().decode().unwrap();

    let n_o_image_scale_x = o_image.width();
    let n_o_image_scale_y = o_image.height();
    
    let n_o_image_ratio: f64 = (n_o_image_scale_x as f64/ n_o_image_scale_y as f64);

    let mut n_crop_tranlsation_x = 0; 
    let mut n_crop_translation_y = 0;
    let mut n_crop_scale_x = 0.0;
    let mut n_crop_scale_y = 0.0;

    let b_ratio_equal =  n_o_image_ratio >= 1.0 && n_ratio >= 1.0;
    if(b_ratio_equal){
        n_crop_scale_x = (n_o_image_scale_x as f32);
        n_crop_scale_y = (n_o_image_scale_y as f32 * (1.0/n_ratio as f32));
    }else{
        n_crop_scale_x = (n_o_image_scale_x as f32 * (1.0/n_ratio as f32));
        n_crop_scale_y = (n_o_image_scale_y as f32);
    }

    println!("n_crop_scale_x {:?}", n_crop_scale_x);
    println!("n_crop_scale_y {:?}", n_crop_scale_y);

    let mut o_image_cropped = image::DynamicImage::ImageRgba8(
        image::imageops::crop(
            &mut o_image,
            n_crop_tranlsation_x,
            n_crop_translation_y, 
            n_crop_scale_x as u32,
            n_crop_scale_y as u32,
            // image::imageops::FilterType::Nearest,
            // image::imageops::FilterType::Triangle,
            // image::imageops::FilterType::CatmullRom,
            // image::imageops::FilterType::Gaussian,
            // image::imageops::FilterType::Lanczos3,
        ).to_image()
    );
    o_image_cropped.save(vec![s_name_file_without_extension.clone(), String::from("_cropped"), s_outpupt_file_extension.clone()].join("")).unwrap();


    let mut o_image_resized = image::DynamicImage::ImageRgba8(
        image::imageops::resize(
            &mut o_image_cropped,
            n_o_image_output_n_scale_x as u32, 
            n_o_image_output_n_scale_y as u32,
            image::imageops::FilterType::Nearest,
            // image::imageops::FilterType::Triangle,
            // image::imageops::FilterType::CatmullRom,
            // image::imageops::FilterType::Gaussian,
            // image::imageops::FilterType::Lanczos3,
        )
    );
    o_image_resized.save(vec![s_name_file_without_extension.clone(), String::from("_resized"), s_outpupt_file_extension.clone()].join("")).unwrap();

    let mut o_image_blurred = image::DynamicImage::ImageRgba8(
        image::imageops::blur(
            &mut o_image_resized,
            o_param["o_image_output"]["n_float_blur_sigma"].as_f64().unwrap() as f32 
        )
    );
    o_image_blurred.save(vec![s_name_file_without_extension.clone(), String::from("_blurred"), s_outpupt_file_extension.clone()].join("")).unwrap();
    println!("saved blurred file");
    

    let mut o_image_overlayed = o_image_blurred.clone();
    let n_overlay_translation_x = ((n_o_image_output_n_scale_x as f64 /2.0) - (o_image.width() as f64 / 2.0) ) as i64;
    let n_overlay_translation_y = ((n_o_image_output_n_scale_y as f64 /2.0) - (o_image.height() as f64 / 2.0) ) as i64;
    println!("n_overlay_translation_x {:?}", n_overlay_translation_x);
    println!("n_overlay_translation_y {:?}", n_overlay_translation_y);
    image::imageops::overlay(
        &mut o_image_overlayed,
        &o_image,
        n_overlay_translation_x as i64,
        n_overlay_translation_y as i64
    );
    

    o_image_overlayed.save(vec![s_name_file_without_extension.clone(), String::from("_overlayed"), s_outpupt_file_extension.clone()].join("")).unwrap();



    let a_o_text = o_param["a_o_text"].as_array().unwrap();

    let mut n_i_a_o_text = 0; 
    let mut o_image_overlayed_text = o_image_overlayed.clone();

    // println!("a_o_text.len(), {:?}", a_o_text.len());
    while(n_i_a_o_text < a_o_text.len()){
        println!("o_text");
        let o_text = &a_o_text[n_i_a_o_text];
        
        let mut n_max_width_normalized = 1.0 as f64;// defaultt value
        if(o_text["n_max_width_normalized"] != serde_json::Value::Null){ 
            n_max_width_normalized = o_text["n_max_width_normalized"].as_f64().unwrap();
        }

        let mut s_path_font_ttf = None;// defaultt value
        if(o_text["s_path_font_ttf"] != serde_json::Value::Null){ 
            s_path_font_ttf = Some(String::from(o_text["s_path_font_ttf"].as_str().unwrap()));
        }
        
        let mut s_url_font = None;
        if(o_text["s_url_font"] != serde_json::Value::Null){ 
            s_url_font = Some(String::from(o_text["s_url_font"].as_str().unwrap()));
        }

        if(s_url_font.is_some()){


            let s_name_file_urlfont = String::from(s_url_font.as_ref().unwrap().clone().rsplit_once('/').unwrap().1);
            let s_url_font_file_extension = String::from(s_name_file_urlfont.rsplit_once('.').unwrap().1);
            let s_name_file_urlfont_without_extension = String::from(s_name_file_urlfont.rsplit_once('.').unwrap().0);
            let mut s_path_file_urlfont = vec![String::from("./"), s_name_file_urlfont.clone()].join("");
            println!("s_name_file_urlfont {:?}", s_name_file_urlfont);
            println!("s_url_font_file_extension {:?}", s_url_font_file_extension);
            println!("s_name_file_urlfont_without_extension {:?}", s_name_file_urlfont_without_extension);
            println!("s_path_file_urlfont {:?}", s_path_file_urlfont);

            let resp = reqwest::get(String::from(s_url_font.unwrap())).await.expect("request failed");
            let body = resp.text().await.expect("body invalid");
            let mut out = File::create(s_path_file_urlfont.clone()).expect("failed to create file");
            io::copy(&mut body.as_bytes(), &mut out).expect("failed to copy content");
            // let s: String = "abcdefg".to_owned();
            // let s_slice: &str = &s[..];  // take a full slice of the string
            // if(s_url_font_file_extension == "woff2"){
            //     let mut s_path_file_urlfont_ttf = vec![String::from("./"), s_name_file_urlfont_without_extension.clone(), String::from(".ttf")].join("");

                
            //     let buffer = std::fs::read(s_path_file_urlfont.clone()).unwrap();
            //     assert!(is_woff2(&buffer));
            //     let ttf = convert_woff2_to_ttf(&mut std::io::Cursor::new(buffer)).unwrap();
            //     let mut o_file_ttf = File::create(s_path_file_urlfont_ttf.clone()).expect("failed to create file");
            //     let mut a_bytes: &[u8] = &ttf; // c: &[u8]
            //     io::copy(&mut a_bytes, &mut o_file_ttf).expect("failed to copy content");
            // }

            s_path_font_ttf = Some(s_path_file_urlfont.clone());

            
        }


        
        // let mut o_response = reqwest::get(s_url_font_ttf.unwrap()).await.expect("request failed");
        // let s_name_file_urlfont = String::from(s_url_font_ttf.unwrap().rsplit_once('/').unwrap().1);
        // // let s_name_file_urlfont_without_extension = String::from(s_name_file_urlfont.rsplit_once('.').unwrap().0);
        // let mut s_path_file_urlfont = vec![String::from("./"), s_name_file_urlfont].join("")
        // let mut o_file_urlfont = File::create(s_path_file_urlfont).expect("failed to create file");
        // std::io::copy(&mut o_response, &mut o_file_urlfont).expect("failed to copy content");

        let mut n_rotation_x = 0;// defaultt value
        if(o_text["n_rotation_x"] != serde_json::Value::Null){ 
            n_rotation_x = o_text["n_rotation_x"].as_u64().unwrap();
        }
        
        let mut n_rotation_y = 0;// defaultt value
        if(o_text["n_rotation_y"] != serde_json::Value::Null){ 
            n_rotation_y = o_text["n_rotation_y"].as_u64().unwrap();
        }
        
        let mut s = "please provide a text";// defaultt value
        if(o_text["s"] != serde_json::Value::Null){ 
            s = o_text["s"].as_str().unwrap();
        }
        
        let mut n_font_size_factor = 1.0;// defaultt value
        if(o_text["n_font_size_factor"] != serde_json::Value::Null){ 
            n_font_size_factor = o_text["n_font_size_factor"].as_f64().unwrap();
        }
        
        let mut a_n_f64__color_rgb : Vec<f64> = vec![0.0,0.0,0.0];// defaultt value
        if(o_text["a_n_f64__color_rgb"] != serde_json::Value::Null){ 
            a_n_f64__color_rgb =  vec![
                o_text["a_n_f64__color_rgb"].as_array().unwrap()[0].as_f64().unwrap(),
                o_text["a_n_f64__color_rgb"].as_array().unwrap()[1].as_f64().unwrap(),
                o_text["a_n_f64__color_rgb"].as_array().unwrap()[2].as_f64().unwrap(),
            ]
        }
        
        let mut a_n_f64__bgcolor_rgba : Vec<f64> = vec![0.2,0.2,0.2,0.2];// defaultt value
        if(o_text["a_n_f64__bgcolor_rgba"] != serde_json::Value::Null){ 
            a_n_f64__bgcolor_rgba =  vec![
                o_text["a_n_f64__bgcolor_rgba"].as_array().unwrap()[0].as_f64().unwrap(),
                o_text["a_n_f64__bgcolor_rgba"].as_array().unwrap()[1].as_f64().unwrap(),
                o_text["a_n_f64__bgcolor_rgba"].as_array().unwrap()[2].as_f64().unwrap(),
                o_text["a_n_f64__bgcolor_rgba"].as_array().unwrap()[3].as_f64().unwrap(),
            ]
        }

        let mut s_break_on = String::from(" -");// defaultt value
        if(o_text["s_break_on"] != serde_json::Value::Null){ 
            s_break_on = String::from(o_text["s_break_on"].as_str().unwrap());
        }
        
        // this has no default 
        let mut n_font_size_pixel  = None;
        if(o_text["n_font_size_pixel"] != serde_json::Value::Null){ 
            n_font_size_pixel  = Some(o_text["n_font_size_pixel"].as_i64().unwrap());
        }

        let mut n_padding_text_to_background = 10;// defaultt value
        if(o_text["n_padding_text_to_background"] != serde_json::Value::Null){ 
            n_padding_text_to_background =  o_text["n_padding_text_to_background"].as_u64().unwrap();
        }

        // put text 
        let s_text = String::from(s);
        const n_factor_pixels_per_character_average: f64 = 1.8; // needs to be static !
        let n_factor_with_max_width = n_factor_pixels_per_character_average * n_max_width_normalized;
        let mut n_font_size = 
            (((n_o_image_output_n_scale_x as f64 / s_text.chars().count() as f64) * n_factor_with_max_width)
            * n_font_size_factor as f64)
            as u32;
        if(n_font_size_pixel.is_some()){
            n_font_size = n_font_size_pixel.unwrap() as u32
        }

        let o_renderer = Some(TextRenderer::default());
        if(s_path_font_ttf.is_some()){
            println!("{:?}", s_path_font_ttf.clone().unwrap());
            let mut f = File::open(s_path_font_ttf.clone().unwrap()).expect("no file found");
            let metadata = std::fs::metadata(s_path_font_ttf.clone().unwrap()).expect("unable to read metadata");
            let mut buffer = vec![0; metadata.len() as usize];
            f.read(&mut buffer).expect("buffer overflow");
    
            let o_renderer = Some(TextRenderer::try_new_with_ttf_font_data(
                buffer
                // include_bytes!("./a-gentle-touch.ttf")
                // include_bytes!("./OpenSans-Regular.ttf")
            ).expect("font could not be load"));

            // Read the font data.
            // let font = include_bytes!("./KannaBoldW6.otf") as &[u8];
            // // Parse it into the font type.
            // let font = fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();
            // // Rasterize and get the layout metrics for the letter 'g' at 17px.
            // let (metrics, bitmap) = font.rasterize('g', 17.0);
            // println!("metrics {:?}", metrics);
            // println!("bitmap {:?}", bitmap);
        }
        let o_text_png_color = text_to_png::Color::new(
            (a_n_f64__color_rgb[0 as usize] * u8::MAX as f64) as u8,
            (a_n_f64__color_rgb[1 as usize] * u8::MAX as f64) as u8,
            (a_n_f64__color_rgb[2 as usize] * u8::MAX as f64) as u8,
        );
        let o_text_png = o_renderer.clone().unwrap().render_text_to_png_data(
            s_text.as_str(),
            n_font_size,
            o_text_png_color
        ).unwrap();


        let s_path_image_text = "o_text_png.png";
        let mut o_file = File::create(s_path_image_text).unwrap();
        let a: &[u8] = &o_text_png.data;
        o_file.write_all(a).unwrap();
        let mut o_image_text = ImageReader::open(s_path_image_text).unwrap().decode().unwrap();
        let n_pixels_per_char = o_image_text.width() as f32 / s_text.chars().count() as f32; 
        let n_chars_max_per_line = ((((n_o_image_output_n_scale_x as f64) * (n_max_width_normalized as f64))) as f32  / n_pixels_per_char) as usize; 
        // println!("n_pixels_per_char {:?}", n_pixels_per_char);
        // println!("n_chars_max_per_line {:?}", n_chars_max_per_line);
        // let mut a_s_breakpart = s_text.split(&[' ', '-'][..]) ;// split by whitespace ' ' or dash '-'
        let mut a_s_breakpart: Vec<String> = Vec::new();
        let mut s_breakpart = String::new();
        for (n_index, s_char) in s_text.chars().enumerate() {
            s_breakpart.push(s_char);
            if(s_break_on.contains(s_char)){
                a_s_breakpart.push(s_breakpart);
                s_breakpart = String::new();
            }
        }
        a_s_breakpart.push(s_breakpart);
        // println!("a_s_breakpart {:?}", a_s_breakpart);
        let mut n_len = 0; 
        let mut a_s_line : Vec<String> = Vec::new();
        let mut s_line = String::new();
        let mut n_count = 0;
        for s_breakpart in a_s_breakpart{
            n_len += s_breakpart.chars().count();
            let n_count_inc = (n_len as f32 / n_chars_max_per_line as f32) as usize;
            if( n_count_inc > n_count){
                n_count += n_count_inc-n_count;
                a_s_line.push(s_line);
                s_line = String::new();
            }
            s_line.push_str(&s_breakpart);
            // println!("s_breakpart {:?}", s_breakpart);
        }
        a_s_line.push(s_line);
        // println!("a_s_line {:?}", a_s_line);

        
        // Create a new ImgBuf with width: imgx and height: imgy
        let n_o_image_text_bg_n_scale_y = (o_image_text.height() * a_s_line.len() as u32) + n_padding_text_to_background as u32;
        let mut o_image_text_bg = image::ImageBuffer::new(
            ((n_o_image_output_n_scale_x as f64) * (n_max_width_normalized as f64)) as u32 + (n_padding_text_to_background*2) as u32, 
            n_o_image_text_bg_n_scale_y
        );

        // Iterate over the coordinates and pixels of the image
        for (n_x, n_y, o_pixel) in o_image_text_bg.enumerate_pixels_mut() {
            *o_pixel = image::Rgba([
                (a_n_f64__bgcolor_rgba[0 as usize] * (u8::MAX as f64)) as u8,
                (a_n_f64__bgcolor_rgba[1 as usize] * (u8::MAX as f64)) as u8,
                (a_n_f64__bgcolor_rgba[2 as usize] * (u8::MAX as f64)) as u8,
                (a_n_f64__bgcolor_rgba[3 as usize] * (u8::MAX as f64)) as u8,
            ]);
        }
   
        let mut o_image_text_with_bg = o_image_text_bg.clone();
        let n_overlay_translation_x = (n_padding_text_to_background as f32 /2.0) as i64;
        let n_overlay_translation_y = (n_padding_text_to_background as f32 /2.0) as i64;
        for (n_index, s_line) in a_s_line.iter().enumerate(){

            let o_text_png = o_renderer.clone().unwrap().render_text_to_png_data(
                s_line,
                n_font_size,
                o_text_png_color
            ).unwrap();
    
            // "o_text_png.png";
            let mut o_file = File::create(s_path_image_text.clone()).unwrap();
            let a: &[u8] = &o_text_png.data;
            o_file.write_all(a).unwrap();
            let mut o_image_text_line = ImageReader::open(s_path_image_text).unwrap().decode().unwrap();

            image::imageops::overlay(
                &mut o_image_text_with_bg,
                &o_image_text_line,
                n_overlay_translation_x,
                n_overlay_translation_y + (n_index as u32 * o_image_text.height()) as i64
            );
            println!("n_overlay_translation_y + (n_index as u32 * o_image_text.height()) as i64 {:?}", n_overlay_translation_y + (n_index as u32 * o_image_text.height()) as i64);
            
            // let s_path_image_text = vec![s_name_file_without_extension.clone(), String::from("_o_text"), String::from(n_index.to_string()), s_outpupt_file_extension.clone()].join("");
            // o_image_text_with_bg.save(vec![s_name_file_without_extension.clone(), String::from("_o_image_text_with_bg"), s_outpupt_file_extension.clone()].join("")).unwrap();

        }


        let mut n_translation_y = ((o_image_overlayed.height() as f32) - (o_image_text_with_bg.height() as f32) ) as i64; //defaultt value
        if(o_text["n_translation_y"] != serde_json::Value::Null){ 
            n_translation_y = o_text["n_translation_y"].as_i64().unwrap();
        }
        let mut n_translation_x = ((o_image_overlayed.width() as f32 / 2.0) - (o_image_text_with_bg.width() as f32/2.0) ) as i64; //defaultt value
        if(o_text["n_translation_x"] != serde_json::Value::Null){ 
            n_translation_x = o_text["n_translation_x"].as_i64().unwrap();
        }

        image::imageops::overlay(
            &mut o_image_overlayed_text,
            &o_image_text_with_bg,
            n_translation_x,
            n_translation_y
        );

        
        n_i_a_o_text+=1
    }
    o_image_overlayed_text.save(vec![s_name_file_without_extension.clone(), String::from("_overlayed_text"), s_outpupt_file_extension.clone()].join("")).unwrap();
    

}