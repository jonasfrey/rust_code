fn main() {
    // 
    // struct O_user{
    //     s_email: String, 
    //     s_name_pre: String,
    //     s_name_last: String,
    // }

    let s = String::from("hello");

    // println!("s[0] {}", s[0]);
    // println!("s[0] {}", s.as_bytes()[10]);



    let a_nu8 = vec![97,98,99,100,0,0,101,102,103];
    let s_with_zero_byte = String::from_utf8(a_nu8);

    println!("str {:?}", s_with_zero_byte);
}