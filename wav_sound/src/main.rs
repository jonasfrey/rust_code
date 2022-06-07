use wav_sound::f_o_wav;
use wav_sound::f_add_sound;
use wav_sound::f_save_o_wav;

//ffmpeg -i 'img-%03d.jpeg' -r 10 out.mkv
fn main() -> std::io::Result<()> {

    // get the struct
    let mut o_wav = f_o_wav();
    o_wav = f_add_sound(
        o_wav,// o_wav struct
        440, //frequency
        String::from("square"), //wave type 'sawtooth' , 'sine' 
        300, // milliseconds
    );
    o_wav = f_add_sound(
        o_wav,// o_wav struct
        440, //frequency
        String::from("sine"), //wave type 'sawtooth' , 'sine' 
        300, // milliseconds
    );
    o_wav = f_add_sound(
        o_wav,// o_wav struct
        440, //frequency
        String::from("sawtooth"), //wave type 'sawtooth' , 'sine' 
        300, // milliseconds
    );
    o_wav = f_add_sound(
        o_wav,// o_wav struct
        440, //frequency
        String::from("triangle"), //wave type 'sawtooth' , 'sine' 
        1000, // milliseconds
    );
    f_save_o_wav(
        o_wav,
        String::from("square_test.wav")
    );

    //     // println!("{}", n_i);


    Ok(())
}
