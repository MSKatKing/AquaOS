pub mod vga_textmode;

use crate::drivers::display::vga_textmode::VGABufferWriter;

pub enum VideoModes {
    VgaTextmode(VGABufferWriter)
}

pub static mut VIDEO_MODE: VideoModes = VideoModes::VgaTextmode(VGABufferWriter::new());

#[macro_export]
macro_rules! write_tts {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        use crate::drivers::display::VIDEO_MODE;
        use crate::drivers::display::VideoModes;

        unsafe {
            match &mut VIDEO_MODE {
                VideoModes::VgaTextmode(ref mut writer) => writer.write_fmt(format_args!($($arg)*)).unwrap(),

            }
        }
    });
}

#[macro_export]
macro_rules! print {
    ($fmt:expr) => (write_tts!($fmt));
    ($fmt:expr, $($arg:tt)*) => (write_tts!($fmt, $($arg)*));
}

#[macro_export]
macro_rules! println {
    () => (write_tts!("\n"));
    ($fmt:expr) => (write_tts!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (write_tts!(concat!($fmt, "\n"), $($arg)*));
}