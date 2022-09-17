use std::io;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    // Read the file.
    let f = File::open("/Users/sam/file.txt")?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    
    // Loop over and modify the data.
    let mut buffer_modified = Vec::new();
    for value in buffer {
        println!("BYTE: {}", value);
        if value == 'a' as u8 {
            let value_modified = 'z' as u8;
            buffer_modified.push(value_modified);
            println!("MODIFIED: {}", value_modified);
        } else {
            buffer_modified.push(value);
        }
    }
    
    // Add additional letter.
    buffer_modified.push('.' as u8);
    
    // Write the modified data.
    let mut f = File::create("/Users/sam/file-modified.txt")?;
    f.write_all(buffer_modified.as_slice())?;
    Ok(())
}