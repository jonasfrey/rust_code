struct O_point_2d{
    n_x: u32, 
    n_y: u32 
}

fn f_multiple_mutable_references_in_different_scopes(){

    let mut o_point_2d = O_point_2d{
        n_x:123,
        n_y:2
    };
    {
        let o_point_2d_ref = &mut o_point_2d;
        f_change_n_x(o_point_2d_ref);
    }
    {
        let o_point_2d_ref2 = &mut o_point_2d;
        f_change_n_x(o_point_2d_ref2);
    }

}
fn f_multiple_immutable_references_in_same_scope(){
    let mut o_point_2d = O_point_2d{
        n_x:123,
        n_y:2
    };
    // we can have multiple immutable references to the same object in 
    let o_point_2d_ref1 = &o_point_2d;
    let o_point_2d_ref2 = &o_point_2d;
    f_print_o_point_2d(o_point_2d_ref1);
    f_print_o_point_2d(o_point_2d_ref2);
}

fn f_print_o_point_2d(
    o_point_2d: &O_point_2d
){
    println!("x|y:{:?}|{:?}", o_point_2d.n_x, o_point_2d.n_y);
}


// fn f_multiple_mutable_references_in_same_scope(){

//     let mut o_point_2d = O_point_2d{
//         n_x:123,
//         n_y:2
//     };
//     let o_point_2d_ref = &mut o_point_2d;
//     // we cannot have multiple mutable references to the same object in one scope
//     let o_point_2d_ref2 = &mut o_point_2d; // < 'cannot borrow `o_point_2d` as mutable more than once at a time'
//     f_change_n_x(&mut o_point_2d_ref);
//     f_change_n_x(&mut o_point_2d_ref2);

// }
fn f_change_n_x(
    o_point_2d: &mut O_point_2d
){
    o_point_2d.n_x = 0;

}
fn main() {
    println!("Hello, world!");
}
