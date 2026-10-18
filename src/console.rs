use crate::uart::{UART0_BASE, Uart};
use core::fmt;
use core::fmt::Write;

pub fn _print(args: fmt::Arguments) {
    let mut uart = Uart::new(UART0_BASE);
    uart.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
