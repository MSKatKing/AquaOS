use core::arch::asm;

pub trait PortRead {
    fn port_read(port: u16) -> Self;
}

pub trait PortWrite {
    fn port_write(&self, port: u16);
}

impl PortRead for u8 {
    #[inline]
    fn port_read(port: u16) -> u8 {
        let result: u8;
        unsafe {
            asm!("in al, dx", in("dx") port, out("al") result, options(nomem, nostack, preserves_flags));
        }
        result
    }
}

impl PortWrite for u8 {
    #[inline]
    fn port_write(&self, port: u16) {
        unsafe {
            asm!("out dx, al", in("dx") port, in("al") *self, options(nomem, nostack, preserves_flags));
        }
    }
}

impl PortRead for u16 {
    #[inline]
    fn port_read(port: u16) -> u16 {
        let result: u16;
        unsafe {
            asm!(
            "in ax, dx",
            in("dx") port,
            out("ax") result,
            options(nomem, nostack, preserves_flags)
            );
        }
        result
    }
}

impl PortWrite for u16 {
    #[inline]
    fn port_write(&self, port: u16) {
        unsafe {
            asm!("out dx, ax", in("dx") port, in("ax") *self, options(nomem, nostack, preserves_flags));
        }
    }
}

pub struct Port {
    port: u16
}

impl Port {
    pub const fn new(port: u16) -> Port {
        Port { port }
    }

    pub fn write<T: PortWrite>(&self, value: T) {
        value.port_write(self.port);
    }

    pub fn read<T: PortRead>(&self) -> T {
        T::port_read(self.port)
    }
}