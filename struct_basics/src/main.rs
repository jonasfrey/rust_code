use std::option;

struct O_property_static_sized {
    s_name: String,
    s_type: String,
    n_bits: u16, 
}
struct O_model {
    s_name: String, 
    a_o_property_static_sized: Vec<O_property_static_sized>
}
struct O_piexl {
    n_red: u8,
    n_green: u8,
    n_blue: u8,
    n_alpha: u8,
}
fn f_s_value_from_o_model(
    o_model: O_model,
    s_property_name: String, 
    a_n_byte: Vec<u8> 
)-> Some{
    let mut n_index_a_o_property_static_sized = 0;
    let mut n_index_a_n_byte = 0;
    let mut a_s_property_name = vec![];
    let mut a_value = vec![];
    let mut n_index_a_n_byte_addition = 1;
    let mut n_bit_index = 0;
    let mut n_byte_index = 0;

    let mut n_bytes_required = 0;
    for( o_property_static_sized in o_model.a_o_property_static_sized){
        
        n_bytes_required = (((n_bit_index + o_property_static_sized.n_bits) as f32 /8.0) as u8)+1

        let mut a_n_byte__value = vec![];
        while(n_byte_index < n_byte_index + n_bytes_required){
            let mut n_u8_byte = 0;
            

            n_byte_index+=1;
        }
        a_value.push(a_n_byte__value);
        n_bit_index += o_property_static_sized.n_bits;
    }

}
fn f_manual(){

    let o_pixel = O_model{
        s_name: "O_pixel", 
        vec![
            O_property_static_sized{
                s_name: "n_red", 
                s_type: "number",
                n_bits: 8,
            }, 
            O_property_static_sized{
                s_name: "n_green", 
                s_type: "number",
                n_bits: 8,
            },
            O_property_static_sized{
                s_name: "n_blue", 
                s_type: "number",
                n_bits: 8,
            }, 
            O_property_static_sized{
                s_name: "n_alpha", 
                s_type: "number",
                n_bits: 8,
            }
        ]
    }
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    let a_nu8__image_stream: Vec<u8> = vec![
        122,255,25,100,
        10,25,225,00,
        1,4,25,10,
        22,55,10,50,
    ];

    for nu8 in &a_nu8__image_stream{

        println!("{:?}", nu8);
    }
}
fn main() {
    f_manual();

}