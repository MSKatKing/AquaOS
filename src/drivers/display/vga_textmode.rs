use core::fmt;
use crate::drivers::ports::outb;

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

const CURSOR_CONTROL_PORT: u16 = 0x3D4;
const CURSOR_DATA_PORT: u16 = 0x3D5;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum VgaColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

pub struct VGABufferWriter {
    column_position: usize,
    row_position: usize,
    color_code: u8,
    start_row: u16
}

impl VGABufferWriter {
    pub fn new() -> Self {
        Self::disable_cursor();
        Self::enable_cursor();

        VGABufferWriter {
            column_position: 0,
            row_position: 0,
            color_code: 0x07,
            start_row: 0
        }
    }

    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let offset = self.row_position * BUFFER_WIDTH + self.column_position;

                unsafe {
                    *VGA_BUFFER.offset(offset as isize * 2) = byte;
                    *VGA_BUFFER.offset(offset as isize * 2 + 1) = self.color_code;
                }

                self.column_position += 1;
            }
        }

        let offset = (self.row_position * BUFFER_WIDTH + self.column_position) as u16;
        outb(CURSOR_CONTROL_PORT, 0x0E);
        outb(CURSOR_DATA_PORT, (offset >> 8) as u8);
        outb(CURSOR_CONTROL_PORT, 0x0F);
        outb(CURSOR_DATA_PORT, offset as u8);
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        self.column_position = 0;
        self.row_position += 1;

        if self.row_position > self.start_row as usize + BUFFER_HEIGHT - 1 {
            self.scroll_down();
        }
    }

    pub fn set_color(&mut self, foreground: VgaColor, background: VgaColor) {
        self.color_code = ((background as u8 & 0xF) << 4) | (foreground as u8 & 0xF);
    }

    fn scroll_up(&mut self) -> bool {
        if self.start_row > 0 {
            false
        } else {
            self.start_row -= 1;

            self.set_offset();

            true
        }
    }

    fn scroll_down(&mut self) {
        self.start_row += 1;
        self.set_offset();
    }

    fn set_offset(&self) {
        let new = 80u16 * self.start_row;
        let high = (new >> 8) as u8;
        let low = (new & 0xFF) as u8;

        outb(0x3D4, 0x0C);
        outb(0x3D5, high);
        outb(0x3D4, 0x0D);
        outb(0x3D5, low);
    }

    fn enable_cursor() {
        outb(CURSOR_CONTROL_PORT, 0x0A);
        outb(CURSOR_DATA_PORT, 0);
        outb(CURSOR_CONTROL_PORT, 0x0B);
        outb(CURSOR_DATA_PORT, 16);
    }

    fn disable_cursor() {
        outb(CURSOR_CONTROL_PORT, 0x0A);
        outb(CURSOR_DATA_PORT, 0x20);
    }
}

impl fmt::Write for VGABufferWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}