/*
*:
* extern unsigned int rust_parse_multiboot_map(uint32_t magic, uint32_t mbi_phys);
*
*/

use multiboot::information::MemoryManagement;
use uart_16550::SerialPort;

use crate::qemu::{self, qemu_testing};

const MB1_BOOTLOADER_MAGIC: u32 = 0x2BADB002;
const MB2_BOOTLOADER_MAGIC: u32 = 0x36D76289;

/*
*  The first 8 bytes are the header
*  and then the tags start
*  the last ending tag is 8 bytes so the minimum size
*  it can be is 16 bytes
*/

mod multiboot_implementation {

    use multiboot::information::PAddr;

    pub trait PhysRead {
        unsafe fn paddr_to_slice(&self, paddr: u64, len: usize) -> Option<&'static [u8]>;
    }

    pub fn physical_addr_to_mb(address: PAddr) {}
}
pub fn rust_parse_multiboot_mapper(magic: u32, mbi_phys: u32) -> u32 {
    if magic == MB1_BOOTLOADER_MAGIC || magic == MB2_BOOTLOADER_MAGIC {
        printings::serial_write_string("boot loader is indeed activated :)");
    }
    match magic {
        MB1_BOOTLOADER_MAGIC => {
            printings::serial_write_string("kernel is using multiboot 1");

            let map = crate::memory::multiboot_helper::use_multiboot(mbi_phys, mem);
        }
        MB2_BOOTLOADER_MAGIC => {
            printings::serial_write_string("kernel is using multiboot 2");
        }
        _ => {
            printings::serial_write_string("multi boot has not been idenified");
        }
    }

    0
}

mod printings {
    use uart_16550::SerialPort;

    pub fn serial_write_byte(b: u8) {
        let mut serial = unsafe { SerialPort::new(0x3F8) };
        serial.init();
        unsafe {
            serial.send(b);
        }
    }

    pub fn serial_write_string(s: &str) {
        let mut serial = unsafe { SerialPort::new(0x3F8) };
        serial.init();

        for b in s.bytes() {
            unsafe {
                serial.send(b);
            }
        }
    }

    pub fn serial_newline() {
        serial_write_byte(b'\r');
        serial_write_byte(b'\n');
    }
}
mod multiboot2 {}
