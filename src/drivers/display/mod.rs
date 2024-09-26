pub mod vga_textmode;

use core::fmt::Write;
use crate::drivers::display::vga_textmode::{VGABufferWriter, VgaColor};

pub enum VideoModes {
    VgaTextmode(VGABufferWriter),
    Uninitialized
}

impl VideoModes {
    pub fn set_color(&mut self, fg: VgaColor, bg: VgaColor) {
        match self {
            VideoModes::VgaTextmode(writer) => writer.set_color(fg, bg),
            VideoModes::Uninitialized => {}
        }
    }
}

impl Write for VideoModes {
    fn write_str(&mut self, args: &str) -> core::fmt::Result {
        match self {
            VideoModes::VgaTextmode(writer) => writer.write_str(args),
            VideoModes::Uninitialized => Ok(())
        }
    }
}

pub static mut VIDEO_MODE: VideoModes = VideoModes::Uninitialized;

#[macro_export]
macro_rules! write_tts {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        use crate::drivers::display::VIDEO_MODE;

        unsafe {
            VIDEO_MODE.write_fmt(format_args!($($arg)*)).unwrap()
        }
    });
}

#[macro_export]
macro_rules! print {
    ($fmt:expr) => (crate::write_tts!($fmt));
    ($fmt:expr, $($arg:tt)*) => (crate::write_tts!($fmt, $($arg)*));
}

#[macro_export]
macro_rules! println {
    () => (crate::write_tts!("\n"));
    ($fmt:expr) => (crate::write_tts!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (crate::write_tts!(concat!($fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! eprint {
    ($fmt:expr) => (unsafe {
        use crate::print;
        use crate::drivers::display::VIDEO_MODE;
        use drivers::display::vga_textmode::VgaColor;

        VIDEO_MODE.set_color(VgaColor::LightRed, VgaColor::Black);
        print!($fmt);
        VIDEO_MODE.set_color(VgaColor::LightGray, VgaColor::Black);
    });
    ($fmt:expr, $($arg:tt)*) => (unsafe {
        use crate::print;
        use crate::drivers::display::VIDEO_MODE;
        use drivers::display::vga_textmode::VgaColor;

        VIDEO_MODE.set_color(VgaColor::LightRed, VgaColor::Black);
        print!($fmt, $($arg)*);
        VIDEO_MODE.set_color(VgaColor::LightGray, VgaColor::Black);
    });
}

#[macro_export]
macro_rules! eprintln {
    () => (println!());
    ($fmt:expr) => (unsafe {
        use crate::println;
        use crate::drivers::display::VIDEO_MODE;
        use drivers::display::vga_textmode::VgaColor;

        VIDEO_MODE.set_color(VgaColor::LightRed, VgaColor::Black);
        println!($fmt);
        VIDEO_MODE.set_color(VgaColor::LightGray, VgaColor::Black);
    });
    ($fmt:expr, $($arg:tt)*) => (unsafe {
        use crate::println;
        use crate::drivers::display::VIDEO_MODE;
        use drivers::display::vga_textmode::VgaColor;

        VIDEO_MODE.set_color(VgaColor::LightRed, VgaColor::Black);
        println!($fmt, $($arg)*);
        VIDEO_MODE.set_color(VgaColor::LightGray, VgaColor::Black);
    });
}

#[macro_export]
macro_rules! wprint {
    ($fmt:expr) => (unsafe {
        use crate::print;
        use crate::drivers::display::VIDEO_MODE;
        use drivers::display::vga_textmode::VgaColor;

        VIDEO_MODE.set_color(VgaColor::Yellow, VgaColor::Black);
        print!($fmt);
        VIDEO_MODE.set_color(VgaColor::LightGray, VgaColor::Black);
    });
    ($fmt:expr, $($arg:tt)*) => (unsafe {
        use crate::print;
        use crate::drivers::display::VIDEO_MODE;
        use drivers::display::vga_textmode::VgaColor;

        VIDEO_MODE.set_color(VgaColor::Yellow, VgaColor::Black);
        print!($fmt, $($arg)*);
        VIDEO_MODE.set_color(VgaColor::LightGray, VgaColor::Black);
    });
}

#[macro_export]
macro_rules! wprintln {
    () => (println!());
    ($fmt:expr) => (unsafe {
        use crate::println;
        use crate::drivers::display::VIDEO_MODE;
        use drivers::display::vga_textmode::VgaColor;

        VIDEO_MODE.set_color(VgaColor::Yellow, VgaColor::Black);
        println!($fmt);
        VIDEO_MODE.set_color(VgaColor::LightGray, VgaColor::Black);
    });
    ($fmt:expr, $($arg:tt)*) => (unsafe {
        use crate::println;
        use crate::drivers::display::VIDEO_MODE;
        use drivers::display::vga_textmode::VgaColor;

        VIDEO_MODE.set_color(VgaColor::Yellow, VgaColor::Black);
        println!($fmt, $($arg)*);
        VIDEO_MODE.set_color(VgaColor::LightGray, VgaColor::Black);
    });
}