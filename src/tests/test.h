#ifndef TEST_H

#define TEST_H
#include "utils.h"

#define TYPECHECK(ptr, type)           \
    do                                 \
    {                                  \
        typeof(ptr) __tmp = (type *)0; \
        (void)__tmp;                   \
    } while (0)

static inline void kpanic(const char *msg)
{
    printk("KERNEL PANIC: %s\n", msg);
    __asm__ __volatile__("cli; hlt"); // stop everything
    for (;;)
        ; // just in case
}

#define kassert(cond, msg)                                                   \
    do                                                                       \
    {                                                                        \
        if (!(cond))                                                         \
        {                                                                    \
            printk("ASSERT FAILED: %s at %s:%d\n", msg, __FILE__, __LINE__); \
            kpanic(msg);                                                     \
        }                                                                    \
    } while (0)

#endif