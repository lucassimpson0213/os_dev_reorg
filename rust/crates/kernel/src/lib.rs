#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
pub mod drivers;
pub mod idt;
pub mod memory;
pub mod qemu;

use uart_16550::SerialPort;

use core::{ffi::c_void, panic::PanicInfo};

use crate::memory::memory_parsing;

extern crate alloc;

use linked_list_allocator::LockedHeap;

#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
fn oom(_: core::alloc::Layout) -> ! {
    panic!("out of memory");
}

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
    crate::memory::memory_parsing::rust_parse_multiboot_mapper(magic, mbi_phys);
    unsafe { SerialPort::new(0x3F8) }.send(b'X');

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
