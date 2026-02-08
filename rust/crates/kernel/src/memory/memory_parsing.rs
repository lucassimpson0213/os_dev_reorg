/*
*:
* extern unsigned int rust_parse_multiboot_map(uint32_t magic, uint32_t mbi_phys);
*
*/

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

pub fn rust_parse_multiboot_mapper(magic: u32, mbi_phys: u32) -> u32 {
    if magic == MB1_BOOTLOADER_MAGIC || magic == MB2_BOOTLOADER_MAGIC {
        printing::serial_write_string("boot loader is indeed activated :)");
    }
    match magic {
        MB1_BOOTLOADER_MAGIC => {
            printing::serial_write_string("kernel is using multiboot 1");
        }
        MB2_BOOTLOADER_MAGIC => {
            printing::serial_write_string("kernel is using multiboot 2");
        }
        _ => {
            printing::serial_write_string("multi boot has not been idenified");
            let subsys = crate::qemu::qemu_testing::subsys::MEMORY;
            let stage = crate::qemu::qemu_testing::stage::INIT;
            let error = qemu_testing::err::DOUBLE_FAULT;
            let info = 0x01;
            crate::qemu::qemu_testing::qemu_panic(subsys, stage, error, info)
        }
    }

    0
}

mod ktesting {

    unsafe fn outl(port: u16, val: u32) {
        core::arch::asm!("out dx, eax", in("dx") port, in("eax") val);
    }

    pub fn qemu_exit(code: u32) -> ! {
        unsafe {
            outl(0xF4, code);
        }
        loop {
            unsafe {
                core::arch::asm!("hlt");
            }
        }
    }
}

mod printing {
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
