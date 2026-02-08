#pragma once
#include <stdint.h>

#define QEMU_DEBUG_PORT 0xF4

/* =========================
   Subsystems (high byte)
   ========================= */

#define SUBSYS_BOOT 0x01
#define SUBSYS_GDT 0x02
#define SUBSYS_IDT 0x03
#define SUBSYS_PAGING 0x04
#define SUBSYS_MEMORY 0x05
#define SUBSYS_INTERRUPT 0x06
#define SUBSYS_SYSCALL 0x07

/* =========================
   Stages
   ========================= */

#define STAGE_ENTER 0x01
#define STAGE_INIT 0x02
#define STAGE_LOAD 0x03
#define STAGE_ENABLE 0x04
#define STAGE_HANDLE 0x05

/* =========================
   Error types
   ========================= */

#define ERR_NONE 0x00
#define ERR_NULL_PTR 0x01
#define ERR_BAD_DESCRIPTOR 0x02
#define ERR_PAGE_FAULT 0x03
#define ERR_GPF 0x04
#define ERR_DOUBLE_FAULT 0x05
#define ERR_BAD_INTERRUPT 0x06
#define ERR_UNKNOWN 0xFF

/* =========================
   Port write
   ========================= */

static inline void outl(uint16_t port, uint32_t val) {
  __asm__ volatile("outl %0, %1" : : "a"(val), "Nd"(port));
}

/* =========================
   Encode panic
   [ subsystem | stage | error | info ]
     8 bits      8 bits   8 bits  8 bits
   ========================= */

#define PANIC_CODE(subsys, stage, error, info)                                 \
  ((uint32_t)((subsys << 24) | (stage << 16) | (error << 8) | (info)))

/* =========================
   Exit to QEMU
   ========================= */

__attribute__((noreturn)) static inline void
qemu_panic(uint8_t subsys, uint8_t stage, uint8_t error, uint8_t info) {
  uint32_t code = PANIC_CODE(subsys, stage, error, info);
  outl(QEMU_DEBUG_PORT, code);

  /* halt forever */
  for (;;)
    __asm__ volatile("hlt");
}
