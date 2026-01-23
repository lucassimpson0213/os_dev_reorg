use crate::serial_println;
use core::ffi::c_void;

unsafe extern "C" {
    pub fn printk(fmt: *const u8, ...) -> c_void;
}

/*
* builder pattern for idt entries using method chaining
*
*
*/

#[repr(C, packed)]
pub struct IdtEntry {
    isr_low: u16,
    kernel_cs: u16,
    reserved: u8,
    attributes: u8,
    isr_mid: u16,
    isr_high: u32,
    reserved_two: u32,
}

impl IdtEntry {
    pub fn new() -> Self {
        return Self {
            isr_low: 0,
            kernel_cs: 0,
            reserved: 0,
            attributes: 0,
            isr_mid: 0,
            isr_high: 0,
            reserved_two: 0,
        };
    }

    pub fn isr(mut self, isr_routine: u32) -> Self {
        let low_bits = isr_routine & 0xFFFF;
        serial_println!("hello");
        self
    }
}
