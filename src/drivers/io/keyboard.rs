use alloc::vec::Vec;
use kernel_proc::interrupt;
use crate::drivers::ports::Port;
use crate::print;

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

const SCANCODE_LOOKUP_CAP: [char; 0x54] = [
    '\0', '\0', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '\0',
    '\0', 'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', '{', '}',
    '\0',
    '\0',
    'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', ':', '\"', '~',
    '\0',
    '|',
    'Z', 'X', 'C', 'V', 'B', 'N', 'M', '<', '>', '?', '\0',
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
    '\0', '\0'
];

const KEY_RELEASE_MASK: u8 = 0x80;

static mut RIGHT_SHIFT: bool = false;
static mut LEFT_SHIFT: bool = false;
static mut CAPS_LOCK: bool = false;

#[interrupt]
fn keyboard_isr() {

    Port::new(0x20).write(0x20u8);

    let scancode = Port::new(0x60).read();

    if scancode & KEY_RELEASE_MASK != 0 {
        match scancode & (!KEY_RELEASE_MASK) {
            0x36 => unsafe {
                RIGHT_SHIFT = false;
            }
            0x2A => unsafe {
                LEFT_SHIFT = false;
            }
            _ => {}
        }
        return;
    }

    match scancode {
        0x36 => unsafe {
            RIGHT_SHIFT = true;
        }
        0x2A => unsafe {
            LEFT_SHIFT = true;
        }
        0x3A => unsafe {
            CAPS_LOCK = !CAPS_LOCK;
        }
        0x0E => {
            print!("{}", 8 as char);
        }
        _ => unsafe {
            if SCANCODE_LOOKUP[scancode as usize] != '\0' {
                let char = if (RIGHT_SHIFT || LEFT_SHIFT) ^ CAPS_LOCK { SCANCODE_LOOKUP_CAP[scancode as usize] } else { SCANCODE_LOOKUP[scancode as usize] };

                print!("{}", char);
            }
        }
    }
}