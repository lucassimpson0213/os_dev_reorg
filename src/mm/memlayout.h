// // symbols.h
// #pragma once
// extern void *_heap_start;
// extern void *_heap_end;
// extern void *_stack_start;
// extern void *_stack_end;
// extern void *_bss_start;
// extern void *_bss_end;
// extern void *_data_start;
// extern void *_data_end;
// extern void *_text_start;
// extern void *_text_end;
// extern void *_rodata_start;
// extern void *_rodata_end;
// extern void *_memory_start;
// extern void *_memory_end;

// #define sym_start(segment) \
//     ((unsigned long)&_##segment##_start)

// #define sym_end(segment) \
//     ((unsigned long)&_##segment##_end)

extern char _kernel_start, _kernel_end;
#define KERNEL_PHYS_START ((uintptr_t)&_kernel_start)
#define KERNEL_PHYS_END   ((uintptr_t)&_kernel_end)
#define PGSIZE 4096
#define V2P(a) (a)
#define P2V(a) (a)


#define PGROUNDUP(sz)  (((sz)+PGSIZE-1) & ~(PGSIZE-1))
#define PGROUNDDOWN(a) (((a)) & ~(PGSIZE-1))

#define PTE_P 0x001
#define PTE_W 0x002
#define PTE_U 0x004

#define PTE_ADDR(pte)((pte) & ~0xFFF)


/*
    printf("MEMORY SECTION MAPPING\n");
printf("~~~~~~~~~~~~~~~~~~~~~~\n");
printf(" ...%-8s: 0x%08lx - 0x%08lx\n", "memory", sym_start(memory), sym_end(memory));
printf(" ...%-8s: 0x%08lx - 0x%08lx\n", "text", sym_start(text), sym_end(text));
printf(" ...%-8s: 0x%08lx - 0x%08lx\n", "rodata", sym_start(rodata), sym_end(rodata));
printf(" ...%-8s: 0x%08lx - 0x%08lx\n", "data", sym_start(data), sym_end(data));
printf(" ...%-8s: 0x%08lx - 0x%08lx\n", "bss", sym_start(bss), sym_end(bss));

*/
