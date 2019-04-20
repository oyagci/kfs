#![no_std]

mod vga_buffer;

extern "C" {
    fn disable_cursor();
}

#[no_mangle]
pub fn kmain() {
    unsafe { disable_cursor(); }

    println!("(KFS) Welcome.");
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
