extern crate core;

use core::{mem, slice};
use multiboot::information::{MemoryManagement, Multiboot, PAddr};

struct Mem;

impl MemoryManagement for Mem {
    unsafe fn paddr_to_slice(&self, addr: PAddr, size: usize) -> Option<&'static [u8]> {
        let ptr = (addr as usize) as *const u8;

        Some(slice::from_raw_parts(ptr, size))
    }

    // If you only want to read fields, you can simply return `None`.
    unsafe fn allocate(&mut self, _length: usize) -> Option<(PAddr, &mut [u8])> {
        None
    }

    unsafe fn deallocate(&mut self, addr: PAddr) {
        if addr != 0 {
            unimplemented!()
        }
    }
}

/// mboot_ptr is the initial pointer to the multiboot structure
/// provided in %ebx on start-up.
pub fn use_multiboot<'a>(
    mboot_ptr: PAddr,
    mem: &'a mut impl MemoryManagement,
) -> Option<Multiboot<'a, 'a>> {
    unsafe { Multiboot::from_ptr(mboot_ptr, mem) }
}
