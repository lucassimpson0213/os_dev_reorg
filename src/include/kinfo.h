// kinfo.h
#include <stdint.h>
#pragma once
typedef struct {
  uint32_t multiboot_magic;
  uint32_t mbi_phys;
  const char *cmdline;
} kinfo_t;

extern kinfo_t KINFO; 
