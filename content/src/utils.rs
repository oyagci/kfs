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

