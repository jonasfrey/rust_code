
fn f_simple_works(){
    let mut a_n = vec![0,1,2];

    a_n.push(a_n[2]);
 
}
struct O_test<'a>{
    a_o_test: Vec<&'a O_test<'a>>
}


fn f_ok_fuck_it_im_going_unsafe(){

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
    unsafe{
        a_o_test[0].a_o_test.push(&a_o_test[0]);
    }

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
    a_o_test[0].a_o_test.push(&a_o_test[0]);
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

    a_o_person[0].a_o_person_linked.push(&a_o_person[1]); // 
                                                        // std::vec::Vec<O_person>
                                                        // let mut a_o_person :Vec<O_person> = Vec::new();
                                                        // cannot borrow `a_o_person` as immutable because it is also borrowed as mutable
// 
                                                        // immutable borrow occurs hererustc(E0502)
                                                        // main.rs(54, 5): mutable borrow occurs here
                                                        // main.rs(54, 37): mutable borrow later used by call
                                                        // main.rs(54, 43): immutable borrow occurs here
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

    f_ok_fuck_it_im_going_unsafe();

}
