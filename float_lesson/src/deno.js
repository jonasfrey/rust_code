

var a_nu8 = new Uint8Array(
    [
        // signed bit is 0, so b_negative = false
        // and all exponent bits are set to 1: +Infinity
        // 0b01111111,0b10000000,0b00000000,0b00000000 
        // signed bit is 1, so b_negative = true
        // and all exponent bits are set to 1: -Infinity
        // 0b11111111,0b10000000,0b00000000,0b00000000 // all exponent bits are set to 1: Infinity
        // 0b11000010,0b11100101,0b01000000,0b00000000 // from https://www.youtube.com/watch?v=2dopLI1GZig
        0b11000010,0b11100101,0b01000000,0b10101100 // check to see what happens when we fill the padding with random bits
    ]
);
var n = new DataView(a_nu8.buffer);
var b_little_endian = false;
console.log(n.getFloat32(b_little_endian));
