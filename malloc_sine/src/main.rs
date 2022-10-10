use std::alloc::System;
use std::alloc::{alloc, dealloc, realloc, Layout};
use libc;

use std::mem;
fn f_huge_vector(){
    let mut a_nu8 : Vec<u8> = Vec::new();

    let mut n_i = 0; 
    let mut n_i_max: u64 = 16 * 1000 * 1000 * 1000; 
    let mut n_time = 1 * 1000 * 1000;
    loop{
        
        // a_nu8 = vec![255; n_i as usize];
        let mut n_j = 0;
        // while(n_j < n_time){
        //     a_nu8.push(255);
        //     n_j+=1;
        // }
        // a_nu8 = vec![0; n_time as usize];
        a_nu8.append(
            &mut vec![0; n_time as usize]
        );
        if(n_i==0){
            a_nu8  = Vec::new();
        }
        n_i= (n_i+n_time) % n_i_max;
    }
}
fn f_cmalloc(){
    unsafe {
        let my_num: *mut u64 = libc::malloc(20*1000*1000*1000 as libc::size_t) as *mut u64;
        if my_num.is_null() {
            panic!("failed to allocate memory");
        }
        // libc::free(my_num as *mut libc::c_void);
        loop{

        }
    }
}

fn f_saw_tooth(){

    let mut n_i = 0; 
    let mut n_i_max: u64 = 4 * 1000 * 1000 * 1000; 
    //                ^giga  ^mega  ^kilo  ^ byte
    unsafe{

        let mut a = Box::new(0); // Allocates from the system allocator.
        let mut layout = Layout::from_size_align(10000000, 2).unwrap();
        let mut ptr = alloc(layout);
        // loop{
        //         n_i = (n_i + 100000) % n_i_max;
            
        //         // *(ptr as *mut u16) = 42;
        //         // assert_eq!(*(ptr as *mut u16), 42);
                
        //         dealloc(ptr, layout);
        //         layout = Layout::from_size_align(n_i, 2).unwrap();
        //         ptr = alloc(layout);
        //         println!("{:?}", n_i);
        // }
    }
}
fn main() {
    // f_saw_tooth();
    // f_cmalloc();
    f_huge_vector();
    // println!("Hello, world!");
}
