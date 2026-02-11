use core::fmt::{self, Write};
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

struct KernelWriter;

impl core::fmt::Write for KernelWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        serial_write_string(s);
        Ok(())
    }
}
pub fn _kprint(args: core::fmt::Arguments) {
    KernelWriter.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ($crate::_kprint(format_args!($($arg)*)));
}

// 4. Implement the print function
