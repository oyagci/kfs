#![no_std]

mod vga_buffer;

extern "C" {
    fn disable_cursor();
}

#[no_mangle]
pub extern "C" fn kmain() {
    unsafe { disable_cursor(); }

    for i in 0..30 {
        println!("KFS-{}", i);
    }
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
