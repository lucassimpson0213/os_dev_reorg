#include <stdint.h>
#include "memlayout.h"
#include "multiboot.h"
#include <stddef.h>
#include "str.h"
#include "test.h"
#include "memlayout.h"


#define NUM_BYTES 512

// this file contains code for the physical memory manager

/*
    initialize bookkeeping region
    starting after heap_start we allocate 512 bytes to keep track of pages
    this can keep track of 2048 pages
    pool size is 8MiB and page size is 4kb.
    With 2bits per page we can address 4 pages per byte
    therefore we have 2048 pages we can address

    new plan just use 1 byte per page, so you need one page full of page metadata
 */
#define PGSIZE 4096
#define bookkeep_end ((uintptr_t)HEAP_START + (PGSIZE))

typedef enum
{
    BK_FREE = 0,
    BK_TAKEN = 1,
    BK_RESERVED = 2
} bk_state_t;

// please just use a byte instead of 2 bits, 2bit headache :(

void initialize_heap()
{
    size_t heap_size = HEAP_END - HEAP_START;
    memset((void *)HEAP_START, 0, heap_size);

    uint8_t *heap = (uint8_t *)HEAP_START;
    heap[0] = BK_RESERVED;

    // zero heap
}

// cases to test
// what if allocator can't allocate that number of pages
// what if allocator squashes heap region
//
void *alloc_page(size_t num_pages)
{

    if(num_pages > 1) {
        return NULL;
    }


    /*
        TODO
        use this mapping and use invariants

        page index p

        address addr = HEAP_START + p * PGSIZE
    */

    char *address = (char *)PGROUNDUP(bookkeep_end);
    uint8_t *heap = (uint8_t *)HEAP_START;

    
    // start at first page instead of page 0 because the first page is reserved for bookkeeping

    kassert((uintptr_t)address % 4096 == 0, "address is misaligned not a multiple of 4096");

    
    size_t page = 1;

    // starting from first page in heap region
    while (address < (char* ) HEAP_END && heap[page] != BK_FREE)
    {
        address += PGSIZE;
        page++;
    }

    if(address >= (char *) HEAP_END) {
        return NULL;
    }
    else {
        heap[page] = BK_TAKEN;
    }

    // zero page

    memset((void *)address, 0, 4096);

    return (void *)address;
}

void free_page(void *address)
{

    if ((uintptr_t)address >= bookkeep_end && (uintptr_t) address < HEAP_END && (uintptr_t)address % 4096 == 0)
    {
        size_t addressHolder = ((uintptr_t)address - (uintptr_t)HEAP_START) / PGSIZE;
        uint8_t *heap = (uint8_t *)HEAP_START;

        if (heap[addressHolder] == BK_TAKEN)
        {
            heap[addressHolder] = BK_FREE;
        }
    }
}
