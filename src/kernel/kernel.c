#include "gdt.h"
#include "kernel_h.h"
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

void traverse_multiboot_mmap(uint32_t mbi_phys, struct MemoryRegion regions[]) {
  multiboot_info_t *mbi = (multiboot_info_t *)(uintptr_t)mbi_phys;
  kassert(mbi_phys != 0, "the bootloader parameter mbi_phys equals 0");
  printk("%x\n", mbi->mmap_addr);

  kassert(map_exists(mbi->flags) != 0,
          "memory map does not exist. the flags are not set");

  if (map_exists(mbi->flags)) {
    multiboot_mmap_entry_t *mmap =
        (multiboot_mmap_entry_t *)(uintptr_t)mbi->mmap_addr;

    // currently p is pointer to the first 8 bytes of the memory address
    uint8_t *p = (uint8_t *)(uintptr_t)(uint32_t)mbi->mmap_addr;

    // make sure that p walks by a byte at a time using an 8 bit pointer
    typecheck(uint8_t, p);

    uint32_t mmap_length = mbi->mmap_length;
    uint8_t *end = p + mmap_length;

    uint region_count = 0;

    while (p < end) {

      multiboot_mmap_entry_t *e = (multiboot_mmap_entry_t *)p;
      kassert(e->size >= 20, "mmap entry size too small");
      // the addresses must be 64 bit
      uint64_t base = e->addr;

      if (e->type == 1 && end > base && region_count < MAX_REGIONS) {
        regions[region_count++] = (struct MemoryRegion){
            .base = e->addr, .len = e->len, .type = e->type};
      }

      // only incrememnt by the size of the struct if we're less than p so we
      // don't overshoot
      if ((e->size + sizeof(e->size)) <= p)
        p += (e->size + sizeof(e->size));
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

static inline void outw(uint16_t port, uint16_t val) {
  __asm__ volatile("outw %0, %1" : : "a"(val), "Nd"(port));
}

void qemu_success() { outw(0xF4, 0x10); }

void qemu_poweroff(void) {
  outw(0x604, 0x2000);
  for (;;)
    __asm__ volatile("hlt");
}

void kernel_main(uint32_t magic, uint32_t mbi_phys) {
  printk("Kernel boots correctly!");
  extern unsigned int rust_ping(void);
  extern void init_heap_rust(void);
  int sum = sum3(1, 2, 3);

  size_t result = serial_init();
  printk("KERNEL_BOOK_OK");
  struct MemoryRegion regions[MAX_REGIONS];

  test_pmm();

  print_hex64(regions->base);

  print_hex64(regions->len);

  print_hex64(regions->base + regions->len);

  init_gdt();
  gdt_flush();

  traverse_multiboot_mmap(mbi_phys, regions);

  terminal_initialize();

  rust_idt_entry();
  init_paging();
  init_heap_rust();
  qemu_success();
  qemu_poweroff();

  // rust_parse_multiboot_map(0, 0);

  //  rust_parse_multiboot_map(magic, mbi_phys);
}
