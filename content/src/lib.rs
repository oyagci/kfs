#![no_std]
#![feature(asm)]

mod vga_buffer;
mod utils;

use vga_buffer::{Color,set_global_color};

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
    utils::disable_cursor();
    utils::enable_cursor(14, 15);

    print_kernel_logo();


    print!("#> ");
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
