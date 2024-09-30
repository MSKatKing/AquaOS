#![no_std]
#![no_main]
#![allow(static_mut_refs)]
#![feature(naked_functions)]

pub mod drivers;
mod memory;

use core::panic::PanicInfo;
use kernel_proc::interrupt;
use crate::drivers::display::{VideoModes, VIDEO_MODE};
use crate::drivers::display::vga_textmode::VGABufferWriter;
use crate::drivers::idt::IDT;
use crate::drivers::ports::Port;
use crate::drivers::timing::{configure_pit, current_time};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe { VIDEO_MODE = VideoModes::VgaTextmode(VGABufferWriter::new()); }

    // Remap the PIC to make sure that it uses the correct ISR handlers
    Port::new(0x20).write(0x11u8);
    Port::new(0xA0).write(0x11u8);

    let p21 = Port::new(0x21);
    let pa1 = Port::new(0xA1);

    p21.write(0x20u8);
    pa1.write(0x28u8);
    p21.write(0x04u8);
    pa1.write(0x02u8);
    p21.write(0x01u8);
    pa1.write(0x01u8);

    p21.write(0xFCu8);
    pa1.write(0xFFu8);

    Port::new(0x64).write(0xAEu8);
    Port::new(0x60).write(0xF4u8);

    let mut idt = IDT::new();
    idt.register_default_isr();
    idt.load();

    configure_pit();

    println!("Welcome to the AquaOS kernel!");
    println!("Type '?' for a list of commands.");
    print!("> ");

    let (seconds, minutes, hours, day, month, year) = current_time();

    println!("Current Time: {:02}:{:02}:{:02} - Date: {:02}/{:02}/{:04}",
        hours, minutes, seconds, month, day, year);

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
