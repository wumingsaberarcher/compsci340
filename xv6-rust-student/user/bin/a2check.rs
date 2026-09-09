#![no_std]
#![no_main]

use user::*;

#[unsafe(no_mangle)]
fn main(_args: Args) {
    // Unaligned and unmapped addresses must be rejected.
    match mprotect(1) {
        Err(SysError::InvalidArgument) | Err(SysError::BadAddress) => {
            println!("unaligned: err");
        }
        other => {
            println!("unaligned: unexpected {:?}", other);
        }
    }

    // 32MiB is well past a freshly exec'd process size.
    match mprotect(32 * 1024 * 1024) {
        Err(SysError::InvalidArgument) | Err(SysError::BadAddress) => {
            println!("unmapped: err");
        }
        other => {
            println!("unmapped: unexpected {:?}", other);
        }
    }

    // Child: write after mprotect must be killed.
    match fork() {
        Ok(0) => {
            let addr = sbrk(100).expect("sbrk");
            unsafe {
                *(addr as *mut u8) = 0xAA;
            }
            mprotect(addr).expect("mprotect");
            unsafe {
                *(addr as *mut u8) = 0xBB;
            }
            println!("mprotect-write: survived");
            exit(0);
        }
        Ok(_) => {
            let mut status = 0usize;
            wait(&mut status).expect("wait");
            println!("mprotect-write: status {}", status as isize);
        }
        Err(_) => {
            println!("fork failed");
        }
    }

    // Child: a real load from address 0 (not optimized to unimp).
    match fork() {
        Ok(0) => {
            let ptr = 0usize as *const i32;
            let val = unsafe { core::ptr::read_volatile(ptr) };
            println!("null-load: survived {}", val);
            exit(0);
        }
        Ok(_) => {
            let mut status = 0usize;
            wait(&mut status).expect("wait");
            println!("null-load: status {}", status as isize);
        }
        Err(_) => {
            println!("fork failed");
        }
    }
}
