#include "utils.h"
#include <stddef.h>
#include <stdint.h>

/* ===== I/O ports ===== */
static inline void outb(uint16_t port, uint8_t val)
{
   __asm__ volatile("outb %0, %1" : : "a"(val), "Nd"(port));
}
static inline uint8_t inb(uint16_t port)
{
   uint8_t ret;
   __asm__ volatile("inb %1, %0" : "=a"(ret) : "Nd"(port));
   return ret;
}

/* ===== Serial (COM1) ===== */
#define COM1 0x3F8

int serial_init(void)
{
   outb(COM1 + 1, 0x00);
   outb(COM1 + 3, 0x80);
   outb(COM1 + 0, 0x03);
   outb(COM1 + 1, 0x00);
   outb(COM1 + 3, 0x03); // 8N1
   outb(COM1 + 2, 0xC7); // FIFO on
   outb(COM1 + 4, 0x0B); // IRQs, RTS/DSR
   outb(COM1 + 4, 0x1E); // loopback test
   outb(COM1 + 0, 0xAE);
   if (inb(COM1 + 0) != 0xAE)
      return 1;
   outb(COM1 + 4, 0x0F); // normal mode
   return 0;
}


void print_hex64(uint64_t number) {
   uint32_t hi = (uint32_t)(number >> 32);
   uint32_t lo = (uint32_t)(number);

   print_hex32_padded(hi);
   print_hex32_padded(lo);
}
static int serial_tx_empty(void) { return inb(COM1 + 5) & 0x20; }
void write_serial(char c)
{
   if (c == '\n')
   {
      while (!serial_tx_empty())
      {
      }
      outb(COM1, '\r');
   }
   while (!serial_tx_empty())
   {
   }
   outb(COM1, (uint8_t)c);
}
static int serial_rx_ready(void) { return inb(COM1 + 5) & 1; }
char read_serial(void)
{
   while (!serial_rx_ready())
   {
   }
   return (char)inb(COM1);
}

/* ===== Console sink ===== */
void consputc(char c) { write_serial(c); }

/* ===== Tiny printers (32-bit) ===== */
 void print_hex32(uint32_t v)
{
   static const char d[] = "0123456789abcdef";
   int lead = 1;
   for (int sh = 28; sh >= 0; sh -= 4)
   {
      char ch = d[(v >> sh) & 0xF];
      if (lead && ch == '0' && sh)
         continue;
      lead = 0;
      consputc(ch);
   }
   if (lead)
      consputc('0');
}
 void print_hex32_padded(uint32_t v)
{
  const char d[] = "0123456789abcdef";
   for (int sh = 28; sh >= 0; sh -= 4)
      consputc(d[(v >> sh) & 0xF]);
}
static void print_uint_dec32(uint32_t v)
{
   char buf[10];
   int i = 0;
   if (!v)
   {
      consputc('0');
      return;
   }
   while (v)
   {
      buf[i++] = (char)('0' + (v % 10));
      v /= 10;
   }
   while (--i >= 0)
      consputc(buf[i]);
}
static void print_sint_dec32(int32_t x)
{
   if (x < 0)
   {
      consputc('-');
      uint32_t ux = (uint32_t)(-(int64_t)x); // handles INT_MIN
      print_uint_dec32(ux);
   }
   else
   {
      print_uint_dec32((uint32_t)x);
   }
}

/* ===== printk: %d %x %p %s %% ===== */
void printk(const char *format, ...)
{
   va_list ap;
   va_start(ap, format);
   int state = 0; // 0=text, 1=after '%'

   for (const char *p = format; *p; ++p)
   {
      char c = *p;
      if (!state)
      {
         if (c == '%')
            state = 1;
         else
            consputc(c);
         continue;
      }
      switch (c)
      {
      case 'd':
      {
         int v = va_arg(ap, int);
         print_sint_dec32(v);
      }
      break;
      case 'x':
      {
         unsigned v = va_arg(ap, unsigned);
         print_hex32(v);
      }
      break;
      case 'p':
      {
         uintptr_t v = (uintptr_t)va_arg(ap, void *);
         consputc('0');
         consputc('x');
         print_hex32_padded((uint32_t)v); // 8 nybbles on 32-bit
      }
      break;
      case 's':
      {
         const char *s = va_arg(ap, const char *);
         if (!s)
            s = "(null)";
         while (*s)
            consputc(*s++);
      }
      break;
      case '%':
         consputc('%');
         break;
      default:
         consputc('%');
         consputc(c); // unknown: print literally
         break;
      }
      state = 0;
   }
   va_end(ap);
}

