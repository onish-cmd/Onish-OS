#![no_std]

const UART_TX_REG: *mut u8 = 0x10000000 as *mut u8;

fn uart_punch_her(c: u8) {
    unsafe {
        UART_TX_REG.write_volatile(c);
    }
}

pub fn print(prompt: &str) {
    for byte in prompt.bytes() {
        uart_punch_her(byte)
    }
}