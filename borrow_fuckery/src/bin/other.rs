// run with 
//$ cargo run --bin other

struct O_person{
    s_name: String,
    a_n_index_a_o_person: Vec<usize>
}

fn f_with_index(){

    let mut a_o_person :Vec<O_person> = Vec::new();

    a_o_person.push(
        O_person{
            s_name:String::from("hans"),
            a_n_index_a_o_person: vec![]
        }
    );

    a_o_person.push(
        O_person{
            s_name:String::from("peter"), 
            a_n_index_a_o_person: vec![]
        }
    );

    a_o_person.push(
        O_person{
            s_name:String::from("klaus"),
            a_n_index_a_o_person: vec![]
        }
    );

    a_o_person[0].a_n_index_a_o_person.push(1 as usize);
    a_o_person[0].a_n_index_a_o_person.push(2 as usize);

    for o_person in a_o_person.iter(){
        println!("-----");
        println!("person: ${:?}", o_person.s_name);
        for n_index_a_o_person in o_person.a_n_index_a_o_person.iter(){
            println!("linked person: ${:?}", a_o_person[*n_index_a_o_person as usize].s_name);
        }
    }
}
fn main(){
    f_with_index();
}