
fn f_simple_works(){
    let mut a_n = vec![0,1,2];

    a_n.push(a_n[2]);
 
}
struct O_test{
    a_o_test: Vec<O_test>
}
fn f_not_working(){

    let mut a_o_test :Vec<O_test> = Vec::new();

    a_o_test.push(
        O_test{
            a_o_test: vec![]
        }
    );
    a_o_test.push(
        O_test{
            a_o_test: vec![]
        }
    );
    a_o_test.push(
        O_test{
            a_o_test: vec![]
        }
    );
    a_o_test[0].a_o_test.push(a_o_test[0]);
}

struct O_person<'a>{
    s_name: String,
    a_o_person_linked: Vec<&'a O_person<'a>>
}
fn f_not_working_too(){

    let mut a_o_person :Vec<O_person> = Vec::new();

    a_o_person.push(
        O_person{
            s_name: String::from("hans"),
            a_o_person_linked: vec![]
        }
    );
    a_o_person.push(
        O_person{
            s_name: String::from("peter"),
            a_o_person_linked: vec![]
        }
    );

    a_o_person[0].a_o_person.push(&a_o_person[1]);
}

fn f_also_not_working(){

    let mut a_o_person :Vec<O_person> = Vec::new();

    a_o_person.push(
        O_person{
            s_name: String::from("hans"),
            a_o_person_linked: vec![]
        }
    );
    let o_person = O_person{
        s_name: String::from("peter"),
        a_o_person_linked: vec![]
    };
    a_o_person.push(
        o_person
    );

    a_o_person[0].a_o_person_linked.push(&o_person);
    
}
fn main() {

    f_not_working();

}
