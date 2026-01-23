#include <stdint.h>
#include "pmm.h"
#include "paging.h"
#include "str.h"


void init_paging() {
    uint32_t *page_directory = alloc_page(1); // must return a 4 KB aligned frame
    memset(page_directory, 0, 4096);

    uint32_t *first_page_table = alloc_page(1);

    for (uint32_t i = 0; i < 1024; i++)
    {
        first_page_table[i] = (i * 0x1000) | 3;
    }

    page_directory[0] = (uint32_t)first_page_table | 3;

    loadPageDirectory(page_directory);
    enablePaging();
}

