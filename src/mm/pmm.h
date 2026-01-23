#ifndef PMM_H
#define PMM_H

#include <stddef.h>
#include <stdint.h>

/*
 * Physical Memory Manager (PMM) interface
 *
 * Design:
 * - Uses one page at HEAP_START as bookkeeping (1 byte per page).
 * - Page 0 (index 0 in the bookkeeping array) is reserved for metadata.
 * - Each subsequent page index maps to:
 *     address = HEAP_START + page_index * PGSIZE
 *
 * API:
 * - initialize_heap(): zeroes the heap region and marks bookkeeping page as reserved.
 * - alloc_page(num_pages): currently supports num_pages == 1, returns a single 4 KiB page or NULL.
 * - free_page(address): marks the page at 'address' as free if it was previously taken.
 */

/**
 * Initialize the heap and PMM bookkeeping structures.
 * Must be called once during early kernel initialization.
 */
void initialize_heap(void);

/**
 * Allocate one or more contiguous physical pages.
 *
 * Currently:
 *  - Only supports num_pages == 1.
 *  - Returns a pointer to a zeroed 4 KiB page on success.
 *  - Returns NULL if no suitable page is available or num_pages > 1.
 */
void *alloc_page(size_t num_pages);

/**
 * Free a previously allocated page.
 *
 * - 'address' must be page-aligned and within the managed heap range.
 * - If the page is marked BK_TAKEN, it is set back to BK_FREE.
 * - Invalid frees are silently ignored (for now).
 */
void free_page(void *address);

#endif /* PMM_H */
