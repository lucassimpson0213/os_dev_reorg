#ifndef TEST_PMM_H
#define TEST_PMM_H

/*
 * PMM test harness.
 *
 * Call test_pmm() once after initialize_heap() and before you rely
 * on the allocator for anything serious.
 */

void test_pmm(void);

#endif /* TEST_PMM_H */
