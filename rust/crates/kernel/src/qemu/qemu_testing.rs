#![allow(dead_code)]

use core::arch::asm;

pub const QEMU_DEBUG_PORT: u16 = 0xF4;

/* =========================
Subsystems (high byte)
========================= */
pub mod subsys {
    pub const BOOT: u8 = 0x01;
    pub const GDT: u8 = 0x02;
    pub const IDT: u8 = 0x03;
    pub const PAGING: u8 = 0x04;
    pub const MEMORY: u8 = 0x05;
    pub const INTERRUPT: u8 = 0x06;
    pub const SYSCALL: u8 = 0x07;
}

/* =========================
Stages
========================= */
pub mod stage {
    pub const ENTER: u8 = 0x01;
    pub const INIT: u8 = 0x02;
    pub const LOAD: u8 = 0x03;
    pub const ENABLE: u8 = 0x04;
    pub const HANDLE: u8 = 0x05;
}

/* =========================
Error types
========================= */
pub mod err {
    pub const NONE: u8 = 0x00;
    pub const NULL_PTR: u8 = 0x01;
    pub const BAD_DESCRIPTOR: u8 = 0x02;
    pub const PAGE_FAULT: u8 = 0x03;
    pub const GPF: u8 = 0x04;
    pub const DOUBLE_FAULT: u8 = 0x05;
    pub const BAD_INTERRUPT: u8 = 0x06;
    pub const UNKNOWN: u8 = 0xFF;
}

/* =========================
Low-level port I/O
========================= */
#[inline(always)]
pub unsafe fn outl(port: u16, val: u32) {
    // outl %eax, %dx
    asm!(
        "out dx, eax",
        in("dx") port,
        in("eax") val,
        options(nomem, nostack, preserves_flags),
    );
}

/* =========================
Encode panic
[ subsystem | stage | error | info ]
  8 bits      8 bits   8 bits  8 bits
========================= */
#[inline(always)]
pub const fn panic_code(subsys: u8, stage: u8, error: u8, info: u8) -> u32 {
    ((subsys as u32) << 24) | ((stage as u32) << 16) | ((error as u32) << 8) | (info as u32)
}

/* =========================
Exit to QEMU (isa-debug-exit)
========================= */
#[inline(never)]
pub fn qemu_panic(subsys: u8, stage: u8, error: u8, info: u8) -> ! {
    let code = panic_code(subsys, stage, error, info);

    unsafe {
        outl(QEMU_DEBUG_PORT, code);
    }

    // If QEMU doesn't exit for some reason, halt forever.
    loop {
        unsafe { asm!("hlt", options(nomem, nostack, preserves_flags)) }
    }
}

/// Convenience macro (so callsites are tiny)
#[macro_export]
macro_rules! qemu_panic {
    ($subsys:expr, $stage:expr, $err:expr, $info:expr) => {
        $crate::debug_exit::qemu_panic($subsys, $stage, $err, $info)
    };
}
