#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

pub mod datastructures;
pub mod drivers;
pub mod idt;
pub mod memory;
pub mod printing;
pub mod qemu;
pub use printing::print::_kprint;

use core::{alloc::Layout, panic::PanicInfo};
use datastructures::linkedlist;
use linked_list_allocator::LockedHeap;

#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    panic!("out of memory");
}

unsafe extern "C" {
    static _heap_start: u8;
    static _heap_end: u8;
}

fn heap_bounds() -> (usize, usize) {
    unsafe {
        let start = (&_heap_start as *const u8) as usize;
        let end = (&_heap_end as *const u8) as usize;
        (start, end)
    }
}

pub fn init_heap() {
    let (start, end) = heap_bounds();
    let size = end - start;

    unsafe {
        HEAP.lock().init(start as *mut u8, size);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn init_heap_rust() {
    init_heap();
    kprint!("{}", 3);
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_hello() -> u32 {
    0xCAFEBABE
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_idt_entry() -> u32 {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_ping() -> u32 {
    0xC0FFEEu32
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
