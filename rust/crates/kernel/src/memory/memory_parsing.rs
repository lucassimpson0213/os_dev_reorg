/*
*:
* extern unsigned int rust_parse_multiboot_map(uint32_t magic, uint32_t mbi_phys);
*
*/

use uart_16550::SerialPort;

const MB1_BOOTLOADER_MAGIC: u32 = 0x2BADB002;
const MB2_BOOTLOADER_MAGIC: u32 = 0x36D76289;

/*
*  The first 8 bytes are the header
*  and then the tags start
*  the last ending tag is 8 bytes so the minimum size
*  it can be is 16 bytes
*/

#[macro_export]
macro_rules! kassert {
    ($cond:expr, $msg:expr) => {{
        if !$cond {
            $crate::printing::serial_write_string("ASSERT FAILED: ");
            $crate::printing::serial_write_string($msg);
            $crate::printing::serial_write_string(" @ ");
            $crate::printing::serial_write_string(file!());
            $crate::printing::serial_write_string(":");
            $crate::printing::serial_write_string(stringify!(line!()));
            $crate::printing::serial_newline();

            // Choose whatever failure code you want; keep it consistent.
            $crate::ktesting::qemu_exit(0x11);
        }
    }};
}

pub fn rust_parse_multiboot_mapper(magic: u32, mbi_phys: u32) -> u32 {
    if magic == MB1_BOOTLOADER_MAGIC || magic == MB2_BOOTLOADER_MAGIC {
        let mut serial = unsafe { SerialPort::new(0x3F8) };
        serial.init();
        serial.send(b'L');
    }

    let multiboot_structure = mbi_phys as *const u8;
    let total_size = unsafe { *multiboot_structure };
    let reserved = unsafe { multiboot_structure.add(4) };
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
