use kernel_proc::interrupt;
use crate::drivers::ports::{inb, outb};
use crate::print;

pub mod keyboard;

const SCANCODE_LOOKUP: [char; 0x54] = [
    '\0', '\0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', '\0',
    '\0', 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']',
    '\n',
    '\0',
    'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', '\'', '`',
    '\0',
    '\\',
    'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/', '\0',
    '*',
    '\0', ' ',
    '\0',
    '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0','\0',
    '\0',
    '\0',
    '\0', '\0', '\0',
    '-',
    '\0', '\0', '\0', '+',
    '\0', '\0', '\0',
    '\0', '.'
];

const KEY_RELEASE_MASK: u8 = 0x80;

#[interrupt]
fn keyboard_isr() {
    outb(0x20, 0x20);

    let scancode = inb(0x60);

    if scancode & KEY_RELEASE_MASK != 0 {
        return;
    }

    if SCANCODE_LOOKUP[scancode as usize] != '\0' {
        print!("{}", SCANCODE_LOOKUP[scancode as usize]);
    }
}