# 32-bit OS (x86) — xv6-inspired

## Overview
A 32-bit x86 kernel that boots via **GRUB (Multiboot v1)**. The kernel is linked to load at **0x00200000 (2 MiB)** and enters at **_start** in `boot.s`.

## What happens at boot
- GRUB loads the ELF kernel per the linker script and jumps to **_start**.
- `boot.s` sets up a stack (symbol defined in the linker script) and transfers control to `kernel_main`.
- The kernel parses **Multiboot v1** info (flags, modules, memory map) to discover RAM.

## Memory layout
- **Linker script** places sections:
  - `.text` (code), `.rodata` (const), `.data` (init globals), `.bss` (zeroed),
  - reserved **stack** region (if defined).
- Load address: **0x00200000** (2 MiB), page-aligned.

## Features so far
- **VGA text mode** writes (0xB8000), simple console.
- **Serial output** (COM1, `0x3F8`) and `printk`.
- **String/utility** helpers.
- **Multiboot v1** parsing: flags, command line, and **memory map** (usable RAM regions).

## Build & Run
- Toolchain: `i686-elf-gcc`, `i686-elf-ld`, `nasm` (if you use Intel-syntax ASM).
- Build: `make`
- Run (quick): `qemu-system-i386 -kernel kernel.elf -serial stdio`
  - Or create a GRUB ISO and boot with: `qemu-system-i386 -cdrom os.iso -serial stdio`

## Next steps / roadmap
- GDT/IDT setup and interrupt handling.
- Paging (4 KiB, maybe 4 MiB PSE) and page alloc (bitmap or freelist).
- PIT/LAPIC timer, scheduler skeleton.
- Syscall entry (`int 0x80` / `SYSENTER`) and user mode.
- ELF loader for user programs; simple FS (ramfs) and drivers (keyboard).

![alt text](image.png)