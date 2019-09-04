#![no_std]
#![feature(asm)]

mod keyboard_driver;
mod utils;
mod vga_buffer;

use keyboard_driver::Keyboard;
use vga_buffer::{set_global_color, Color};

fn print_kernel_logo() {
    set_global_color(Color::Green, Color::Black);
    println!("\n  _  _______ ____");
    set_global_color(Color::Yellow, Color::Black);
    println!(" | |/ /  ___/ ___|");
    set_global_color(Color::Red, Color::Black);
    println!(" | ' /| |_  \\___ \\");
    set_global_color(Color::Magenta, Color::Black);
    println!(" | . \\|  _|  ___) |");
    set_global_color(Color::Blue, Color::Black);
    println!(" |_|\\_\\_|   |____/");

    set_global_color(Color::White, Color::Black);
    print!("\n (KFS) ");
    set_global_color(Color::LightBlue, Color::Black);
    print!("Kernel ");
    set_global_color(Color::LightRed, Color::Black);
    print!("From ");
    set_global_color(Color::LightGreen, Color::Black);
    println!("Scratch\n");
    set_global_color(Color::White, Color::Black);

    set_global_color(Color::LightGray, Color::Black);
    println!(" By nbouchin and oyagci\n");
}

#[allow(unused_attributes)]
#[no_mangle]
pub fn kmain() {
    vga_buffer::clear_screen();
    utils::disable_cursor();
    utils::enable_cursor(14, 15);
    let mut kb = Keyboard::new();
    let mut s: utils::KReadlineOutput;

    print_kernel_logo();

    loop {
        s = utils::kreadline(&mut kb, "$> ");
        s.dump();
    }
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
