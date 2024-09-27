#![no_std]
#![no_main]
#![allow(static_mut_refs)]
#![feature(naked_functions)]

pub mod drivers;

use core::panic::PanicInfo;
use kernel_proc::interrupt;
use crate::drivers::display::{VideoModes, VIDEO_MODE};
use crate::drivers::display::vga_textmode::VGABufferWriter;
use crate::drivers::idt::IDT;
use crate::drivers::ports::{inb, outb};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe { VIDEO_MODE = VideoModes::VgaTextmode(VGABufferWriter::new()); }

    // Remap the PIC to make sure that it uses the correct ISR handlers
    outb(0x20, 0x11);
    outb(0xA0, 0x11);

    outb(0x21, 0x20);
    outb(0xA1, 0x28);
    outb(0x21, 0x04);
    outb(0xA1, 0x02);
    outb(0x21, 0x01);
    outb(0xA1, 0x01);

    outb(0x21, 0xFD);
    outb(0xA1, 0xFF);

    outb(0x64, 0xAE);
    outb(0x60, 0xF4);

    let mut idt = IDT::new();
    idt.register_default_isr();
    idt.load();

    println!("Welcome to the AquaOS kernel!");
    println!("Type '?' for a list of commands.");
    print!("> ");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    eprintln!("\n{}", info);
    loop {}
}

#[interrupt]
fn help() {

}
