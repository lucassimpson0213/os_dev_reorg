#ifndef UTILS_H
#define UTILS_H

#include <stdint.h>
#include <stdarg.h>
#define MAX_REGIONS 100

/* Serial */
int serial_init(void);
void write_serial(char c);
char read_serial(void);

/* Console */
void consputc(char c);
void printk(const char *format, ...);
void print_hex64(uint64_t v);
void print_hex32_padded(uint32_t v);



#endif // UTILS_H
