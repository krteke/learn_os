#![no_std]
#![no_main]

mod console;
mod sbi;
mod uart;

use core::arch::{asm, global_asm};

use crate::{sbi::reboot, uart::UART0_BASE};

global_asm!(include_str!("entry.asm"));

macro_rules! linker_symbol_addr {
    ($symbol:path) => {
        ($symbol as *const ()).addr()
    };
}

fn clear_bss() {
    unsafe extern "C" {
        safe fn sbss();
        safe fn ebss();
    }
    (linker_symbol_addr!(sbss)..linker_symbol_addr!(ebss))
        .for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

#[unsafe(no_mangle)]
pub extern "C" fn kmain(hart_id: usize, _dtb_addr: usize) -> ! {
    clear_bss();

    let uart = uart::Uart::new(UART0_BASE);
    uart.init();

    println!();
    println!("========================================\n");
    println!("  MyOS v0.1.0\n");
    println!("  RISC-V 64-bit — Written in Rust\n");
    println!("========================================\n");
    println!();

    println!("Booted on hart {}", hart_id);
    println!("Hello from MyOS!\n");
    println!();

    loop {
        if let Some(c) = uart.getc() {
            match c {
                8 => print!("{} {}", c as char, c as char),
                b'\n' | b'\r' => println!(),
                b'r' => {
                    reboot();
                }
                _ => print!("{}", c as char),
            }
        }
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!();
    println!("!!! KERNEL PANIC !!!");

    if let Some(location) = info.location() {
        println!(
            "  at {}:{}:{}",
            location.file(),
            location.line(),
            location.column()
        );
    }

    if let Some(message) = info.message().as_str() {
        println!("  {}", message);
    } else {
        println!("  {}", info.message());
    }

    println!();
    println!("System halted.");

    loop {
        unsafe { asm!("wfi") };
    }
}
