#![no_std]
#![feature(asm)]

mod vga_buffer;
mod utils;

use utils::outb;

fn disable_cursor() {
    outb(0x3D4, 0x0A);
    outb(0x3D5, 0x20);
}

#[no_mangle]
pub fn kmain() {
    println!("
  _  _______ ____
 | |/ /  ___/ ___|
 | ' /| |_  \\___ \\
 | . \\|  _|  ___) |
 |_|\\_\\_|   |____/
");
    println!(" (KFS) Kernel From Scratch\n");

    print!("#> ");
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
