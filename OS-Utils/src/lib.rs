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