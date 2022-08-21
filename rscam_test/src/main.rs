// use std::fs::File;
// use std::io::prelude::*;
// fn main() {
    
//     use rscam::{Camera, Config};

//     let mut camera = Camera::new("/dev/video0").unwrap();
    
//     camera.start(&Config {
//         interval: (1, 30),      // 30 fps.
//         resolution: (1280, 720),
//         format: b"MJPG",
//         ..Default::default()
//     }).unwrap();
    
//     for i in 0..10 {
//         let frame = camera.capture().unwrap();
//         let mut file = File::create(&format!("frame-{}.jpg", i)).unwrap();
//         file.write_all(&frame[..]).unwrap();
//     }
// }

// use v4l::buffer::Type;
// use v4l::io::mmap::Stream;
// use v4l::io::traits::CaptureStream;
// use v4l::video::Capture;
// use v4l::Device;
// use v4l::FourCC;
// fn main(){

//     let mut dev = Device::new(0).expect("Failed to open device");

//     let mut stream =
//     Stream::with_buffers(&mut dev, Type::VideoCapture, 4).expect("Failed to create buffer stream");

//     loop {
//         let (buf, meta) = stream.next().unwrap();
//         println!(
//             "Buffer size: {}, seq: {}, timestamp: {}",
//         buf.len(),
//         meta.sequence,
//         meta.timestamp
//     );
//     }
// }

use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let mut f = File::open("/dev/video1")?;
    let mut buffer = [0; 10];

    // read up to 10 bytes
    f.read(&mut buffer)?;
    println!("{:?}",buffer);
    
    //  let mut buffer = Vec::new();
    // // read the whole file
    // f.read_to_end(&mut buffer)?;

    // // read into a String, so that you don't need to do the conversion.
    // let mut buffer = String::new();
    // f.read_to_string(&mut buffer)?;

    // // and more! See the other methods for more details.
    Ok(())
}


// use std::fs::File;
// use std::io::Read;
// fn main(){
//     let mut f = File::open("/dev/video0");
//     // let mut s = String::new();
//     let mut buffer = [0; 10];
//     f.read_to_string(&mut buffer);
//     println!("s:'{}'", buffer);
// }