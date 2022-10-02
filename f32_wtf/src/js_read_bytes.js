// ```rust
// let n: f32 = 33.3300018; 
// let a_nu8 =  n.to_be_bytes()
// println!("n {:?}", n);
// println!("a_nu8 {:?}",a_nu8);
// ```

var a_nu8 = new Uint8Array([66, 5, 81, 236]);
var n = new DataView(a_nu8.buffer);
var b_little_endian = false;
console.log(n.getFloat32(b_little_endian));

var a_nu8 = new Uint8Array([66, 5, 81, 236, 0,0,0,0]);
var n = new DataView(a_nu8.buffer);
var b_little_endian = false;
console.log(n.getFloat64(b_little_endian));