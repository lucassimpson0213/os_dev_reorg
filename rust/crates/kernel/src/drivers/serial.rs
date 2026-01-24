use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial = unsafe { SerialPort::new(0x3F8) }; // COM1
        serial.init();
        Mutex::new(serial)
    };
}
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).ok();
}
