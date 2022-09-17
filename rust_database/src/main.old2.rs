use std::io::Write;                                                                                                                                                                  
use std::io::prelude::*;                                                                                                                                                             
use std::fs::File;                                                                                                                                                                   
struct O_user {
    n_id: u32,
    b_actve: bool,
    s_name: String,
    s_email: String,
}
                                                                                                                                                                        
fn main() {                                                                                                                                                                          
    let o_user = O_user {
        n_id: 12341234,
        b_actve: true,
        s_name: String::from("some one"),
        s_email: String::from("someone@example.com"),
    };

    
    let data = vec![o_user,o_user];                                                                                                                                                     
    let mut f = File::create("output.vtk").expect("Unable to create file");                                                                                                          
    f.write_all(&data).expect("Unable to write data");                                                                                                                            
                                                                                                                                                                            
}