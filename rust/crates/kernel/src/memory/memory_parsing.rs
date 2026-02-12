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

pub mod multiboot1_data {
    /// Value found in %eax after multiboot jumps to our entry point.
    pub const SIGNATURE_EAX: u32 = 0x2BADB002;

    pub type PAddr = u64;

    /// Representation of Multiboot Information according to the Multiboot1 spec.
    #[repr(C)]
    #[derive(Default)]
    pub struct MultibootInfo {
        pub flags: u32,

        pub mem_lower: u32,
        pub mem_upper: u32,

        pub boot_device: BootDevice,

        /// Physical address of a zero-terminated C string.
        pub cmdline: u32,

        pub mods_count: u32,
        pub mods_addr: u32,

        pub symbols: Symbols,

        pub mmap_length: u32,
        pub mmap_addr: u32,

        pub drives_length: u32,
        pub drives_addr: u32,

        pub config_table: u32,

        pub boot_loader_name: u32,

        pub apm_table: u32,

        pub vbe_control_info: u32,
        pub vbe_mode_info: u32,
        pub vbe_mode: u16,
        pub vbe_interface_seg: u16,
        pub vbe_interface_off: u16,
        pub vbe_interface_len: u16,

        pub framebuffer_table: FramebufferTable,
    }

    /// The ‘boot_device’ field.
    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct BootDevice {
        pub drive: u8,
        pub partition1: u8,
        pub partition2: u8,
        pub partition3: u8,
    }

    impl Default for BootDevice {
        fn default() -> Self {
            Self {
                drive: 0xff,
                partition1: 0xff,
                partition2: 0xff,
                partition3: 0xff,
            }
        }
    }

    /// Multiboot raw module entry (as stored in memory).
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default)]
    pub struct MBModule {
        pub start: u32,
        pub end: u32,
        pub string: u32,
        pub reserved: u32,
    }

    /// Types that define if the memory is usable or not.
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[repr(u32)]
    pub enum MemoryType {
        /// memory, available to OS
        Available = 1,
        /// reserved, not available (rom, mem map dev)
        Reserved = 2,
        /// ACPI Reclaim Memory
        ACPI = 3,
        /// ACPI NVS Memory
        NVS = 4,
        /// defective RAM modules
        Defect = 5,
    }

    /// Multiboot format of the MMAP buffer.
    ///
    /// Note: in Multiboot1, the `size` field does not include itself.
    #[repr(C, packed)]
    #[derive(Clone, Copy)]
    pub struct MemoryEntry {
        pub size: u32,
        pub base_addr: u64,
        pub length: u64,
        pub mtype: u32,
    }

    impl Default for MemoryEntry {
        fn default() -> Self {
            // 0-length reserved entry
            Self {
                size: 20, // typical (struct size minus size field), but 0 default is also fine
                base_addr: 0,
                length: 0,
                mtype: MemoryType::Reserved as u32,
            }
        }
    }

    /// Information about a module (friendly form).
    #[derive(Debug)]
    pub struct Module<'a> {
        pub start: PAddr,
        pub end: PAddr,
        pub string: Option<&'a str>,
    }

    impl<'a> Module<'a> {
        pub fn new(start: PAddr, end: PAddr, name: Option<&'a str>) -> Self {
            Self {
                start,
                end,
                string: name,
            }
        }
    }

    /// Multiboot format for Symbols.
    #[repr(C)]
    pub union Symbols {
        pub aout: AOutSymbols,
        pub elf: ElfSymbols,
        _align: [u32; 4],
    }

    impl Default for Symbols {
        fn default() -> Self {
            Self {
                elf: ElfSymbols::default(),
            }
        }
    }

    /// Safe wrapper for either AOutSymbols or ElfSymbols.
    #[derive(Debug, Copy, Clone)]
    pub enum SymbolType {
        AOut(AOutSymbols),
        Elf(ElfSymbols),
    }

    /// Multiboot format for AOut symbols.
    #[repr(C)]
    #[derive(Default, Copy, Clone)]
    pub struct AOutSymbols {
        pub tabsize: u32,
        pub strsize: u32,
        pub addr: u32,
        pub reserved: u32,
    }

    impl core::fmt::Debug for AOutSymbols {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("AOutSymbols")
                .field("tabsize", &self.tabsize)
                .field("strsize", &self.strsize)
                .field("addr", &self.addr)
                .finish()
        }
    }

    /// Multiboot format for ELF symbols.
    #[repr(C)]
    #[derive(Default, Copy, Clone)]
    pub struct ElfSymbols {
        pub num: u32,
        pub size: u32,
        pub addr: u32,
        pub shndx: u32,
    }

    impl core::fmt::Debug for ElfSymbols {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("ElfSymbols")
                .field("num", &self.num)
                .field("size", &self.size)
                .field("addr", &self.addr)
                .field("shndx", &self.shndx)
                .finish()
        }
    }

    /// Contains the information about the framebuffer.
    #[repr(C)]
    #[derive(Default)]
    pub struct FramebufferTable {
        pub addr: u64,
        pub pitch: u32,
        pub width: u32,
        pub height: u32,
        pub bpp: u8,
        pub ty: u8,
        pub color_info: ColorInfo,
    }

    impl core::fmt::Debug for FramebufferTable {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("FramebufferTable")
                .field("addr", &self.addr)
                .field("pitch", &self.pitch)
                .field("width", &self.width)
                .field("height", &self.height)
                .field("bpp", &self.bpp)
                .field("ty", &self.ty)
                .finish()
        }
    }

    /// Safe wrapper for framebuffer color info.
    #[derive(Debug, Clone, Copy)]
    pub enum ColorInfoType {
        Palette(ColorInfoPalette),
        Rgb(ColorInfoRgb),
        Text,
    }

    /// Multiboot format for the framebuffer color info.
    #[repr(C)]
    pub union ColorInfo {
        pub palette: ColorInfoPalette,
        pub rgb: ColorInfoRgb,
        _align: [u32; 2],
    }

    // Default type is 0 (indexed color), so initialize palette info
    impl Default for ColorInfo {
        fn default() -> Self {
            Self {
                palette: ColorInfoPalette {
                    palette_addr: 0,
                    palette_num_colors: 0,
                },
            }
        }
    }

    /// Information for indexed color mode.
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct ColorInfoPalette {
        pub palette_addr: u32,
        pub palette_num_colors: u16,
    }

    /// Information for direct RGB color mode.
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct ColorInfoRgb {
        pub red_field_position: u8,
        pub red_mask_size: u8,
        pub green_field_position: u8,
        pub green_mask_size: u8,
        pub blue_field_position: u8,
        pub blue_mask_size: u8,
    }
}

mod multiboot_implementation {

    use multiboot::information::PAddr;

    use crate::memory::memory_parsing::multiboot1_data;

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
            let multiboot_info = multiboot1_data::MultibootInfo;

            // crate::memory::multiboot_helper::use_multiboot(mboot_ptr, mem)
        }
        MB2_BOOTLOADER_MAGIC => {
            printings::serial_write_string("kernel is using multiboot 2");
        }
        _ => {
            printings::serial_write_string("multi boot has not been idenified");
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
