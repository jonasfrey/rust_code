

class O_model_property{
    constructor(
        s_name,
        s_type,
        // o_model_property_type,
        n_bytes = false, 
        n_decimal_places = false, 
        b_private = false,
        n_minimum_number = null, 
        n_maximum_number = null, 
        n_minimum_string_length = null, 
        n_maximum_string_length = null, 
        s_regex = null,
        default_value = null, 
        b_required = null, 
        b_unique = null,
    ){
        this.s_name = s_name
        this.s_type = s_type
        // this.o_model_property_type = o_model_property_type
        this.n_bytes = n_bytes
        this.n_decimal_places = n_decimal_places
        this.b_private = b_private
        this.n_minimum_number = n_minimum_number
        this.n_maximum_number = n_maximum_number
        this.n_minimum_string_length = n_minimum_string_length
        this.n_maximum_string_length = n_maximum_string_length
        this.s_regex = s_regex
        this.default_value = default_value
        this.b_required = b_required
        this.b_unique = b_unique

        var s_prefix = 'getInt'
        if(this.s_type == "unsigned_integer"){
            s_prefix = `getUint`
        }
        if(this.s_type == "integer"){
            s_prefix = `getInt`
        }
        if(this.s_type == "float"){
            s_prefix = `getFloat`
        }
        this.s_js_dataview_getter_function_name = `${s_prefix}${this.n_bytes*8}`;

    }
    f_value_by_u8intarray(o_dataview){
        var value = undefined;
        if(this.o_model_property_type.s_name == "unsigned_integer"){
            value = o_dataview()
        }
        new DataView(a_buf, n_i, o_object_property.o_type.n_bytes)[o_object_property.o_type.s_js_dataview_getter_function_name](0, true);
    }

}

class O_model{
    constructor(
        s_name, 
        a_o_model_property, 
    ){
        this.s_name = s_name
        this.a_o_model_property = a_o_model_property
    }
}

var f_a_o_object_by_Uint8Array = function(
    a_uint8,
    o_model
){
    const a_buf = a_uint8.buffer
    
    // var n_i = 0;
    var n_bytes_offset = 8; // we have to start with index 8 because in rust the first 8 bytes of array are used for the number of elements(structs in form of u8/bytes) in the array (let a_u8: Vec<u8> bincode::serialize(&[Vec<Struct>]).unwrap())
    var a_o = []
    
    const o_text_decoder_utf8 = new TextDecoder("utf-8")

    while(n_bytes_offset < a_uint8.byteLength){
    
        var o = {}
        for(var n_index in o_model.a_o_model_property){
            const o_model_property = o_model.a_o_model_property[n_index]
            // console.log(o_model_property)
            // console.log(n_bytes_offset)
            var val = undefined;
            if(o_model_property.s_type == "boolean"){
                val = (new DataView(
                    a_buf,
                    n_bytes_offset,
                    1
                ).getUint8(0,true) != 0) ?true: false;
                n_bytes_offset = n_bytes_offset + 1
            }
            if(o_model_property.s_type == "string"){
                // const o_text_decoder_utf8 = new TextDecoder("utf-8")
                // var a = new Uint8Array([66,67,68,69])
                // var s = String.fromCharCode.apply(null, a.subarray(2,4));
                // var s2 = String.fromCharCode.apply(null, a);
                // var s3 = o_text_decoder_utf8.decode(a.subarray(2,4));
                var n_string_length = Number(new DataView(
                    a_buf,
                    n_bytes_offset + 0,
                    8
                ).getBigUint64(0, true));
                val = o_text_decoder_utf8.decode(
                    a_uint8.subarray(
                        n_bytes_offset + 8,
                        n_bytes_offset + 8 + n_string_length
                    )
                );
                n_bytes_offset = n_bytes_offset + 8 + n_string_length
            }

            if(o_model_property.s_type !== "boolean" && o_model_property.s_type !== "string"){
                val = new DataView(
                    a_buf,
                    n_bytes_offset,
                    o_model_property.n_bytes
                    )[o_model_property.s_js_dataview_getter_function_name](0, true);

                n_bytes_offset = n_bytes_offset + o_model_property.n_bytes
            }
            // console.log(`val: `+val);         
            // console.log("n_bytes_offset");
            // console.log(n_bytes_offset);
            // console.log(a_uint8.byteLength);
            o[o_model_property.s_name] = val
    

        }
        a_o.push(o)

    }
    
    return a_o; 

}
    

class O_crud_operation{
    constructor(
        s_name,
        o_model, 
        s_filter_function,
        o_filter_function_param, 
        s_update_function,
        o_update_function_param,
    ){
        this.s_name = s_name 
        this.o_model = o_model 
        this.s_filter_function = s_function_name
        this.o_filter_function_param = o_filter_function_param
        this.s_update_function = s_function_name 
        this.o_update_function_param = o_update_function_param
    }
}

class O_deno_rust_db{
    constructor(){

    }

    f_a_o(

    ){

    }
}


//usage 
var a_o_model = [
    new O_model(
        "O_person", 
        [
            new O_model_property(
                "n_id", 
                "unsigned_integer", 
                4
            ), 
            new O_model_property(
                "s_name", 
                "string"
            ),
            new O_model_property(
                "b_male", 
                "boolean"
            ),
            new O_model_property(
                "n_age", 
                "float", 
                4, 
                5
            )
        ]
    )
]
// new o_deno_rust_db = O_deno_rust_db();
// //create 
// o_deno_rust_db.f_a_o(
//     new O_crud_operation(
        
        
//     )
// )

//test 
const a_uint8_file_content = await Deno.readFile('./A_o_person');
var a_o = f_a_o_object_by_Uint8Array(
    a_uint8_file_content, 
    a_o_model[0]
)
console.log(a_o)

export {O_deno_rust_db}