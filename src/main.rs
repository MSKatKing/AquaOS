#![no_std]
#![no_main]
#![allow(static_mut_refs)]
#![feature(naked_functions)]
#![feature(allocator_api)]

pub mod drivers;
mod memory;

extern crate alloc;

use core::arch::asm;
use core::panic::PanicInfo;
use kernel_proc::interrupt;
use crate::drivers::display::{VideoModes, VIDEO_MODE};
use crate::drivers::display::vga_textmode::VGABufferWriter;
use crate::drivers::idt::IDT;
use crate::drivers::ports::Port;
use crate::drivers::timing::configure_pit;

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

    p21.write(0x00u8);
    pa1.write(0x00u8);

    Port::new(0x64).write(0xAEu8);
    Port::new(0x60).write(0xF4u8);

    let mut idt = IDT::new();
    idt.register_default_isr();
    idt.load();

    configure_pit();

    println!("Welcome to the AquaOS kernel!");
    println!("Type '?' for a list of commands.");
    print!("> ");

    timeout!(|| false, Duration::from_millis(50));

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    eprintln!("\n{}", info);
    loop {}
}

#[interrupt]
fn help() {

}
