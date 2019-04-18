#![no_std]

#[no_mangle]
pub extern "C" fn krust() {
    let vga_buffer = 0xb8000 as *mut u8;
    let s: &[u8] = b"KFS-1 by Oguzhan YAGCI <oyagci@student.42.fr>";

    for (i, &byte) in s.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
