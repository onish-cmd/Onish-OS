#![no_std]
#![no_main]

use core::panic::PanicInfo;
extern crate os_utils;
extern crate sc;

// CAUTION: DO NOT EDIT ANY OF THIS FILE AND RECOMPILE UNLESS YOU HAVE TO!

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Mounts + Greetings
    os_utils::mount("proc\0", "/proc\0", "proc\0");
    os_utils::mount("devtmpfs\0", "/dev\0", "devtmpfs\0");
    os_utils::mount("sysfs\0", "/sys\0", "sysfs\0");
    os_utils::print("[ OK ] FILESYSTEMS MOUNTED\n");
    os_utils::print("Welcome to Onish-OS\n");

    // START BASH
    let cmd = "/bin/bash\0";
    let argv: [*const u8; 2] = [cmd.as_ptr(), core::ptr::null()];

    unsafe {
        sc::syscall3(
            221,
            cmd.as_ptr() as usize,
            argv.as_ptr() as usize,
            0
        );
    }
    loop {}
}

#[cfg(not(test))] //added to pass tests
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    os_utils::print("INIT PANIC!!!");
    os_utils::suicide(93);
}