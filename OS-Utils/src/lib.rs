extern crate sc;
use std::io::{self, Write};

pub fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Flush Failed!");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Read Failed!");
    return user_input.trim().to_string();
}

pub fn print(prompt: &str) {
    unsafe {
        sc::syscall3(
            64,
            1,
            prompt.as_ptr() as usize,
            prompt.len()
        );
    }
}

pub fn suicide(code: usize) -> ! {
    unsafe{
        sc::syscall1(93, code);
    }
    loop{}
}

pub fn mount(source: &str, target: &str, fstype: &str) {
    unsafe{
        sc::syscall5(
            40,
            source.as_ptr() as usize,
            target.as_ptr() as usize,
            fstype.as_ptr() as usize,
            0,
            0
        );
    }
}

pub fn fork() -> usize {
    unsafe {sc::syscall2(220, 17, 0)}
}

pub fn wait4child() {
    unsafe {sc::syscall4(260, -1isize as usize, 0, 0, 0)};
}

pub fn attach_console() {
    unsafe {
        let fd = sc::syscall4(56, -100isize as usize, "/dev/console\0".as_ptr() as usize, 2, 0);
        sc::syscall3(24, fd, 0, 0);
        sc::syscall3(24, fd, 1, 0);
        sc::syscall3(24, fd, 2, 0);
    }
}

// The "Safety Keys" for the Linux Kernel
const LINUX_REBOOT_MAGIC1: usize = 0xfee1dead;
const LINUX_REBOOT_MAGIC2: usize = 0x28121969;

// Command Codes
const LINUX_REBOOT_CMD_POWER_OFF: usize = 0x4321fedc;
const LINUX_REBOOT_CMD_RESTART: usize = 0x01234567;

pub fn reboot() {
    println!("Rebooting");
    unsafe {
        sc::syscall4(142, 
        LINUX_REBOOT_MAGIC1, 
        LINUX_REBOOT_MAGIC2, 
        LINUX_REBOOT_CMD_RESTART, 
        0);
    }
}
pub fn shutdown() {
    println!("Shuting Down");
    unsafe {
        sc::syscall4(142, 
        LINUX_REBOOT_MAGIC1, 
        LINUX_REBOOT_MAGIC2, 
        LINUX_REBOOT_CMD_POWER_OFF, 
        0);
    }
}