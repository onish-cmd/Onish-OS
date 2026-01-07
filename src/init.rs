use std::io::{self, Write};

use os_utils;
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
    os_utils::print("--VERSION 0.6--\n");

    // START BASH
    os_utils::attach_console();
    let pid = os_utils::fork();

    if pid == 0 {
        let cmd = "/bin/sh\0";
        let argv: [*const u8; 2] = [cmd.as_ptr(), core::ptr::null()];
        let envp: [*const u8; 1] = [core::ptr::null()];
        unsafe {
            sc::syscall3(221, cmd.as_ptr() as usize,
            argv.as_ptr() as usize,
            envp.as_ptr() as usize);
        }
        os_utils::print("Failed to start Bash");
        os_utils::suicide(93)
    } else {
        // ask user if they want to reboot or shutdown.
        os_utils::wait4child();
        let ans = os_utils::input("Do you want to reboot or shutdown?");
        if ans.to_lowercase() == "reboot" {
            os_utils::reboot();
        } else {
            os_utils::shutdown();
        }
    }
}