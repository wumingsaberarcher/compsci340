#![no_std]
#![no_main]

use user::*;

#[unsafe(no_mangle)]
fn main(_args: Args) {
    let addr = sbrk(100).expect("sbrk failed!");
    let page = addr as *mut u8;

    // write to make sure memory has been allocated and is writable (set to 255)
    unsafe {
        *page.add(0) = 0xFF;
    }

    mprotect(addr).expect("mprotect failed!");

    unsafe {
        // *page.add(0) = (1 & 0xFF) as u8; // if you uncomment this line,
        // the program stops, as the kernel kills it for trying to write to
        // a protected page
        println!("{}", *page.add(0)); // should still be 255
    }

    munprotect(addr).expect("munprotect failed!");

    unsafe {
        *page.add(0) = (2 & 0xFF) as u8;
        println!("{}", *page.add(0)); // should read 2
    }
}
