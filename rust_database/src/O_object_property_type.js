class O_object_property_type{
    constructor(
        o
    ){
        var a_s_prop_name_allowed = [
            "n_bytes", 
            "s_name_abstract", 
            "s_rust_name",
            "s_js_dataview_getter_function_name",
            "b_can_be_negative"
        ]
        for(var s_prop_name in o){
            if(a_s_prop_name_allowed.includes(s_prop_name)){
                this[s_prop_name] = o[s_prop_name]
            }else{
                console.log(`s_prop_name ${s_prop_name} is not allowed`)
            }
        }

    }
}

var o_object_property_type_u8 = new O_object_property_type(
    {
        n_bytes:1,
        s_name_abstract: "integer",
        s_rust_name: "u8",
        s_js_dataview_getter_function_name: "getUint8", 
        b_can_be_negative: false
    }
)
var o_object_property_type_u16 = new O_object_property_type(
    {
        n_bytes:2, 
        s_name_abstract: "integer",
        s_rust_name: "u16",
        s_js_dataview_getter_function_name: "getUint16", 
        b_can_be_negative: false
    }
)
var o_object_property_type_u32 = new O_object_property_type(
    {
        n_bytes:4, 
        s_name_abstract: "integer",
        s_rust_name: "u32",
        s_js_dataview_getter_function_name: "getUint32", 
        b_can_be_negative: false
    }
)
var o_object_property_type_u64 = new O_object_property_type(
    {
        n_bytes:8, 
        s_name_abstract: "integer",
        s_rust_name: "u64",
        s_js_dataview_getter_function_name: "getUint64",//not supported by js DataView 
        b_can_be_negative: false
    }
)
var o_object_property_type_i8 = new O_object_property_type(
    {
        n_bytes:1, 
        s_name_abstract: "integer",
        s_rust_name: "i8",
        s_js_dataview_getter_function_name: "getInt8", 
        b_can_be_negative: true
    }
)
var o_object_property_type_i16 = new O_object_property_type(
    {
        n_bytes:2, 
        s_name_abstract: "integer",
        s_rust_name: "i16",
        s_js_dataview_getter_function_name: "getInt16", 
        b_can_be_negative: true
    }
)
var o_object_property_type_i32 = new O_object_property_type(
    {
        n_bytes:4, 
        s_name_abstract: "integer",
        s_rust_name: "i32",
        s_js_dataview_getter_function_name: "getInt32", 
        b_can_be_negative: true
    }
)
var o_object_property_type_i64 = new O_object_property_type(
    {
        n_bytes:8, 
        s_name_abstract: "integer",
        s_rust_name: "i64",
        s_js_dataview_getter_function_name: "getInt64",//not supported by js DataView 
        b_can_be_negative: true
    }
)
var o_object_property_type_f32 = new O_object_property_type(
    {
        n_bytes:4, 
        s_name_abstract: "float",
        s_rust_name: "f32",
        s_js_dataview_getter_function_name: "getFloat32", 
        b_can_be_negative: true
    }
)
var o_object_property_type_f64 = new O_object_property_type(
    {
        n_bytes:8, 
        s_name_abstract: "float",
        s_rust_name: "f64",
        s_js_dataview_getter_function_name: "getFloat64", 
        b_can_be_negative: true
    }
)


var f_o_object_property_type_string = function(
    n_bytes
){
    return new O_object_property_type(
        {
            n_bytes:n_bytes, 
            s_name_abstract: "string",
            s_rust_name: `[u8;${n_bytes}]`,
            b_can_be_negative: false
        }
    )
}

export {
    O_object_property_type,
    o_object_property_type_u8,
    o_object_property_type_u16,
    o_object_property_type_u32,
    o_object_property_type_u64,
    o_object_property_type_i8,
    o_object_property_type_i16,
    o_object_property_type_i32,
    o_object_property_type_i64,
    o_object_property_type_f32,
    o_object_property_type_f64,
    f_o_object_property_type_string
}