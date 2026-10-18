use core::arch::asm;
#[cfg(feature = "duos")]
use core::ptr;

use crate::println;

const SBI_SRST: usize = 0x53525354;
const SBI_SYSTEM_RESET: usize = 0;

const RESET_COLD_REBOOT: usize = 1;
const RESET_NO_REASON: usize = 0;

fn sbi_call(eid: usize, fid: usize, arg0: usize, arg1: usize, arg2: usize) -> (isize, isize) {
    let error;
    let value;

    unsafe {
        asm!(
            "ecall",
            inlateout("a0") arg0 => error,
            inlateout("a1") arg1 => value,
            in("a2") arg2,
            in("a6") fid,
            in("a7") eid,
        );
    }

    (error, value)
}

#[cfg(feature = "qemu")]
pub fn reboot() {
    let (error, _) = sbi_call(
        SBI_SRST,
        SBI_SYSTEM_RESET,
        RESET_COLD_REBOOT,
        RESET_NO_REASON,
        0,
    );

    println!("SBI reboot failed: {}", error);
}

#[cfg(feature = "duos")]
pub fn reboot() -> ! {
    const RTC_CTRL: usize = 0x0502_5000;
    const RTC_CORE: usize = 0x0502_6000;

    unsafe {
        ptr::write_volatile((RTC_CORE + 0xCC) as *mut u32, 1);

        ptr::write_volatile((RTC_CTRL + 0x04) as *mut u32, 0xAB18);

        ptr::write_volatile((RTC_CTRL + 0x08) as *mut u32, 0xFFFF_0810);
    }

    loop {
        core::hint::spin_loop();
    }
}
