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
#if defined(__linux__)
#error                                                                         \
    "You are not using a cross-compiler, you will most certainly run into trouble"
#endif

/* This tutorial will only work for the 32-bit ix86 targets. */
#if !defined(__i386__)
#error "This tutorial needs to be compiled with a ix86-elf compiler"
#endif

void terminal_scroll();
/* Hardware text mode color constants. */
enum vga_color {
  VGA_COLOR_BLACK = 0,
  VGA_COLOR_BLUE = 1,
  VGA_COLOR_GREEN = 2,
  VGA_COLOR_CYAN = 3,
  VGA_COLOR_RED = 4,
  VGA_COLOR_MAGENTA = 5,
  VGA_COLOR_BROWN = 6,
  VGA_COLOR_LIGHT_GREY = 7,
  VGA_COLOR_DARK_GREY = 8,
  VGA_COLOR_LIGHT_BLUE = 9,
  VGA_COLOR_LIGHT_GREEN = 10,
  VGA_COLOR_LIGHT_CYAN = 11,
  VGA_COLOR_LIGHT_RED = 12,
  VGA_COLOR_LIGHT_MAGENTA = 13,
  VGA_COLOR_LIGHT_BROWN = 14,
  VGA_COLOR_WHITE = 15,
};

static inline uint8_t vga_entry_color(enum vga_color fg, enum vga_color bg) {
  return fg | bg << 4;
}

static inline uint16_t vga_entry(unsigned char uc, uint8_t color) {
  return (uint16_t)uc | (uint16_t)color << 8;
}

#define VGA_WIDTH 80
#define VGA_HEIGHT 25
#define VGA_MEMORY 0xB8000

size_t terminal_row;
size_t terminal_column;
uint8_t terminal_color;
uint16_t *terminal_buffer = (uint16_t *)VGA_MEMORY;

void terminal_initialize(void) {
  terminal_row = 0;
  terminal_column = 0;
  terminal_color = vga_entry_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);

  for (size_t y = 0; y < VGA_HEIGHT; y++) {
    for (size_t x = 0; x < VGA_WIDTH; x++) {
      const size_t index = y * VGA_WIDTH + x;
      terminal_buffer[index] = vga_entry(' ', terminal_color);
    }
  }
}

void terminal_setcolor(uint8_t color) { terminal_color = color; }

void terminal_putentryat(char c, uint8_t color, size_t x, size_t y) {
  const size_t index = y * VGA_WIDTH + x;
  terminal_buffer[index] = vga_entry(c, color);
}

void terminal_putchar(char c) {

  if (c == '\n') {
    terminal_row++;
    terminal_column = 0;
  }
  terminal_putentryat(c, terminal_color, terminal_column, terminal_row);

  if (++terminal_column == VGA_WIDTH) {
    terminal_column = 0;
    if (terminal_row + 1 == VGA_HEIGHT)
      terminal_row = 0;
    terminal_scroll();
  }
}

void terminal_scroll() {
  // for each row in the vga terminal buffer
  //  move each row up one
  //  and clear the last row

  for (int y = 1; y < VGA_HEIGHT; y++) {
    for (int x = 0; x < VGA_WIDTH; x++) {
      const size_t index = y * VGA_WIDTH + x;
      const size_t replace_index = (y - 1) * VGA_WIDTH + x;
      terminal_buffer[replace_index] = terminal_buffer[index];
    }
  }
  const uint8_t clear_terminal_color =
      vga_entry_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);

  for (int x = 0; x < VGA_WIDTH; x++) {
    const size_t last_row_plus_x = (VGA_HEIGHT - 1) * VGA_WIDTH + x;

    terminal_buffer[last_row_plus_x] = vga_entry(' ', clear_terminal_color);
  }
}

void terminal_write(const char *data, size_t size) {
  for (size_t i = 0; i < size; i++)
    terminal_putchar(data[i]);
}

void terminal_write_string(const char *data) {
  terminal_write(data, strlen(data));
}

_Static_assert(offsetof(multiboot_mmap_entry_t, addr) == 4, "mmap.addr offset");
_Static_assert(offsetof(multiboot_mmap_entry_t, len) == 12, "mmap.len offset");
_Static_assert(offsetof(multiboot_mmap_entry_t, type) == 20,
               "mmap.type offset");

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

void kernel_main(uint32_t magic, uint32_t mbi_phys) {

  extern unsigned int rust_ping(void);
  extern void init_heap_rust(void);
  int sum = sum3(1, 2, 3);

  size_t result = serial_init();

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

  // rust_parse_multiboot_map(0, 0);

  //  rust_parse_multiboot_map(magic, mbi_phys);
}
