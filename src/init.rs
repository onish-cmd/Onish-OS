#![no_std]
#![no_main]

use core::panic::PanicInfo; // DO NOT REMOVE THIS BECAUSE OF TESTS
extern crate os_utils;
extern crate sc;

// CAUTION: DO NOT EDIT ANY OF THIS FILE AND RECOMPILE UNLESS YOU HAVE TO!

#[no_mangle]
pub extern "C" fn _start() -> ! {
    os_utils::print("Welcome to Onish-OS");

    let cmd = "/bin/bash\0";

    unsafe {
        sc::syscall3(
            221,
            cmd.as_ptr() as usize,
            0,
            0
        );
    }
    loop {}
}

#[cfg(not(test))] //added to pass tests
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}