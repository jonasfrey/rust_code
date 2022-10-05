use std::io::Cursor;
use text_to_png::TextRenderer;
use image::io::Reader as ImageReader;
use image::GenericImageView;
use std::sync::Once;
use std::fs::File;
use std::io::Write;

use image::{ImageBuffer, Rgba};


fn main(){
    // scale == size , how many pixels width
    // translation == position, a fixed point somewhere
    // rotation == rotation

    let n_scale_x = 1920;
    let n_scale_y = 1080;
    let n_ratio: f32 = (n_scale_x as f32 / n_scale_y as f32);

    let s_path_image = "out-0.png";
    // let s_file_name =  s_path_image.split(".").collect::<Vec<&str>>().split_last().unwrap().join(".");

    let s_file_name = String::from(s_path_image.rsplit_once('.').unwrap().0);
    println!("f {:?}", s_file_name);

    // std::process::exit(1);

    let mut o_image = ImageReader::open(s_path_image).unwrap().decode().unwrap();

    let n_o_image_scale_x = o_image.width();
    let n_o_image_scale_y = o_image.height();
    
    let n_o_image_ratio: f32 = (n_o_image_scale_x as f32/ n_o_image_scale_y as f32);


    let mut n_crop_tranlsation_x = 0; 
    let mut n_crop_translation_y = 0;
    let mut n_crop_scale_x = 0;
    let mut n_crop_scale_y = 0;

    let b_ratio_equal =  n_o_image_ratio >= 1.0 && n_ratio >= 1.0;
    if(b_ratio_equal){
        n_crop_scale_x = (n_o_image_scale_x);
        n_crop_scale_y = (n_o_image_scale_y as f32 * (1.0/n_ratio)) as u32;
    }else{
        n_crop_scale_x = (n_o_image_scale_x as f32 * (1.0/n_ratio)) as u32;
        n_crop_scale_y = (n_o_image_scale_y as f32) as u32;
    }
    println!("n_crop_scale_x {:?}", n_crop_scale_x);
    println!("n_crop_scale_y {:?}", n_crop_scale_y);

    let mut o_image_cropped = image::DynamicImage::ImageRgba8(
        image::imageops::crop(
            &mut o_image,
            n_crop_tranlsation_x,
            n_crop_translation_y, 
            n_crop_scale_x,
            n_crop_scale_y,
            // image::imageops::FilterType::Nearest,
            // image::imageops::FilterType::Triangle,
            // image::imageops::FilterType::CatmullRom,
            // image::imageops::FilterType::Gaussian,
            // image::imageops::FilterType::Lanczos3,
        ).to_image()
    );
    o_image_cropped.save(vec![s_file_name.clone(), String::from("_cropped.jpg")].join("")).unwrap();


    let mut o_image_resized = image::DynamicImage::ImageRgba8(
        image::imageops::resize(
            &mut o_image_cropped,
            n_scale_x, 
            n_scale_y,
            image::imageops::FilterType::Nearest,
            // image::imageops::FilterType::Triangle,
            // image::imageops::FilterType::CatmullRom,
            // image::imageops::FilterType::Gaussian,
            // image::imageops::FilterType::Lanczos3,
        )
    );
    o_image_resized.save(vec![s_file_name.clone(), String::from("_resized.jpg")].join("")).unwrap();

    let mut o_image_blurred = image::DynamicImage::ImageRgba8(
        image::imageops::blur(
            &mut o_image_resized,
            100.0
        )
    );
    o_image_blurred.save(vec![s_file_name.clone(), String::from("_blurred.jpg")].join("")).unwrap();
    

    let mut o_image_overlayed = o_image_blurred.clone();
    let n_overlay_translation_x = ((n_scale_x as f32 /2.0) - (o_image.width() as f32 / 2.0) ) as i64;
    let n_overlay_translation_y = ((n_scale_y as f32 /2.0) - (o_image.height() as f32 / 2.0) ) as i64;
    println!("n_overlay_translation_x {:?}", n_overlay_translation_x);
    println!("n_overlay_translation_y {:?}", n_overlay_translation_y);
    image::imageops::overlay(
        &mut o_image_overlayed,
        &o_image,
        n_overlay_translation_x,
        n_overlay_translation_y
    );
    

    o_image_overlayed.save(vec![s_file_name.clone(), String::from("_overlayed.jpg")].join("")).unwrap();


    // put text 
    let s_text = String::from("a very very long lorem ipsum dolor sit amet text which never will end because there are ? so ! many: \"symbols\" and 'words', yes indeed! can you stil read it ???");
    let s_color = String::from("Red");
    let n_factor_pixels_per_character_average = 2.0;
    let n_size = ((n_scale_x as f32 / s_text.chars().count() as f32) * n_factor_pixels_per_character_average) as u32;

    let o_renderer = TextRenderer::try_new_with_ttf_font_data(include_bytes!("OpenSans-Regular.ttf"))
    .expect("Example font is definitely loadable");

    let o_text_png = o_renderer.render_text_to_png_data(
        s_text.as_str(),
        n_size,
        s_color.as_str(),
    ).unwrap();
    let s_path_image_text = "o_text_png.png";
    let mut o_file = File::create(s_path_image_text).unwrap();
    let a: &[u8] = &o_text_png.data;
    o_file.write_all(a).unwrap();

    let mut o_image_text = ImageReader::open(s_path_image_text).unwrap().decode().unwrap();
    let mut n_padding = 10;
    // let mut o_image_text_bg = ImageBuffer::<image::Rgba<u8>, Container>::new(o_image_text.width()+n_padding,o_image_text.height()+n_padding);
    let mut o_image_text_bg : image::RgbaImage = ImageBuffer::new(o_image_text.width()+n_padding,o_image_text.height()+n_padding);
    o_image_text_bg.fill(40);
    let mut o_image_text_with_bg = o_image_text_bg.clone();
    let n_overlay_translation_x = n_padding as i64;
    let n_overlay_translation_y = n_padding as i64;
    image::imageops::overlay(
        &mut o_image_text_with_bg,
        &o_image_text,
        n_overlay_translation_x,
        n_overlay_translation_y
    );
    let mut n_bottom_padding = 10;
    let mut o_image_overlayed_text = o_image_overlayed.clone();
    let n_overlay_translation_x = ((o_image_overlayed.width() as f32 /2.0) - (o_image_text_with_bg.width() as f32 / 2.0) ) as i64;
    let n_overlay_translation_y = ((o_image_overlayed.height() as f32) - n_bottom_padding as f32 - (o_image_text_with_bg.height() as f32) ) as i64;
    image::imageops::overlay(
        &mut o_image_overlayed_text,
        &o_image_text_with_bg,
        n_overlay_translation_x,
        n_overlay_translation_y
    );

    o_image_overlayed_text.save(vec![s_file_name.clone(), String::from("_overlayed_text.jpg")].join("")).unwrap();

}