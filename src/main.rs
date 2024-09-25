#![no_std]
#![no_main]
#![allow(static_mut_refs)]

pub mod drivers;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, world!");
    println!("Hello, world!");
    print!("Hello, world! {}", 5);

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
