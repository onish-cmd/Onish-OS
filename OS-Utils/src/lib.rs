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