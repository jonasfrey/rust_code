var n_i = 1; 
var n_max = 20;
var n_max_rot = Math.PI * 2;
var n_step_rot = n_max_rot / n_max;
var o_param = {
    "a_o_manipulation": [
        {
            "s_path_image_input": "default.png",
            "s_operation_name": "resize",
            "s_scale_x": `default.png.n_scale_x`, 
            "s_scale_y": `default.png.n_scale_y`,
            "s_path_image_output": "default_overlayed.png"
        },  
    ]
}
while(n_i < n_max){
    var n_rot = n_step_rot * n_i;

    o_param.a_o_manipulation.push(
        {
            "s_path_image_input": "default.png",
            "s_operation_name": "resize",
            "s_scale_x": `(default.png.n_scale_x/${n_max})*${n_max-n_i}`, 
            "s_scale_y": `(default.png.n_scale_y/${n_max})*${n_max-n_i}`,
            "s_path_image_output": "default_rescaled.png"
        },
        {
            "s_path_image_input": "default_overlayed.png",
            "s_operation_name": "overlay",
            "s_path_image_foreground": "default_rescaled.png",
            "s_translation_x": "(default.png.n_scale_x /2)- (default_rescaled.png.n_scale_x/2)",
            "s_translation_y": "(default.png.n_scale_y /2)- (default_rescaled.png.n_scale_y/2)",
            "s_path_image_output": "default_overlayed.png"
        }, 
    )
    n_i+=1;
}
Deno.writeTextFile(
    `${import.meta.url.split("/").pop().split('.').slice(0,-1).join('.')}.json`,
    JSON.stringify(o_param)
)