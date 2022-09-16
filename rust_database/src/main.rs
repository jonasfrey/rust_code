use std::io::Write;                                                                                                                                                                  
use std::io::prelude::*;                                                                                                                                                             
use std::fs::File;                                                                                                                                                                   
struct O_user {
    b_actve: bool,
    s_name: String,
    s_email: String,
    n_age_years: u8,
}
                                                                                                                                                                        
fn main() {                                                                                                                                                                          
    let o_user = O_user {
        b_actve: true,
        s_name: String::from("some one"),
        s_email: String::from("someone@example.com"),
        n_age_years: 28,
    };

    
    let data = vec![o_user,o_user];                                                                                                                                                     
    let mut f = File::create("output.vtk").expect("Unable to create file");                                                                                                          
    f.write_all(&data).expect("Unable to write data");                                                                                                                            
                                                                                                                                                                            
}