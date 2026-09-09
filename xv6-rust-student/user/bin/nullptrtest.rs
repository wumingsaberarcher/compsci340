#![no_std]
#![no_main]

use user::*;

#[unsafe(no_mangle)]
fn main(_args: Args) {
    let ptr: *const i32 = core::ptr::null();
    unsafe {
        println!("{}", *ptr); // attempt to dereference a null pointer
    }
}
