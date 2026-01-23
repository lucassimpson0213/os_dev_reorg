#include "test_pmm.h"
#include "pmm.h"
#include "memlayout.h"
#include "test.h"
#include "memlayout.h"
#include "multiboot.h"
#include <stdint.h>
#include <stddef.h>


#ifndef PGSIZE
#define PGSIZE 4096
#endif

// Adjust if your heap can describe more than this many pages
#define MAX_TEST_PAGES 4097

static int is_page_aligned(void *addr) {
    return ((uintptr_t)addr % PGSIZE) == 0;
}

/*
 * Test 1: single alloc + free + re-alloc the same page
 */
static void test_single_alloc_free(void) {
    void *p = alloc_page(1);
    kassert(p != NULL, "single_alloc_free: alloc_page(1) returned NULL");
    kassert(is_page_aligned(p), "single_alloc_free: returned address not page-aligned");
    kassert((uintptr_t)p >= (uintptr_t)HEAP_START,
            "single_alloc_free: address below HEAP_START");
    kassert((uintptr_t)p < (uintptr_t)HEAP_END,
            "single_alloc_free: address beyond HEAP_END");

    free_page(p);

    // allocate again, should succeed (likely same page in first-fit)
    void *p2 = alloc_page(1);
    kassert(p2 != NULL, "single_alloc_free: second alloc_page(1) returned NULL");

    free_page(p2);
}

/*
 * Test 2: allocate until exhaustion, then ensure we get NULL.
 */
static void test_exhaustion(void) {
    size_t total_pages = (HEAP_END - HEAP_START) / PGSIZE;
    size_t usable_pages = (total_pages > 0) ? total_pages - 1 : 0; // page 0 = bookkeeping

    printk("%d", total_pages);
    printk("%d", usable_pages);
    kassert(usable_pages <= MAX_TEST_PAGES,
            "test_exhaustion: increase MAX_TEST_PAGES to cover your heap");

    void *pages[MAX_TEST_PAGES];
    size_t count = 0;

    while (count < usable_pages) {
        void *p = alloc_page(1);
        if (p == NULL) {
            break;
        }
        kassert(is_page_aligned(p), "test_exhaustion: got non-aligned page");
        pages[count++] = p;
    }

    // We should have filled all usable pages
    kassert(count == usable_pages,
            "test_exhaustion: did not allocate all usable pages (off-by-one?)");

    // At this point, we should be out of memory
    void *extra = alloc_page(1);
    kassert(extra == NULL, "test_exhaustion: alloc_page should return NULL when full");

    // free everything
    for (size_t i = 0; i < count; i++) {
        free_page(pages[i]);
    }
}

/*
 * Test 3: ensure two pages don't overlap (write different patterns).
 */
static void test_no_overlap(void) {
    void *a = alloc_page(1);
    void *b = alloc_page(1);

    kassert(a != NULL && b != NULL, "test_no_overlap: allocations failed");
    kassert(a != b, "test_no_overlap: two allocations returned the same address");
    kassert(is_page_aligned(a) && is_page_aligned(b),
            "test_no_overlap: pages not aligned");

    uint8_t *pa = (uint8_t *)a;
    uint8_t *pb = (uint8_t *)b;

    for (size_t i = 0; i < PGSIZE; i++) {
        pa[i] = 0xAA;
        pb[i] = 0x55;
    }

    for (size_t i = 0; i < PGSIZE; i++) {
        kassert(pa[i] == 0xAA, "test_no_overlap: page A content corrupted");
        kassert(pb[i] == 0x55, "test_no_overlap: page B content corrupted");
    }

    free_page(a);
    free_page(b);
}

/*
 * Test 4: last-page edge case — can we actually allocate the last page?
 */
static void test_last_page(void) {
    size_t total_pages = (HEAP_END - HEAP_START) / PGSIZE;
    size_t usable_pages = (total_pages > 0) ? total_pages - 1 : 0;

    kassert(usable_pages > 0, "test_last_page: no usable pages in heap");
    kassert(usable_pages <= MAX_TEST_PAGES,
            "test_last_page: increase MAX_TEST_PAGES to cover your heap");

    void *pages[MAX_TEST_PAGES];
    size_t count = 0;

    // allocate all usable pages
    while (count < usable_pages) {
        void *p = alloc_page(1);
        kassert(p != NULL, "test_last_page: unexpected NULL during full allocation");
        pages[count++] = p;
    }

    kassert(count == usable_pages,
            "test_last_page: did not allocate all usable pages (off-by-one?)");

    // one more should fail
    void *extra = alloc_page(1);
    kassert(extra == NULL, "test_last_page: expected NULL after full allocation");

    for (size_t i = 0; i < count; i++) {
        free_page(pages[i]);
    }
}

/*
 * Test 5: double free should be safely ignored by your current implementation.
 */
static void test_double_free(void) {
    void *p = alloc_page(1);
    kassert(p != NULL, "test_double_free: allocation failed");

    free_page(p);
    free_page(p); // should be ignored without breaking metadata

    void *q = alloc_page(1);
    kassert(q != NULL, "test_double_free: allocation after double free failed");

    free_page(q);
}

void test_pmm(void) {
    // Start from a clean slate
    initialize_heap();

    test_single_alloc_free();
    test_exhaustion();
    test_no_overlap();
    test_last_page();
    test_double_free();

    // Optional: if you have a logger/printf in your kernel:
    // kprintf("PMM tests passed.\n");
}