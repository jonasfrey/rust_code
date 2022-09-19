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
        let a_a_b : Vec<Vec<bool>> = vec!(vec!(false; o_img_rgb.dimensions().0 as usize); o_img_rgb.dimensions().1 as usize);

        // let a_a_b : Vec<Vec<bool>> = Vec::new();
        let mut o_json_pixel = json!({
            // quotes on keys are optional
            "a_a_b":a_a_b,
            "s_letter": s_filename,
            "b_uppercase": true, 
        });
        let n_index_rgba = 0;
        let n_index_pixel = 0;
        let n_x = 0; 
        let n_y = 0;
         
        // while(n_index_rgba <  o_img_rgb.to_luma8() ){
        //     n_index_pixel = (n_index_rgba as f32 / 4.0) as u32;
        //     if(n_index_rgba % 4== 0){

        //         n_y = n_in
        //         n_x = n_x+1 % o_img_rgb.width();
        //     }
        //     n_index_rgba++;
        // }
        println!("o_img_rgb.pixels(){:?}",o_img_rgb.pixels());
        println!("o_img_rgb{:?}",o_img_rgb); 
        for o_pixel in o_img_rgb.pixels(){
            println!("o_pixel in o_img_rgb.pixels(){:?}", o_pixel);
            if(o_pixel.2[3] == 0){
                o_json_pixel["a_a_b"][o_pixel.0 as usize][o_pixel.1 as usize] = serde_json::Value::Bool(true);
            }
        }
        std::fs::write(
            s_filename+".json",
            serde_json::to_string_pretty(&o_json_pixel).unwrap(),
        )
        .unwrap();
        // println!("o_json_pixel {:?}", o_json_pixel["a_a_b"]);
        // o_json_pixel["a_a_b"][0] = serde_json::Value::Array(Vec::new());
        // println!("o_json_pixel {:?}", o_json_pixel);

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