#![no_std]
#![feature(asm)]

mod vga_buffer;
mod utils;

#[allow(unused_attributes)]
#[no_mangle]
pub fn kmain() {
    utils::disable_cursor();
    utils::enable_cursor(14, 15);

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
