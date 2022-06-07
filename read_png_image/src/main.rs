use std::fs;
use std::io::Cursor;
use image::io::Reader as ImageReader;
use image::GenericImageView;
use serde_json::json;


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main()  -> Result<(), Box<dyn std::error::Error>>{
    let paths = fs::read_dir("./images/").unwrap();


    for path in paths {
        let s_test = path.unwrap();

        let s_wtf = s_test.path().display().to_string();

        let mut a_s_part_path: Vec<&str>= s_wtf.split("/").collect();
        let mut a_s_part_filename: Vec<&str> = a_s_part_path.last().unwrap().split(".").collect();
        a_s_part_filename.pop();
        let s_filename = a_s_part_filename.join(".");

        // println!("s_filename: {:?}", s_filename);
        // println!("s_part: {:?}", a_s_part.last());
        // println!("s_part: {:?}", a_s_part.last());
        
        // let s_path = path.unwrap().path().display();
        // println!("Name: {:?}", s_path);
        // println!("Name: {:?}", s_path.to_string());
        // let img = ImageReader::open(s_wtf).decode();
        
        let o_img_rgb = ImageReader::open(s_wtf)?.decode()?;
        // let a_a_b : Vec<Vec<bool>> = vec!(vec!(false, o_img_rgb.dimensions().0), o_img_rgb.dimensions().1);
        let a_a_b : Vec<Vec<bool>> = Vec::new();
        let mut o_json_pixel = json!({
            // quotes on keys are optional
            "a_a_b":a_a_b,
            "s_letter": s_filename,
            "b_uppercase": true, 
        });

        println!("o_json_pixel {:?}", o_json_pixel["a_a_b"]);
        o_json_pixel["a_a_b"][0] = serde_json::Value::Array(Vec::new());
        println!("o_json_pixel {:?}", o_json_pixel);

        // let a_o_pixel = o_img_rgb.pixels();
        // let mut a_b = vec!();  
        // for o_pixel in a_o_pixel{
        //     let n_y = o_pixel.1;
        //     if(n_y == 0){
        //         a_b = vec!();  
        //         o_json_pixel["a_a_b"].push(a_b)
        //     }
        //     let a_rgba_data = o_pixel.2;
        //     if(a_rgba_data[3] == 0){
        //         a_b.push(true);
        //     }else{
        //         a_b.push(false);
        //     }

        //     println!("o_pixel {:?}", o_pixel);
        // }
        println!("o_json_pixel {:?}", o_json_pixel);
        // println!("pixels {:?}", o_img_rgb.pixels());
        // let o_img_gray = o_img_rgb.to_luma8();

        // // print_type_of(&o_img_gray);

        // let a_data = o_img_gray.pixels();
        // println!("{:?}", a_data);
        // // println!("{:?}", img.dimensions());
    
        // image::save_buffer(
        //     "gray.png",
        //     &o_img_gray,
        //     o_img_rgb.dimensions().0,
        //     o_img_rgb.dimensions().1,
        //     // color::ColorType(0),
        //     // image::GrayImage(8)
        //     // image::Gray(8)
        //     image::color::ColorType::Gray
        // );
    }   
    // let x = fs::read_dir("./images/").unwrap();
    Ok(())
}