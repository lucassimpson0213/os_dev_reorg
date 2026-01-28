/*
*:
* extern unsigned int rust_parse_multiboot_map(uint32_t magic, uint32_t mbi_phys);
*
*/
use crate::serial_println
pub fn rust_parse_multiboot_map(magic: u32, mbi_phys: u32) -> u32 {
    serial_println!("{:?}", magic);
    return 0;
}






