use std::io::Write;                                                                                                                                                                  
use std::io::prelude::*;                                                                                                                                                             
use std::fs::File;                                                                                                                                                                   
                                                                                                                                                                                     
fn main() {                                                                                                                                                                          
    let data = vec![1., 2., 3.];                                                                                                                                                     
    let mut f = File::create("output.vtk").expect("Unable to create file");                                                                                                          
    for i in &data{                                                                                                                                                                  
        f.write_all((*i)).expect("Unable to write data");                                                                                                                            
    }                                                                                                                                                                                
}