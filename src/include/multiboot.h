#include <stdint.h>

typedef struct multiboot_info {
    uint32_t flags;
    uint32_t mem_lower;
    uint32_t mem_upper;
    uint32_t boot_device;
    uint32_t cmdline;
    uint32_t mods_count;
    uint32_t mods_addr;
    uint32_t syms[4];
    uint32_t mmap_length;
    uint32_t mmap_addr;
} __attribute__((packed)) multiboot_info_t;

typedef struct multiboot_mmap_entry {
    uint32_t size;
    uint64_t addr;
    uint64_t len;
    uint32_t type;
} __attribute__((packed))  multiboot_mmap_entry_t;


typedef struct MemoryRegion{
    uint64_t base;
    uint64_t len;
    uint32_t type;
} MemoryRegion_t;

extern void *_heap_start, *_heap_end;
#define HEAP_START ((uintptr_t)&_heap_start)
#define HEAP_END ((uintptr_t)&_heap_end)

#define map_exists(x)((x) & (1u << 6))
