#[derive(PartialEq, PartialOrd)]
struct O_person{
    n_id: u8, 
    s_name : String
}

fn f_change_values_which_have_been_pushed_to_vector(){

    
    let mut n_u8 = 10;
    let mut a_n_u8: Vec<u8> = Vec::new();
    a_n_u8.push(n_u8);

    n_u8 = 20;

    println!("n_u8 {}", n_u8);

    // not working :(
    // let mut o_person = O_person{
    //     n_id: 10, 
    //     s_name: String::from("hans")
    // };

    // let mut a_o_person: Vec<O_person> = Vec::new();

    // a_o_person.push(o_person);

    // o_person.n_id = 20;

    // not working :(
    // let mut o_person = O_person{
    //     n_id: 10, 
    //     s_name: String::from("hans")
    // };

    // let mut a_o_person_refs: Vec<&O_person> = Vec::new();

    // a_o_person_refs.push(&o_person);

    // println!("n_id {:?}", o_person.n_id);
    // o_person.n_id = 20;
    // println!("n_id {:?}", o_person.n_id);   
    // f_update_in_vector(&mut a_o_person_refs);
    // println!("n_id {:?}", o_person.n_id);   


    // working but... 
    // let mut o_person = O_person{
    //     n_id: 10, 
    //     s_name: String::from("hans")
    // };
    // let mut a_o_person: Vec<O_person> = Vec::new();
    // a_o_person.push(o_person);
    // // let n_index_o_person: usize = a_o_person.iter().position(|&o| o == o_person).unwrap();
    // let n_index_o_person: usize  = a_o_person.len() -1;
    
    // println!("n_id {:?}", a_o_person[n_index_o_person].n_id);

    // a_o_person[n_index_o_person].n_id = 20;

    // println!("n_id {:?}", a_o_person[n_index_o_person].n_id);   
    
    // f_update_in_vector(&mut a_o_person, n_index_o_person);
    
    // println!("n_id {:?}", a_o_person[n_index_o_person].n_id);   




    let mut o_person = O_person{
        n_id: 10, 
        s_name: String::from("hans")
    };
    let mut a_o_person: Vec<O_person> = Vec::new();
    a_o_person.push(o_person);
    let n_index_o_person: usize = a_o_person.len() -1;
    let mut o_person_ref = &mut a_o_person[n_index_o_person];
    
    println!("n_id {:?}", o_person_ref.n_id);

    o_person_ref.n_id = 20;

    println!("n_id {:?}", o_person_ref.n_id);   
    
    f_update_in_vector_ref(o_person_ref);

    println!("n_id {:?}", o_person_ref.n_id);   




}
fn f_update_in_vector_ref(
    o_person: &mut O_person
){
    o_person.n_id = 34;
}
fn f_update_in_vector(
    a_o_person: &mut Vec<O_person>, 
    n_index: usize
){
    a_o_person[n_index].n_id = 34;
}

fn f_n_u8_vector_as_u64(a_u8: &[u8], b_big: bool)-> u64{

    let mut n_u64 = 0;

    let mut n_i = 0; 
    let mut n_i_used = 0;
    while(n_i < 8){
        if(b_big){
            n_i_used = n_i;
        }else{
            n_i_used = (8-1)-n_i;
        }
        n_u64 = n_u64 | u64::from(a_u8[n_i]) << (8 * n_i_used);

        n_i+=1;
    }

    return n_u64;
}
fn f_interpret_bytes_as_number(){
    // interpret the first 8 bytes of this vector as a u64 number
    
    let a : Vec<u8> = vec![10,10,0,0,0,0,0,0,0,0,0,0]; 
    
    let mut a_first_8_bytes : [u8; 8] = [0;8];
    a_first_8_bytes.clone_from_slice(&a[0..8]); 
    // let n :u64 = a_first_8_bytes;
    
    let n_little_endian = u64::from_be_bytes(a_first_8_bytes);
    let n_big_endian = u64::from_le_bytes(a_first_8_bytes);
    // u64::from_be_bytes
    
    println!("n little endian: {}", n_little_endian);
    println!("n big endian: {}", n_big_endian);
    
    let n = f_n_u8_vector_as_u64(&a[0..8], true);
    println!("n: {}", n);
    
    let n_little_endian = f_n_u8_vector_as_u64(&a[0..8], false);
    println!("n_little_endian: {}", n_little_endian);

}
fn main() {

    f_change_values_which_have_been_pushed_to_vector();
}
