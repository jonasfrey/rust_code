

const p_build = Deno.run({cmd:["cargo", "build"]})
const { n_process_build_return_code } = await p_build.status();


var a_s_name = [
  "jonas", 
  "kurt", 
  "bert", 
  "claudio", 
  "dagobert", 
  "elias", 
  "franz", 
  "gertrud", 
  "hans", 
  "ilias", 
  "jakob"
]
var o_param_create = {
  object: {
      //    s_name_wriasdflkjasd: "abcdefghijklmnopqrstuvwxyzjonas"
      // s_name: "abcdefghijklmnopqrstuvwxyzjonas"
      // s_name: "ﻻﻻﻻﻻﻻﻻﻻﻻﻻﻻ",// testing arabic unicode char
      n_id: 5,
      s_name: a_s_name[parseInt(Math.random()* a_s_name.length)],
      n_age_milliseconds: 10294967295 // u32 max is 4294967295
  },
  s_function_name: "f_a_o_create"
};
            
var o_param_read = {
  object: {
    // s_name: "jonas",
    n_id: 2,
},
s_function_name: "f_a_o_read"
}
var o_param_update = {
  object: {
    n_id: 5,
    s_name: "JONAAAAS",
},
s_function_name: "f_a_o_update"
}
// var o_param = o_param_create
// var o_param = o_param_read
var o_param = o_param_update

// var s_json = `'${JSON.stringify(o_param)}'`
var s_json = `${JSON.stringify(o_param)}`

var a_s_command = [
         "./../target/debug/rust_database",
         s_json
]
const p = Deno.run({
   cmd: a_s_command,
   stdout: "piped",
   stderr: "piped",
 });
 
 console.log(a_s_command.join(" "))
 const { code: n_process_return_code } = await p.status();
 
//  console.log(n_process_return_code)
 // Reading the outputs closes their pipes
 const a_output = await p.output();
 const a_error = await p.stderrOutput();
 const s_output = new TextDecoder().decode(a_output);
 const s_error = new TextDecoder().decode(a_error);

 if (n_process_return_code === 0) {
    //    await Deno.stdout.write(rawOutput);
    console.log("success")
   console.log(s_output);

 } else {
   console.log("error")
   console.log(s_error);
 }


Deno.exit(n_process_return_code);