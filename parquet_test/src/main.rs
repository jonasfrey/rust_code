use parquet::file::reader::{FileReader, SerializedFileReader};
use std::{fs::File, path::Path};

fn f_print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


fn f_microlensing_theoretical(
    n_d_time: f64,
    n_umin_amplitude: f64, 
    n_tE_duration_event: f64, 
    n_t_max: f64, 
)->f64{

    let n_static = -2.5;
    // # FUNCTIONS
// def f_ML_theo(n_d, n_umin, n_tE, n_I, n_t_max): #theoretische ML-Funktion, input = time-array, output = mag-array
    // #umin => between 0 and 1 - the smaller, the bigger the amplitude
    // #tE => duration of Event - the bigger, the wider the curve
    // #I => intensity I = (light intensity)/Area of star without amplification
    // #t_max => time when amplification A of I reaches maximum a
    // n_u = np.sqrt(n_umin**2 + ((n_d-n_t_max)/n_tE)**2)

    let n_u = (((n_umin_amplitude as i64).pow(2) + ((( n_d_time - n_t_max)/n_tE_duration_event) as i64).pow(2)) as f64).sqrt();
    if(n_u * ((n_u as i64).pow(2)+4)as f64) == 0.0{
        return -10.0 // if umin = 0 and d = t0, amplitude theoretically becomes infinite 
    }else{
        let n_A = n_tE_duration_event * (((n_u as i64).pow(2) + 2) as f64 / (n_u*(((n_u as i64).pow(2)+4) as f64).sqrt()) as f64);
        let n_M = n_static*(n_A.log10());
        return n_M;
        // let n_A = n_I*((n_u**2 + 2) / (n_u*np.sqrt(n_u**2 + 4))) 
        // let n_M = -2.5*np.log10(n_A) # conversion to magnitude
    }
}

fn f_a_rect_read_and_optional_write(
    a_n_u8__image: &mut Vec<u8>,
    n_image_scale_x : u32,
    n_image_scale_y : u32,
    n_image_channels: u32,
    n_rect_translation_x : u32,
    n_rect_translation_y : u32,
    n_rect_scale_x : u32,
    n_rect_scale_y : u32,
    a_n_u8__color : Option<&Vec<u8>>
) -> Vec<u8> {

    let mut a_vec: Vec<u8> = Vec::new();

    let mut n_rgba_value = 0;
    let mut n_pixel_index = 0;
    let mut n_x = 0;
    let mut n_y = 0;
    let mut n_channel = 0;
    
    let n_index_max = a_n_u8__image.len()-1;

    while(n_y < n_rect_scale_y){
        n_x = 0;
        while(n_x < n_rect_scale_x){
            let n_index_pixel = 
            ((n_rect_translation_y + n_y) * n_image_scale_x * n_image_channels) +
            ((n_rect_translation_x + n_x) * n_image_channels);
            
            n_channel = 0;
            while(n_channel < n_image_channels){
                let n_index = (n_index_pixel+n_channel);
                if(n_index < 0 || n_index > n_index_max.try_into().unwrap()){
                    n_channel+=1;
                    continue;
                }
                if(a_n_u8__color.is_none() == false){
                    a_n_u8__image[n_index as usize] = a_n_u8__color.unwrap()[n_channel as usize];
                }
                a_vec.push(a_n_u8__image[n_index as usize]);
                n_channel+=1;
            }
            // println!("x|y {:?}|{:?}", n_x, n_y);
            n_x+=1;
        }
        n_y+=1;
        // println!("x|y {:?}|{:?}", n_x, n_y);
    }

    return a_vec

}

fn f_animate_microlensing_theoretical(){

    let n_scale_x = 1920.0;
    let n_scale_y = 1080.0;
    let n_pixel_channels = 4;

    let mut a_n_u8__image: Vec<u8> = 
    vec![
        0;
        (
            n_scale_x *  
            n_scale_y * 
            (n_pixel_channels as f64)
        ) as usize
    ];

    let s_name = String::from("microlensing curve theoretical, press space and move mouse!");
    let o_window = create_window(s_name, Default::default()).unwrap();
    let mut o_image = ImageView::new(
        ImageInfo::rgba8(
            n_scale_x as u32,  
            n_scale_y as u32
        ),
        &a_n_u8__image
    );
    let a_n_u8__color = vec![122,255,255,255];
    
    while(true){


        f_a_rect_read_and_optional_write(
             &mut a_n_u8__image,
             n_scale_x as u32,
             n_scale_y as u32,
             n_pixel_channels,
             0,
             0,
             100,
             100,
             Some(&a_n_u8__color)
        );

        o_image = ImageView::new(
            ImageInfo::rgba8(
                n_scale_x as u32,  
                n_scale_y as u32
            ),
            &a_n_u8__image
        );
        
        o_window.set_image(s_name.clone(), o_image).unwrap();
    }
}
fn f_exmaple_read_single_file(){

    let s_file_name = String::from("ztf_000468_zi_c16_q4_dr7.parquet");
    let o_path = Path::new(&s_file_name);
    
    if let Ok(file) = File::open(&o_path) {
        let reader = SerializedFileReader::new(file).unwrap();
    
        let parquet_metadata = reader.metadata();
        // assert_eq!(parquet_metadata.num_row_groups(), 1);
        // println!("parquet_metadata {:?}", parquet_metadata);
        let row_group_reader = reader.get_row_group(0).unwrap();
        // assert_eq!(row_group_reader.num_columns(), 1);
    
        println!("row_group_reader.num_columns() {:?}", row_group_reader.num_columns());
    
    }
}
fn f_example_read_multiple_files(){

    let paths = vec![
        "ztf_000468_zi_c16_q4_dr7.parquet",
    ];
    // Create a reader for each file and flat map rows
    let rows = paths.iter()
        .map(|p| SerializedFileReader::try_from(*p).unwrap())
        .flat_map(|r| r.into_iter());

    for row in rows {
        f_print_type_of(&row);

        for (n_index, (s_prop_name, value)) in row.get_column_iter().enumerate() {

            if(s_prop_name == "catflags"){
                if(value >= 1){
                    //....
                }
            }
            println!("n_index: {:?}", n_index);
            println!("s_prop_name: {:?}", s_prop_name);
            println!("value: {:?}", value);
        }
        // println!("{}", row);
        // println!("{}", row["objectid"]);
    }
}

fn f_example_read_and_write(){
    
    let paths = vec![
        "ztf_000468_zi_c16_q4_dr7.parquet",
    ];
    // Create a reader for each file and flat map rows
    let rows = paths.iter()
        .map(|p| SerializedFileReader::try_from(*p).unwrap())
        .flat_map(|r| r.into_iter());

    for row in rows {
        println!("row: {:?}", row);
        // 
        // f_print_type_of(&row);
// 
        for (n_index, (s_prop_name, value)) in row.get_column_iter().enumerate() {

            // s_prop_name "nepochs"
            // value 1234
            // n_index 6

            println!("n_index: {:?}", n_index);
            println!("s_prop_name: {:?}", s_prop_name);
            println!("value: {:?}", value);
            //
        }
        // println!("{}", row);
        // println!("{}", row["objectid"]);
    }
}
fn main() {

    // f_exmaple_read_single_file();
    f_example_read_multiple_files();

}
