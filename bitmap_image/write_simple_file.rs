use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    {
        let mut file = File::create("test")?;
        // Write a slice of bytes to the file
        const N_I: usize = 128;
        let mut n_start = 0;
        let mut a_nums:[u8; N_I ] = [0;128];
        while n_start < N_I {
            a_nums[n_start] = n_start as u8; 
            n_start+=1;
        }
        // file.write_all(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15])?;
        file.write_all(
            &a_nums
            // [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        )?;
    }

    {
        let mut file = File::open("test")?;
        // read the same file back into a Vec of bytes
        let mut buffer = Vec::<u8>::new();
        file.read_to_end(&mut buffer)?;
        println!("{:?}", buffer);
    }

    Ok(())
}