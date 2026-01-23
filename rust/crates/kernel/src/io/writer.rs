use core::fmt::{self, Write};

// This function must be implemented to write to your specific hardware
// (e.g., serial port on a microcontroller, or a syscall in an OS kernel).

fn write_to_hardware(bytes: &[u8]) {}

struct Writer;
