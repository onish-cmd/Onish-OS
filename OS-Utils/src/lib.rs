#![no_std]
extern crate sc;

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