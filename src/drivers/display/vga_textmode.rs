use core::fmt;

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

pub struct VGABufferWriter {
    column_position: usize,
    row_position: usize,
    color_code: u8
}

impl VGABufferWriter {
    pub const fn new() -> Self {
        VGABufferWriter {
            column_position: 0,
            row_position: 0,
            color_code: 0xb
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

        if self.row_position < BUFFER_HEIGHT - 1 {
            self.row_position += 1;
        } else {
            self.row_position = 0;
        }
    }
}

impl fmt::Write for VGABufferWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}