#![no_std]

use core::panic::PanicInfo;



#[unsafe(no_mangle)]
pub extern "C" fn rust_hello() -> u32 {
    0xCAFEBABE
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

