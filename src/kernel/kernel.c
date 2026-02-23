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
#include "terminal.h"
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
