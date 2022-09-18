// import {readFileSync} from "fs"
import {f_a_o_object_by_Uint8Array} from "./f_a_o_object_by_Uint8Array.js"
import {o_object_user} from "./o_object_user.js"

const a_uint8_file_content =await Deno.readFile('./A_o_user_f_a_o_read');

var a_o = f_a_o_object_by_Uint8Array(a_uint8_file_content, o_object_user)
console.log(a_o)