
struct O_user {
    b_actve: bool,
    s_name: String,
    s_email: String,
    n_age_years: u8,
}

fn main() {
    println!("Hello, world!");

    let o_user = O_user {
        b_actve: true,
        s_name: String::from("some one"),
        s_email: String::from("someone@example.com"),
        n_age_years: 28,
    };


    let o_file = File::open("foo.txt")?;
    let mut o_buf_reader = BufReader::new(o_file);
    let mut contents = String::new();
    o_buf_reader.read_to_string(&mut contents)?;
    assert_eq!(contents, "Hello, world!");
    Ok(())

}