use crate::gdtentry::Entry;
use bitflags::bitflags;
use std::fmt;

// 63..56 base_high
// 55    granularity
// 54    default_size
// 53    long_mode
// 52    available
// 51..48 limit_high
// 47    present
// 46..45 dpl
// 44    user_segment
// 43    executable
// 42    conforming
// 41    writable
// 40    accessed
// 39..16 base_low
// 15..0  limit_low

bitflags! {
    /// Flags for a GDT descriptor. Not all flags are valid for all descriptor types.
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    pub struct DescriptorFlags: u64 {
        /// Set by the processor if this segment has been accessed. Only cleared by software.
        /// _Setting_ this bit in software prevents GDT writes on first use.
        const ACCESSED          = 1 << 40;
        /// For 32-bit data segments, sets the segment as writable. For 32-bit code segments,
        /// sets the segment as _readable_. In 64-bit mode, ignored for all segments.
        const WRITABLE          = 1 << 41;
        /// For code segments, sets the segment as “conforming”, influencing the
        /// privilege checks that occur on control transfers. For 32-bit data segments,
        /// sets the segment as "expand down". In 64-bit mode, ignored for data segments.
        const CONFORMING        = 1 << 42;
        /// This flag must be set for code segments and unset for data segments.
        const EXECUTABLE        = 1 << 43;
        /// This flag must be set for user segments (in contrast to system segments).
        const USER_SEGMENT      = 1 << 44;
        /// These two bits encode the Descriptor Privilege Level (DPL) for this descriptor.
        /// If both bits are set, the DPL is Ring 3, if both are unset, the DPL is Ring 0.
        const DPL_RING_3        = 3 << 45;
        /// Must be set for any segment, causes a segment not present exception if not set.
        const PRESENT           = 1 << 47;
        /// Available for use by the Operating System
        const AVAILABLE         = 1 << 52;
        /// Must be set for 64-bit code segments, unset otherwise.
        const LONG_MODE         = 1 << 53;
        /// Use 32-bit (as opposed to 16-bit) operands. If [`LONG_MODE`][Self::LONG_MODE] is set,
        /// this must be unset. In 64-bit mode, ignored for data segments.
        const DEFAULT_SIZE      = 1 << 54;
        /// Limit field is scaled by 4096 bytes. In 64-bit mode, ignored for all segments.
        const GRANULARITY       = 1 << 55;

        /// Bits `0..=15` of the limit field (ignored in 64-bit mode)
        const LIMIT_0_15        = 0xFFFF;
        /// Bits `16..=19` of the limit field (ignored in 64-bit mode)
        const LIMIT_16_19       = 0xF << 48;
        /// Bits `0..=23` of the base field (ignored in 64-bit mode, except for fs and gs)
        const BASE_0_23         = 0xFF_FFFF << 16;
        /// Bits `24..=31` of the base field (ignored in 64-bit mode, except for fs and gs)
        const BASE_24_31        = 0xFF << 56;
    }
}

// 63..56 base_high
// 55    granularity
// 54    default_size
// 53    long_mode
// 52    available
// 51..48 limit_high
// 47    present
// 46..45 dpl
// 44    user_segment
// 43    executable
// 42    conforming
// 41    writable
// 40    accessed
// 39..16 base_low
// 15..0  limit_low
mod descriptor_flags {

    use crate::DescriptorFlags;

    impl DescriptorFlags {
        // Flags that we set for all our default segments
        pub const COMMON: Self = Self::from_bits_truncate(
            Self::USER_SEGMENT.bits()
                | Self::PRESENT.bits()
                | Self::WRITABLE.bits()
                | Self::ACCESSED.bits()
                | Self::LIMIT_0_15.bits()
                | Self::LIMIT_16_19.bits()
                | Self::GRANULARITY.bits(),
        );

        /// A kernel data segment (64-bit or flat 32-bit)
        pub const KERNEL_DATA: Self =
            Self::from_bits_truncate(Self::COMMON.bits() | Self::DEFAULT_SIZE.bits());
        /// A flat 32-bit kernel code segment
        pub const KERNEL_CODE32: Self = Self::from_bits_truncate(
            Self::COMMON.bits() | Self::EXECUTABLE.bits() | Self::DEFAULT_SIZE.bits(),
        );
        /// A 64-bit kernel code segment
        pub const KERNEL_CODE64: Self = Self::from_bits_truncate(
            Self::COMMON.bits() | Self::EXECUTABLE.bits() | Self::LONG_MODE.bits(),
        );
        /// A user data segment (64-bit or flat 32-bit)
        pub const USER_DATA: Self =
            Self::from_bits_truncate(Self::KERNEL_DATA.bits() | Self::DPL_RING_3.bits());

        /// A flat 32-bit user code segment
        pub const USER_CODE32: Self =
            Self::from_bits_truncate(Self::KERNEL_CODE32.bits() | Self::DPL_RING_3.bits());

        /// A 64-bit user code segment
        pub const USER_CODE64: Self =
            Self::from_bits_truncate(Self::KERNEL_CODE64.bits() | Self::DPL_RING_3.bits());
    }
    #[cfg(test)]
    mod tests {
        use super::*;

        macro_rules! dbg_type {
            ($val:expr) => {
                println!(
                    "type = {}, value = {:?}",
                    core::any::type_name_of_val(&$val),
                    &$val
                );
            };
        }
        #[test]
        pub fn decode_data() {
            let common = DescriptorFlags::COMMON;
            println!("{:?}", common);
            dbg_type!(common);
            common.bits();
        }
    }
}
mod gdtentry {
    use crate::fmt;
    use u64 as EntryValue;

    #[repr(transparent)]
    pub struct Entry(u64);

    impl Entry {
        // Create a new Entry from a raw value.
        pub const fn new(raw: u64) -> Self {
            Self(raw)
        }

        /// The raw bits for this entry. Depending on the [`Descriptor`] type, these
        /// bits may correspond to those in [`DescriptorFlags`].
        pub fn raw(&self) -> u64 {
            // TODO: Make this const fn when AtomicU64::load is const.
            let raw = self.0;
            raw
        }
    }

    impl Clone for Entry {
        fn clone(&self) -> Self {
            Self::new(self.raw())
        }
    }

    impl PartialEq for Entry {
        fn eq(&self, other: &Self) -> bool {
            self.raw() == other.raw()
        }
    }

    impl Eq for Entry {}

    impl fmt::Debug for Entry {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // Display inner value as hex
            write!(f, "Entry({:#018x})", self.raw())
        }
    }
}
/*
GDT Segment Descriptor (32-bit protected mode)
=============================================

Total size: 64 bits (8 bytes)

Bit layout (MSB → LSB):

 63            56 55 54 53 52 51        48 47      45 44 43      40 39        32 31        16 15         0
+---------------+--+--+--+--+------------+----------+--+---------+------------+------------+------------+
|   BASE[31:24] | G|DB| L|AVL| LIMIT[19:16] | PRESENT | DPL |  S  |   TYPE     | BASE[23:16] | BASE[15:0] |
+---------------+--+--+--+--+------------+----------+--+---------+------------+------------+------------+
                                                                                               LIMIT[15:0]

Field meanings:
- BASE        : 32-bit segment base address
- LIMIT       : 20-bit segment limit
- TYPE        : Segment type (code/data, read/write, conforming, etc.)
- S           : Descriptor type (1 = code/data, 0 = system)
- DPL         : Descriptor Privilege Level (0–3)
- PRESENT     : Segment present in memory
- AVL         : Available for OS use
- L           : Long mode (must be 0 in 32-bit protected mode)
- DB          : Default operand size (1 = 32-bit segment)
- G           : Granularity (0 = byte, 1 = 4KiB pages)

Flat 32-bit segment convention:
- BASE  = 0x00000000
- LIMIT = 0xFFFFF
- G     = 1 (4KiB granularity)
- DB    = 1 (32-bit)
- L     = 0
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
        let low_bits = (isr_routine & 0xFFFF);
        low_bits.to_ne_bytes();
        println!("low bits {}", low_bits);
        self
    }
}

fn main() {
    println!("Hello, world!");
    let idt_entry = IdtEntry::new();
    let flags = DescriptorFlags::COMMON;
    let bits_of_flags = DescriptorFlags::COMMON.bits();
    println!(
        "common flags = {:#x} ({:#010b})",
        bits_of_flags, bits_of_flags
    );

    println!("Flags: {:?}", flags);

    let entry: Entry = Entry::new(flags.bits());
    println!("Entry type: {:?}", entry);
    let new_idt_entry = idt_entry.isr(0xFFFF);
}
