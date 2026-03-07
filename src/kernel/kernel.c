#include "gdt.h"
#include "multiboot.h"
#include "pmm.h"
#include "practice.h"
#include "rust_import.h"
#include "str.h"
#include "test.h"
#include "test_pmm.h"
#include "typecheck.h"
#include "utils.h"
#include "vm.h"
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

/* Check if the compiler thinks you are targeting the wrong operating system. */
// #if defined(__linux__)
// #er ror \
//   "You are not using a cross-compiler, you will most certainly run into
//    trouble"
// #endif

/* This tutorial will only work for the 32-bit ix86 targets. */
// #if !defined(__i386__)
// #error "This tutorial needs to be compiled with a ix86-elf compiler"
// #endif

void traverse_multiboot_mmap(uint32_t mbi_phys, struct MemoryRegion regions[])
{
  multiboot_info_t *mbi = (multiboot_info_t *)(uintptr_t)mbi_phys;
  kassert(mbi_phys != 0, "the bootloader parameter mbi_phys equals 0");
  printk("%x\n", mbi->mmap_addr);

  kassert(map_exists(mbi->flags) != 0,
          "memory map does not exist. the flags are not set");

  if (map_exists(mbi->flags))
  {
    multiboot_mmap_entry_t *mmap =
        (multiboot_mmap_entry_t *)(uintptr_t)mbi->mmap_addr;

    // currently p is pointer to the first 8 bytes of the memory address
    uint8_t *memory_map_pointer = (uint8_t *)(uintptr_t)(uint32_t)mbi->mmap_addr;

    // make sure that p walks by a byte at a time using an 8 bit pointer
    typecheck(uint8_t, memory_map_pointer);

    uint32_t mmap_length = mbi->mmap_length;

    //add the address of the memory_map and the memory map length to get the end address
    uint8_t *end = memory_map_pointer + mmap_length;

    uint region_count = 0;

    while (memory_map_pointer < end)
    {

      multiboot_mmap_entry_t *multiboot_entry = (multiboot_mmap_entry_t *)memory_map_pointer;
      kassert(multiboot_entry->size >= 20, "mmap entry size too small");
      // the addresses must be 64 bit
      uint64_t base = multiboot_entry->addr;

      if (multiboot_entry->type == 1 && end > base && region_count < MAX_REGIONS)
      {
        regions[region_count++] = (struct MemoryRegion){
            .base = multiboot_entry->addr, .len = multiboot_entry->len, .type = multiboot_entry->type};
      }

      // only incrememnt by the size of the struct if we're less than p so we
      // don't overshoot
      if ((multiboot_entry->size + sizeof(multiboot_entry->size)) <= memory_map_pointer)
        memory_map_pointer += (multiboot_entry->size + sizeof(multiboot_entry->size));
    }

    kassert(p == end, "p is not equal to end pointer");

    // one iteration

    // this is the address of the memory map
    printk("mmap memory address %d\n", (uint32_t)mbi->mmap_addr);
    printk("sizeof entry struct: %d\n", sizeof(multiboot_mmap_entry_t));
    printk("entry->size field: %d\n", mmap->size);
    printk("entry->addr: %x\n", (uint32_t)mmap->addr);
    printk("entry->len: %x\n", (uint32_t)mmap->len);
    printk("entry->type: %d\n", mmap->type);
    printk("entry pointer: %p\n", mmap);
  }
}

static inline void outw(uint16_t port, uint16_t val)
{
  __asm__ volatile("outw %0, %1" : : "a"(val), "Nd"(port));
}

void qemu_success() { outw(0xF4, 0x10); }

void qemu_poweroff(void)
{
  outw(0x604, 0x2000);
  for (;;)
    __asm__ volatile("hlt");
}

void kernel_main(uint32_t magic, uint32_t mbi_phys)
{
  printk("Kernel boots correctly!");
  extern unsigned int rust_ping(void);
  extern void init_heap_rust(void);

  size_t result = serial_init();
  printk("KERNEL_BOOK_OK");

  test_pmm();

  init_gdt();
  gdt_flush();

  rust_idt_entry();
  init_paging();
  init_heap_rust();
  qemu_success();
  qemu_poweroff();

  // rust_parse_multiboot_map(0, 0);

  //  rust_parse_multiboot_map(magic, mbi_phys);
}
