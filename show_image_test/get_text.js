//luckily i wrote this sometime 

import {O_binary_string} from "https://deno.land/x/o_binary_string@0.7/O_binary_string.module.js"



// var n_font_size = 10
// var s_string = "Axy!`"
// const o_binary_string = new O_binary_string(s_string, n_font_size)
// console.log(o_binary_string.o_canvas_data);
var n_start = 32; 
var n_end = 127; 
var n_i = n_start; 
var n_font_size = 15
var s_path_file = `./src/constants_chars_a_u8.rs`;
var s_file_content = ``
var a_n_size_x = [];
var a_n_size_y = [];
var a_n_start_index_chars_information = [];
var a_chars_information = new Uint8Array();


while(n_i < n_end){
    var s_char = String.fromCharCode(n_i);
    // console.log(s_char);
    try{
        var o_binary_string = new O_binary_string(s_char, n_font_size)
        // await Deno.writeFile(`./${s_char}charu8_n_channelsu8_n_widthu16_n_heightu16_`, o_binary_string.o_canvas_data.o_image_data_cropped.data, { mode: 0o644 });
        s_file_content += `
            // s_char: '${s_char}', n_char: ${n_i}
            pub const a_u8_char_${n_i}: [u8;${o_binary_string.o_canvas_data.o_image_data_cropped.data.length}] = [${o_binary_string.o_canvas_data.o_image_data_cropped.data.toString()}];
            pub const n_size_x_char_${n_i} : u32 = ${o_binary_string.o_canvas_data.o_image_data_cropped.width};
            pub const n_size_y_char_${n_i} : u32 = ${o_binary_string.o_canvas_data.o_image_data_cropped.height};
        `
        a_n_size_x.push(o_binary_string.o_canvas_data.o_image_data_cropped.width)
        a_n_size_y.push(o_binary_string.o_canvas_data.o_image_data_cropped.height)
        a_n_start_index_chars_information.push(a_chars_information.length);
        var a_chars_information_new = new Uint8Array(a_chars_information.length + o_binary_string.o_canvas_data.o_image_data_cropped.data.length)
        a_chars_information_new.set(a_chars_information);
        a_chars_information_new.set(o_binary_string.o_canvas_data.o_image_data_cropped.data, a_chars_information.length);
        a_chars_information = a_chars_information_new
        
    }catch(e){
        console.log(e)
        console.log(s_char)
    }
    n_i+=1;
}
s_file_content += `
// information for all characters, can be retrieved by a_...[32+n_number_of_ascii_char]
pub const n_channels: u8 = 4;
pub const a_n_size_x: [u32;${a_n_size_x.length}] = [${a_n_size_x.toString()}];
pub const a_n_size_y: [u32;${a_n_size_y.length}] = [${a_n_size_y.toString()}];
pub const a_chars_information: [u8;${a_chars_information.length}] = [${a_chars_information.toString()}];
pub const a_n_start_index_chars_information: [u32;${a_n_start_index_chars_information.length}] = [${a_n_start_index_chars_information.toString()}];

`
console.log(a_chars_information)
await Deno.writeTextFile(s_path_file, s_file_content)

// const bytes = new Uint8Array([72, 101, 108, 108, 111]);
// await Deno.writeFile("hello.txt", bytes, { mode: 0o644 });