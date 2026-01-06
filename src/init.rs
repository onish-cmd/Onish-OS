#![no_std]
#![no_main]

use core::panic::PanicInfo;
use os_utils::{print};
use sc;

// CAUTION: DO NOT EDIT ANY OF THIS FILE AND RECOMPILE UNLESS YOU HAVE TO!

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print("Welcome to Onish-OS");

    let cmd = "bin/bash\0";

    unsafe {
        sc::syscall3(
            sc::nr::EXECVE,
            cmd.as_ptr() as usize,
            0,
            0,
        )
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) {
    loop {}
}