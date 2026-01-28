#![no_std]
pub mod drivers;
pub mod idt;
pub mod memory;

use idt::makeidt::IdtEntry;

use core::{ffi::c_void, panic::PanicInfo};

#[unsafe(no_mangle)]
pub extern "C" fn rust_hello() -> u32 {
    0xCAFEBABE
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_idt_entry() -> u32 {
    return 0;
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_ping() -> u32 {
    0xC0FFEEu32
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_parse_multiboot_map(magic: u32, mbi_phys: u32) -> u32 {
    return 0;
}
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::drivers::serial::_print(core::format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (
        $crate::serial_print!(concat!($fmt, "\n"), $($arg)*)
    );
}
