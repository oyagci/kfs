#![no_std]
#![feature(asm)]

mod keyboard_driver;
mod utils;
mod vga_buffer;

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

fn kreadline(kb: &mut keyboard_driver::Keyboard, s: &str) -> &'static str {
    let mut buffer: &'static str = "";
    let mut pos: usize = 0;
    let mut i: usize = 0;

    print!("{}", s);
    pos += s.len();

    loop {
        match kb.update() {
            Some(s) => match s {
                    b'\n' => {
                        print!("{}", s as char);
                        i = 0;
                        return buffer;
                    },
                    127 => if i > 0 {
                        print!("{}", s as char);
                        i -= 1;
                    },
                    _ => {
                        print!("{}", s as char);
                        i += 1;
                    }
            },
            None => {}
        };
    }
}

#[allow(unused_attributes)]
#[no_mangle]
pub fn kmain() {
    utils::disable_cursor();
    utils::enable_cursor(14, 15);
    let mut kb = keyboard_driver::Keyboard::new();
    let mut s: &'static str;

    print_kernel_logo();

    loop {
        s = kreadline(&mut kb, "$> ");
        println!("{}", s);
    }

}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
