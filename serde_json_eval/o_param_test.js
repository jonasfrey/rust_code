var o_param_json_complex = {
    "a_o":[
        {
            ":s_evaluate_me": "1+1"
        },
        // this object will be a_o[-1]
        {
            "b_enumerable": false,
            //each s_property_name starting with ":s"
            // will mathematically evaluated
            // the result will be present with the 
            // s_property_name ':s_...' => 'n_...'
            "n_evaluate_me": "2"
        },
        //
        {
            ":s_evaluate_me": "a_o[-1]+5"
        }
    ]
}
var o_param_json_simple = {
    "a_o":[
        {
            ":s_evaluate_me": "1+1"
        },
        // this object will be a_o[-1]
        {
            "b_enumerable": false,
            //each s_property_name starting with ":s"
            // will mathematically evaluated
            // the result will be present with the 
            // s_property_name ':s_...' => 'n_...'
            "n_evaluate_me": "2"
        },
        //
        {
            ":s_evaluate_me": "2*3"
        }
    ]
}

// var o_param_json = o_param_json_complex;
var o_param_json = o_param_json_simple;

var s_path_file = import.meta.url.split("/").pop().split(".").slice(0,-1).join(".") + ".json";

var s_o_param_json = JSON.stringify(
    {a_o: o_param_json.a_o.filter(o=>o.b_enumerable != false)}
)

Deno.writeTextFile(
    s_path_file,
    s_o_param_json
)

// const p = await Deno.run({ cmd: ["cargo", "run", `'$(<${s_path_file})'`], stdout:"piped" });
const p = await Deno.run({ cmd: ["cargo", "run", s_o_param_json], stdout:"piped" });
await p.status();
console.log(new TextDecoder().decode(await p.output()));
//-a b c dconsole.log(new TextDecoder().decode(await p.stderrOutput()));
