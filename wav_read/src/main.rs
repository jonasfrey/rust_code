use std::fs::File;
use std::path::Path;

fn main() {

    let mut o_file_pdub8k = File::open(Path::new("./premium_deluxe_ultra_brachial_8k_pink.wav")).unwrap();
    let (o_header_pdub8k, o_data_pdub8k) = wav::read(&mut o_file_pdub8k).unwrap();

    let mut o_file_dream = File::open(Path::new("./Dream_Test_24_bit_-18_dbfs.wav")).unwrap();
    let (o_header_dream, o_data_dream) = wav::read(&mut o_file_dream).unwrap();


    // println!("{:?}", o_data_dream);
    // println!("{:?}", o_header_dream);

    // let mut a_n_i = o_data_dream.as_sixteen().unwrap().clone();
    let mut a_n_i_dream = o_data_dream.as_twenty_four().unwrap().clone();
    let mut a_n_i_pdub8k = o_data_pdub8k.as_twenty_four().unwrap().clone();
    let mut n_i = 0; 

    while(n_i < a_n_i_dream.len()){
        // a_n_i[n_i] += (((n_i as f32*0.2) as f32).sin()*6500.0)as i16;
        // a_n_i[n_i] += (((n_i as f32*0.2) as f32).sin()*65000.0)as i32;
        a_n_i_dream[n_i] += a_n_i_pdub8k[n_i%a_n_i_pdub8k.len()];
        n_i+=1;
    }

    // let a_n_i_2 = a_n_i.clone();

    let mut out_file = File::create(Path::new("./out_premium_deluxe_ultra_brachial_8k_pink.wav")).unwrap();
    // wav::write(o_header_dream, &wav::bit_depth::BitDepth::Sixteen(a_n_i.clone()), &mut out_file).unwrap();
    wav::write(o_header_dream, &wav::bit_depth::BitDepth::TwentyFour(a_n_i_dream.clone()), &mut out_file).unwrap();


}
