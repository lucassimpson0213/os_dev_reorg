/* setgdt.S - assembled by GAS via i686-elf-gcc */
.intel_syntax noprefix
.code32

.global setGdt

.section .data
/* 6-byte GDTR pseudo-descriptor: limit(16) + base(32) */
gdtr:
  .word 0
  .long 0

.section .text
setGdt:
  /* cdecl args:
     [esp+4] = uint16_t limit
     [esp+8] = uint32_t base
  */
  mov ax,  WORD PTR [esp + 4]
  mov WORD PTR [gdtr], ax

  mov eax, DWORD PTR [esp + 8]
  mov DWORD PTR [gdtr + 2], eax

  lgdt [gdtr]
  ret

