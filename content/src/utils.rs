#![allow(unused)]

pub fn outb(port: u16, val: u8) {
    unsafe {
        asm!("out dx, al" : : "{dx}"(port), "{al}"(val) : : "intel", "volatile");
    }
}

pub fn inb(port: u8) -> u8 {
    let result: u8;

    unsafe {
        asm!("in al, dx" : "={al}"(result) : "{dx}"(port) : : "intel");
    }
    result
}

pub fn disable_cursor() {
    outb(0x3D4, 0x0A);
    outb(0x3D5, 0x20);
}
