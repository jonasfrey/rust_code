
// A QOI file consists of a 14-byte header, followed by any number of data “chunks” and an 8-byte end marker. 
struct O_header_qoi {
    u8: s_magic_bytes,
    u32: n_pixels_x, //width
    u32: n_pixels_y, //height
    u8: n_channels,
    u8: b_colorspace // false = sRGB with linear alpha, true = all channels linear
}
struct O_pixel_rgbau8 { 
    u8: n_red,
    u8: n_green,
    u8: n_blue,
    u8: n_alpha,
}
struct 
fn f_n_index_by_o_pixel_rgbau8(
    O_pixel_rgbau8: o_pixel_rgbau8
){
    let n_index = (
        o_pixel_rgbau8.n_red * 3 +
        o_pixel_rgbau8.n_green * 5 +
        o_pixel_rgbau8.n_blue * 7 +
        o_pixel_rgbau8.n_alpha * 11 +
    ) % 64

    return n_index;
}

fn f_decode(){
    let o_pixel_previous = O_pixel_rgbau8(
        0,
        0,
        0,
        255
    );

}
fn main() {

    let a_nu8_end_marker : [u8] = [ 0, 0, 0, 0, 0, 0, 0, 1];

}
