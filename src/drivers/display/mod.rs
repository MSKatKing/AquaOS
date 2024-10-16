pub mod vga_textmode;
pub mod swappable;

use core::fmt::Write;
use crate::drivers::display::swappable::{VideoDriver, VideoModeSpecification};

pub struct NullVideoDriver;

impl Write for NullVideoDriver {
    fn write_str(&mut self, _s: &str) -> core::fmt::Result {
        Ok(())
    }
}

impl VideoDriver for NullVideoDriver {
    unsafe fn create() -> Result<&'static mut Self, &'static str> { Err("Cannot create a null video driver.") }
    fn initialize(&self) -> Result<(), &str> { Err("Cannot initialize a null video driver.") }
    fn name(&self) -> &str { "Null Video Driver (os-provided)" }
    fn supported_display_modes(&self) -> &[(&str, VideoModeSpecification)] { &[] }
    fn clear_screen(&self) { }
}

pub static mut VIDEO_DRIVER: &'static mut dyn VideoDriver = &mut NullVideoDriver;

/// This macro will change the video driver to the struct passed in through $driver.
///
/// If the `create()` or `initialize()` methods of the desired driver fail in any way,
/// the driver will remain the current driver.
#[allow(unused_unsafe)]
#[macro_export]
macro_rules! change_video_driver {
    ($driver:ident) => {
        unsafe {
            use crate::eprintln;
            use crate::drivers::display::swappable::VideoDriver;
            use core::any::type_name;

            // Todo: check if the driver supports any of the modes supported by the system
            let driver = $driver::create();

            if let Ok(driver) = driver {
                let previous_driver: &'static mut dyn VideoDriver = VIDEO_DRIVER;
                VIDEO_DRIVER = driver;
                if let Err(msg) = VIDEO_DRIVER.initialize() {
                    VIDEO_DRIVER = previous_driver;

                    /*
                    ** If the driver is a null driver, then this will fail. This is ok, however, because the null driver
                    ** represents a driver that does not exist and henceforth does not need to initialize anything.
                    **
                    ** As per the documentation for the `initialize()` method, if it is successful once during the runtime
                    ** of the OS, it's output will always be successful for the remainder of the runtime of the OS.
                    */
                    let _ = VIDEO_DRIVER.initialize();
                    eprintln!("Failed to switch driver to {:?}: Initializing driver failed: {}. Driver reverted to previous driver.", type_name::<$driver>(), msg);
                }
            } else {
                eprintln!("Failed to create driver {:?}. Creating driver failed: {}", type_name::<$driver>(), driver.err().unwrap());
            }
        }
    };
}

#[macro_export]
macro_rules! write_tts {
    ($($arg:tt)*) => ({
        use crate::drivers::display::VIDEO_DRIVER;

        VIDEO_DRIVER.write_fmt(format_args!($($arg)*)).unwrap_or(())
    });
}

#[macro_export]
macro_rules! print {
    ($fmt:expr) => (#[allow(unused_unsafe)] unsafe { crate::write_tts!($fmt) });
    ($fmt:expr, $($arg:tt)*) => (#[allow(unused_unsafe)] unsafe { crate::write_tts!($fmt, $($arg)*) });
}

#[macro_export]
macro_rules! println {
    () => (#[allow(unused_unsafe)] unsafe { crate::write_tts!("\n") });
    ($fmt:expr) => (#[allow(unused_unsafe)] unsafe { crate::write_tts!(concat!($fmt, "\n")) });
    ($fmt:expr, $($arg:tt)*) => (#[allow(unused_unsafe)] unsafe { crate::write_tts!(concat!($fmt, "\n"), $($arg)*) });
}

#[macro_export]
macro_rules! eprint {
    ($fmt:expr) => (#[allow(unused_unsafe)] unsafe {
        use crate::print;
        use crate::drivers::display::VIDEO_DRIVER;
        use drivers::display::swappable::VgaColor;

        VIDEO_DRIVER.set_color(VgaColor::LightRed, VgaColor::Black);
        print!($fmt);
        VIDEO_DRIVER.set_color(VgaColor::LightGray, VgaColor::Black);
    });
    ($fmt:expr, $($arg:tt)*) => (#[allow(unused_unsafe)] unsafe {
        use crate::print;
        use crate::drivers::display::VIDEO_DRIVER;
        use drivers::display::swappable::VgaColor;

        VIDEO_DRIVER.set_color(VgaColor::LightRed, VgaColor::Black);
        print!($fmt, $($arg)*);
        VIDEO_DRIVER.set_color(VgaColor::LightGray, VgaColor::Black);
    });
}

#[macro_export]
macro_rules! eprintln {
    () => (println!());
    ($fmt:expr) => (#[allow(unused_unsafe)] unsafe {
        use crate::println;
        use crate::drivers::display::VIDEO_DRIVER;
        use drivers::display::swappable::VgaColor;

        VIDEO_DRIVER.set_color(VgaColor::LightRed, VgaColor::Black);
        println!($fmt);
        VIDEO_DRIVER.set_color(VgaColor::LightGray, VgaColor::Black);
    });
    ($fmt:expr, $($arg:tt)*) => (#[allow(unused_unsafe)] unsafe {
        use crate::println;
        use crate::drivers::display::VIDEO_DRIVER;
        use drivers::display::swappable::VgaColor;

        VIDEO_DRIVER.set_color(VgaColor::LightRed, VgaColor::Black);
        println!($fmt, $($arg)*);
        VIDEO_DRIVER.set_color(VgaColor::LightGray, VgaColor::Black);
    });
}

#[macro_export]
macro_rules! wprint {
    ($fmt:expr) => (#[allow(unused_unsafe)] unsafe {
        use crate::print;
        use crate::drivers::display::VIDEO_DRIVER;
        use drivers::display::swappable::VgaColor;

        VIDEO_DRIVER.set_color(VgaColor::Yellow, VgaColor::Black);
        print!($fmt);
        VIDEO_DRIVER.set_color(VgaColor::LightGray, VgaColor::Black);
    });
    ($fmt:expr, $($arg:tt)*) => (#[allow(unused_unsafe)] unsafe {
        use crate::print;
        use crate::drivers::display::VIDEO_DRIVER;
        use drivers::display::swappable::VgaColor;

        VIDEO_DRIVER.set_color(VgaColor::Yellow, VgaColor::Black);
        print!($fmt, $($arg)*);
        VIDEO_DRIVER.set_color(VgaColor::LightGray, VgaColor::Black);
    });
}

#[macro_export]
macro_rules! wprintln {
    () => (println!());
    ($fmt:expr) => (#[allow(unused_unsafe)] unsafe {
        use crate::println;
        use crate::drivers::display::VIDEO_DRIVER;
        use drivers::display::swappable::VgaColor;

        VIDEO_DRIVER.set_color(VgaColor::Yellow, VgaColor::Black);
        println!($fmt);
        VIDEO_DRIVER.set_color(VgaColor::LightGray, VgaColor::Black);
    });
    ($fmt:expr, $($arg:tt)*) => (#[allow(unused_unsafe)] unsafe {
        use crate::println;
        use crate::drivers::display::VIDEO_DRIVER;
        use drivers::display::swappable::VgaColor;

        VIDEO_DRIVER.set_color(VgaColor::Yellow, VgaColor::Black);
        println!($fmt, $($arg)*);
        VIDEO_DRIVER.set_color(VgaColor::LightGray, VgaColor::Black);
    });
}