use core::{fmt, ptr};

#[cfg(feature = "qemu")]
pub const UART0_BASE: usize = 0x1000_0000;

#[cfg(feature = "qemu")]
const REG_SHIFT: usize = 0;

#[cfg(feature = "duos")]
pub const UART0_BASE: usize = 0x04140000;

#[cfg(feature = "duos")]
const REG_SHIFT: usize = 2;

pub struct Uart {
    base: usize,
}

impl Uart {
    pub const fn new(base: usize) -> Self {
        Uart { base }
    }

    pub fn init(&self) {
        self.write_reg(3, 0x03);
        self.write_reg(2, 0x01);
        self.write_reg(1, 0x00);
    }

    pub fn putc(&self, c: u8) {
        while self.read_reg(5) & (1 << 5) == 0 {}
        self.write_reg(0, c as u32);
    }

    pub fn puts(&self, s: &str) {
        for byte in s.bytes() {
            if byte == b'\n' {
                self.putc(b'\r');
            }
            self.putc(byte);
        }
    }

    pub fn getc(&self) -> Option<u8> {
        if self.read_reg(5) & 1 != 0 {
            Some(self.read_reg(0) as u8)
        } else {
            None
        }
    }

    fn read_reg(&self, reg: usize) -> u32 {
        let addr = self.base + (reg << REG_SHIFT);

        unsafe {
            #[cfg(feature = "qemu")]
            {
                ptr::read_volatile(addr as *const u8) as u32
            }

            #[cfg(feature = "duos")]
            ptr::read_volatile(addr as *const u32)
        }
    }

    fn write_reg(&self, reg: usize, value: u32) {
        let addr = self.base + (reg << REG_SHIFT);

        unsafe {
            #[cfg(feature = "qemu")]
            ptr::write_volatile(addr as *mut u8, value as u8);

            #[cfg(feature = "duos")]
            ptr::write_volatile(addr as *mut u32, value);
        }
    }
}

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.puts(s);
        Ok(())
    }
}
