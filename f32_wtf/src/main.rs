struct O_person {
    n_id: u32, 
    s_name: String,
    b_male: bool,
    n_age: f32,
}

fn main() {
    // println!("Hello, world!");

    let n: f32 = 33.3300018; 
    println!("n {:?}", n);
    println!("a_nu8 {:?}", n.to_be_bytes());

    // let o_pers = O_person{
    //     n_id: 10,
    //     s_name: String::from("hans"),
    //     b_male: true, 
    //     n_age: 33.3300018 
    // };
    // println!("o_pers.n_age {:?}", o_pers.n_age);
    // let n = 33.3300018; 
    // println!("n {:?}", n);
}
