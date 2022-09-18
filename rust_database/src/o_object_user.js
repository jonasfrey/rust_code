import {O_object} from "./O_object.js"
import {O_object_property} from "./O_object_property.js"
import {
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
} from "./O_object_property_type.js"

var o_object_user = new O_object(
  "O_user", 
  [
      new O_object_property(
          {
            s_name: "n_id", 
            o_type: o_object_property_type_u32
          },
      ),
      new O_object_property(
          {
            s_name: "n_age_milliseconds", 
            o_type: o_object_property_type_u32
          },
      ),
      new O_object_property(
          {
            s_name: "s_name", 
            o_type: f_o_object_property_type_string(20)// n bytes
          },
      ),
  ]
)

export {o_object_user}