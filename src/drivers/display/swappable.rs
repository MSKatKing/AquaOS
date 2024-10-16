use core::fmt::{Display, Formatter, Write};

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

/**
    This trait should be implemented on any struct that acts as a video driver.

    This trait is not implemented fully, and only supports VGA textmode operations.
    Drawing pixels to the screen is not supported.

    The trait `core::fmt::Write` MUST be implemented, specifically the `write_str()`
    method, as that is the base method used to write to the screen.
**/
pub trait VideoDriver: Write {
    /// This function is used to create the graphics driver. DO NOT push to
    /// any ports in this function. This function is only used to create
    /// and data that may be in the struct.
    ///
    /// The result is whether creating the driver failed or was successful.
    /// If the driver failed to create, the &'static str should contain the
    /// error message.
    ///
    /// This method is never called directly by the user, it is only called in
    /// macros provided by the OS.
    ///
    /// Due to Rust's safety system and my inexperience to get around it,
    /// you must declare the value returned by `create()` as a `static mut`.
    /// This is why the function is marked as an `unsafe fn`.
    ///
    /// If you don't do this, Rust will throw errors about lifetimes not lasting
    /// long enough.
    ///
    /// # Examples
    ///
    /// ```
    /// impl VideoDriver for MyVideoDriver {
    ///     ...
    ///
    ///     fn create() -> Result<&'static mut Self, &'static str> {
    ///         // You must use the type here instead of `Self` because statics are not
    ///         // attached to the type they are declared in
    ///         static mut DEFAULT: MyVideoDriver = MyVideoDriver { ... };
    ///         Ok(&mut DEFAULT)
    ///     }
    ///
    ///     ...
    /// }
    /// ```
    unsafe fn create() -> Result<&'static mut Self, &'static str> where Self: Sized;

    /// This function is used to switch the video mode to which ever mode
    /// the driver desires. It can safely call functions from itself
    /// as it is now the current driver being used by the system.
    ///
    /// The Result returned tells the operating system if the driver
    /// was successfully initialized or not. If not, the OS will remove
    /// the driver and revert to the previous one. The &str should contain
    /// the error message to display when the method fails.
    ///
    /// If this method returns `Ok(())` at all during the runtime of the OS,
    /// it's output is not expected to change for the remainder of the
    /// runtime.
    ///
    /// This method is never called directly by the user, it is only called in
    /// macros provided by the OS.
    fn initialize(&self) -> Result<(), &str>;

    /// The output of this function is the name of the driver.
    ///
    /// Used for user-interface contexts only.
    ///
    /// This method is automatically implemented with the value
    /// `"Unknown Driver"`
    ///
    /// # Example
    ///
    /// ```
    /// impl VideoDriver for MyVideoDriver {
    ///     ...
    ///
    ///     fn name(&self) -> &str {
    ///         "80x25 VGA Textmode Driver"
    ///     }
    ///
    ///     ...
    /// }
    /// ```
    fn name(&self) -> &str { "Unknown Driver" }

    /// The output of this function represents the name (&str) and the id (u32)
    /// of the modes that this driver supports. The id is the same as the
    /// integer value that is inputted into the port to change the display
    /// mode to the associated mode.
    ///
    /// If the device does not support any of the display modes that the driver
    /// does, the OS will not display the driver in supported drivers.
    ///
    /// # Example
    ///
    /// ```
    /// impl VideoDriver for MyVideoDriver {
    ///     ...
    ///
    ///     fn supported_display_modes(&self) -> &[(&str, u16)] {
    ///         &[
    ///             ("80x25 VGA Textmode Driver (16 color)", 0x03),
    ///             ("80x25 VGA Textmode Driver (mono color)", 0x07),
    ///         ]
    ///     }
    ///
    ///     ...
    /// }
    /// ```
    fn supported_display_modes(&self) -> &[(&str, VideoModeSpecification)];

    /// This function should clear the screen of all characters and should
    /// move the cursor to the beginning of the screen.
    fn clear_screen(&self);

    /// This function sets the color of the text that will be printed.
    ///
    /// This is a TEXTMODE ONLY method.
    /// This method is not required to be implemented by drivers.
    fn set_color(&mut self, _foreground: VgaColor, _background: VgaColor) {}
}

// Todo: comments on these
#[derive(Debug)]
pub struct VideoModeSpecification {
    textmode: bool,
    width: u32,
    height: u32,
    bits_per_pixel: u8
}

impl Display for VideoModeSpecification {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("Video Mode Specification: \n\tTextmode: {}\n\tWidth: {}\n\tHeight: {}\n\tBits Per Pixel: {}\n", self.textmode, self.width, self.height, self.bits_per_pixel))
    }
}

impl VideoModeSpecification {
    pub const fn new() -> Self {
        Self {
            textmode: false,
            width: 0,
            height: 0,
            bits_per_pixel: 0
        }
    }

    pub const fn from(textmode: bool, width: u32, height: u32, bits_per_pixel: u8) -> Self {
        Self {
            textmode,
            width,
            height,
            bits_per_pixel
        }
    }

    pub const fn with_textmode(self, textmode: bool) -> Self {
        Self {
            textmode,
            .. self
        }
    }
    pub const fn with_width(self, width: u32) -> Self {
        Self {
            width,
            .. self
        }
    }
    pub const fn with_height(self, height: u32) -> Self {
        Self {
            height,
            .. self
        }
    }
    pub const fn with_bits_per_pixel(self, bits_per_pixel: u8) -> Self {
        Self {
            bits_per_pixel,
            .. self
        }
    }

}