#![allow(unused)]

use crate::keyboard_driver::Keyboard;
use crate::print;

pub fn outb(port: u16, val: u8) {
    unsafe {
        asm!("out dx, al" : : "{dx}"(port), "{al}"(val) : : "intel", "volatile");
    }
}

pub fn inb(port: u16) -> u8 {
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

pub fn enable_cursor(start: u8, end: u8) {
    outb(0x3D4, 0x0A);
    outb(0x3D5, (inb(0x3D5) & 0xC0) | 14);

    outb(0x3D4, 0x0B);
    outb(0x3D5, (inb(0x3D5) & 0xE0) | 15);
}

pub struct KReadlineOutput {
    buffer: [char; 512]
}
impl KReadlineOutput {
    pub fn dump(&self) {
        for i in self.buffer.iter() {
            if *i == 0 as char { break; }
            print!("{}", i as &char);
        }
    }
}

pub fn kreadline(kb: &mut Keyboard, s: &str) -> KReadlineOutput {
    let mut output = KReadlineOutput { buffer: ['\0'; 512] };
    let mut i: usize = 0;

    print!("{}", s);

    loop {
        match kb.update() {
            Some(s) => match s {
                    b'\n' => {
                        print!("{}", s as char);
                        output.buffer[i] = '\n';
                        return output;
                    },
                    127 => if i > 0 {
                        print!("{}", s as char);
                        i -= 1;
                    },
                    _ => {
                        print!("{}", s as char);
                        output.buffer[i] = s as char;
                        i += 1;
                    }
            },
            None => {}
        };
    }
}
