
var f_a_o_object_by_Uint8Array = function(
    a_uint8,
    o_object
){
    const a_buf = a_uint8.buffer
    
    console.log("stated")
    var n_bytes_o_struct = o_object.a_o_object_property.map((o) =>o.n_bytes).reduce((n_sum, n) => n_sum+=n);
    console.log(n_bytes_o_struct)
    console.log(a_uint8.byteLength)
    // var n_i = 0;
    var n_i = 8; // we have to start with index 8 because in rust the first 8 bytes of array are used for the number of elements(structs in form of u8/bytes) in the array (let a_u8: Vec<u8> bincode::serialize(&[Vec<Struct>]).unwrap())
    var a_o = []
    
    const o_text_decoder_utf8 = new TextDecoder("utf-8")

    while(n_i < a_uint8.byteLength){
    
        var o = {}
        for(var n_index in o_object.a_o_object_property){
            const o_object_property = o_object.a_o_object_property[n_index]
            if(o_object_property.o_type.s_name_abstract == "string"){
                // we have to get rid of the padding \x00 bytes at the end of the string
                var a_buffer_sub = a_uint8.subarray(n_i, Array.prototype.indexOf.call(a_uint8, 0, n_i));
                console.log(a_uint8)
                console.log(n_i)
                o[o_object_property.s_name] = o_text_decoder_utf8.decode(a_buffer_sub); //version 1 thanks @AapoAlas 
            }else{
                o[o_object_property.s_name] = new DataView(a_buf, n_i, o_object_property.o_type.n_bytes)[o_object_property.o_type.s_js_dataview_getter_function_name](0, true);
            }

            n_i += o_object_property.o_type.n_bytes;
        }
        // console.log(o)
        a_o.push(o)
        // n_i+=n_bytes_o_struct;
    }
    
    return a_o; 

}
    



    export {f_a_o_object_by_Uint8Array}