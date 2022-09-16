
// var a_uint8 = new Uint8Array(Array.from(Array(128).keys()));
// var string = new TextDecoder().decode(a_uint8);
// console.log(string)

import {readFileSync} from "fs"
import {f_do_it} from "./f_do_it.js"

// await Bun.write ('./A_o_person.dat', a_uint8_file_content);
// const a_uint8_file_content =await Deno.readFile('./A_o_person.dat');
const a_uint8_file_content = readFileSync('./A_o_person.dat');

f_do_it(a_uint8_file_content)
