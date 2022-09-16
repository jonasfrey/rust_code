use std::io::Read;
use std::mem;
use std::slice;

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
struct Configuration {
    item1: u8,
    item2: u16,
    item3: i32,
    item4: [char; 8],
}

const CONFIG_DATA: &[u8] = &[
    0xfd, // u8
    0xb4, 0x50, // u16
    0x45, 0xcd, 0x3c, 0x15, // i32
    0x71, 0x3c, 0x87, 0xff, // char
    0xe8, 0x5d, 0x20, 0xe7, // char
    0x5f, 0x38, 0x05, 0x4a, // char
    0xc4, 0x58, 0x8f, 0xdc, // char
    0x67, 0x1d, 0xb4, 0x64, // char
    0xf2, 0xc5, 0x2c, 0x15, // char
    0xd8, 0x9a, 0xae, 0x23, // char
    0x7d, 0xce, 0x4b, 0xeb, // char
];

fn main() {
    let mut buffer = CONFIG_DATA;

    let mut config: Configuration = unsafe { mem::zeroed() };

    let config_size = mem::size_of::<Configuration>();
    unsafe {
        let config_slice = slice::from_raw_parts_mut(&mut config as *mut _ as *mut u8, config_size);
        // `read_exact()` comes from `Read` impl for `&[u8]`
        buffer.read_exact(config_slice).unwrap();
    }

    println!("Read structure: {:#?}", config);
}