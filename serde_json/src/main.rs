
use serde::{Serialize, Deserialize};
 
#[derive(Serialize, Deserialize, Debug)]
struct Person {
    person_id: i32,
    person_name: String,
    person_name_fixed_size: [u8; 20]
}

fn main() {

    let person_name = "hans";
    let mut person_name_padded = [0; 20];
    person_name_padded[..person_name.len()].copy_from_slice(person_name.as_bytes()); 

    let person = Person {
        person_id: 100,
        person_name: (&person_name).to_string(),
        person_name_fixed_size: person_name_padded,
        // person_name: person_name_padded
    };

    println!("person {:?}", serde_json::to_string(&person).unwrap());

    println!("Hello, world!");
}
