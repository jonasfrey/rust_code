import { createCanvas, loadImage } from "https://deno.land/x/canvas/mod.ts";

import { decode as f_decode_png } from "https://deno.land/x/pngs/mod.ts";

import JPEG from "https://deno.land/x/jpeg/mod.ts";

// const img = await Deno.readFile("./img.jpg");

import { resize } from "https://deno.land/x/deno_image@v0.0.2/mod.ts";


  
var s_path_image = "./7.jpg";
var s_path_image_resized = "./7_resized.jpg";


const f_blur_image = function(
    a_nu8, 
    n_width, 
    n_height, 
    n_channels = 4, 
    n_kernel_size_x = 3,
    n_kernel_size_y = 3
){
    // console.log(a_nu8)
    // console.log(a_nu8[0])
    // Deno.exit(1)
    // var n_kernel_pixels = n_kernel_size_x * n_kernel_size_x
    var n_kernel_pixels = n_kernel_size_x
    var n_pixel = 0; 
    while(n_pixel < n_width*n_height){
        var n_pixel_x = n_pixel % n_width;
        var n_pixel_y = parseInt(n_pixel / n_width);
        var n_index_pixel = n_pixel * n_channels;
        
        var n_kernel_index = 0;
        var a_nu32__pixel_sum = new Uint32Array(n_channels);

        var n_kernel_y = 0; 
        while( n_kernel_y < n_kernel_size_y){
            var n_kernel_x = 0; 
            while(n_kernel_x < n_kernel_size_x){
                var n_x = n_pixel_x + n_kernel_x
                var n_y = n_pixel_y + n_kernel_y
                var n_channel = 0; 
                var n_index_pixel_kernel = n_x * n_channels + n_y * n_width * n_channels
                while(n_channel < n_channels){
                    a_nu32__pixel_sum[n_channel] += a_nu8[n_index_pixel_kernel+n_channel]
                    n_channel+=1
                }
                n_kernel_x+=1;
            }
            n_kernel_y+=1;
        }
        // console.log(a_nu32__pixel_sum);
        var n_channel = 0;
        while(n_channel < n_channels){
            a_nu8[n_index_pixel+n_channel] = parseInt(a_nu32__pixel_sum[n_channel]/(n_kernel_size_x*n_kernel_size_y))
            n_channel+=1
        }

        n_pixel+=1;
    }
}


const n_width = 1920;
const n_height = 1080;

const img = await resize(await Deno.readFileSync(s_path_image), {width: n_height, height: n_height});

Deno.writeFileSync(s_path_image_resized, img);

const canvas = createCanvas(
    n_width, 
    n_height
);
const ctx = canvas.getContext("2d");

ctx.fillStyle = "red";
ctx.fillRect(10, 10, 200 - 20, 200 - 20);


// // const file = await Deno.open(s_path_image);
// const file = await Deno.readFile(s_path_image);

// const raw = JPEG.decode(file);
// console.log(raw)

// // var decoded = await (decode(file));
// // console.log(decoded);

// var iData = new ImageData(raw.data, raw.width, raw.height);

// ctx.putImageData(iData, 0,0)
// // const bytesRead = await file.read(buf);


const image = await loadImage(s_path_image);
console.log(image)
console.log(image);
console.log(canvas.toBuffer())
// ctx.putImageData((new Uint8Array(100)).fill(200).buffer, 0, 0)
ctx.drawImage(image,0,0);

var id = ctx.getImageData(0,0,n_width,n_height);
// console.log(id.data)

// f_blur_image(
//     id.data,
//     n_width, 
//     n_height,
//     4,
//     50,
//     20,
// )
// console.log(id.data)

ctx.filter = "blur(10px)"
ctx.putImageData(id,0,0)
// ctx.filter = 'blur(10px)';
// ctx.filter = "saturate(0%)";
// ctx.filter = 'contrast(1.4) sepia(1) drop-shadow(-9px 9px 3px #e81)';

// console.log(ctx);
// await Deno.writeFile("image.png", file.buffer);
await Deno.writeFile("image.png", canvas.toBuffer());