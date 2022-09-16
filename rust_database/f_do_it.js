
var f_do_it = function(a_uint8_file_content){

// const a_uint8_file_content =await fetch('file:///A_o_person.dat');
var string = new TextDecoder().decode(a_uint8_file_content);
// console.log(string)

const a_buf =a_uint8_file_content.buffer

var a_o_struct_prop = [

    {n_bytes:4, s_type: "number", s_name: "n_id"},
    {n_bytes:20, s_type: "string", s_name: "s_name"},
    {n_bytes:20, s_type: "string", s_name: "s_email"},

  ]

var n_bytes_o_struct = a_o_struct_prop.map((o) =>o.n_bytes).reduce((n_sum, n) => n_sum+=n);

var n_i = 0; 

var a_o = []

const o_text_decoder_utf8 = new TextDecoder("utf-8")
while(n_i < a_uint8_file_content.byteLength){

    var o = {}
    for(var n_index in a_o_struct_prop){
        const o_struct_prop = a_o_struct_prop[n_index]
        if(o_struct_prop.s_type == "string"){
            o[o_struct_prop.s_name] = o_text_decoder_utf8.decode(new DataView(a_buf, n_i, o_struct_prop.n_bytes))
        }
        if(o_struct_prop.s_type == "number"){
            o[o_struct_prop.s_name] = new DataView(a_buf, n_i, o_struct_prop.n_bytes)["getUint"+o_struct_prop.n_bytes*8](0, true);
        }
        n_i += o_struct_prop.n_bytes;
    }
    a_o.push(o)
    // n_i+=n_bytes_o_struct;
}

console.log(a_o[0])
console.log("...")
console.log(a_o[a_o.length-1])
}

export {f_do_it}