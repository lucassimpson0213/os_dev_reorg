# =========================
# Toolchain
# =========================
CC      := i686-elf-gcc
QEMU    := qemu-system-i386
GDB     := gdb
ISO_DIR     := iso
GRUB_DIR    := $(ISO_DIR)/boot/grub
KERNEL_BIN  := kernel.elf
ISO_IMAGE   := myos.iso

# =========================
# Directories
# =========================
SRC_DIR     := src
BOOT_DIR    := $(SRC_DIR)/boot
KERNEL_DIR  := $(SRC_DIR)/kernel
MM_DIR      := $(SRC_DIR)/mm
LIB_DIR     := $(SRC_DIR)/lib
ARCH_DIR    := $(SRC_DIR)/arch/x86
TEST_DIR    := $(SRC_DIR)/tests
INCLUDE_DIR := $(SRC_DIR)/include
BUILD_DIR   := build

# =========================
# Rust support library
# =========================
RUST_DIR := rust/crates/kernel
RUST_LIB := rust/target/i686-os/debug/libkernel.a

# =========================
# Flags
# =========================
INCLUDES := -I$(INCLUDE_DIR) -I$(MM_DIR) -I$(LIB_DIR) -I$(ARCH_DIR) -I$(TEST_DIR)
CFLAGS   := -std=gnu99 -ffreestanding -g -O2 -Wall -Wextra -fno-builtin -nostdlib $(INCLUDES)
LDFLAGS  := -T linker.ld -nostdlib

# =========================
# Source files
# =========================
# Assembly sources
ASM_SRCS := $(BOOT_DIR)/boot.s \
            $(BOOT_DIR)/paging.s \
            $(BOOT_DIR)/setgdt.s \
            $(BOOT_DIR)/practice.s \
            $(BOOT_DIR)/reload.s

# C sources
C_SRCS := $(KERNEL_DIR)/kernel.c \
          $(MM_DIR)/pmm.c \
          $(MM_DIR)/vm.c \
          $(MM_DIR)/kalloc.c \
          $(LIB_DIR)/utils.c \
          $(LIB_DIR)/str.c \
          $(ARCH_DIR)/gdt.c \
          $(TEST_DIR)/testpmm.c

# =========================
# Object files
# =========================
ASM_OBJS := $(patsubst $(SRC_DIR)/%.s,$(BUILD_DIR)/%.o,$(ASM_SRCS))
C_OBJS   := $(patsubst $(SRC_DIR)/%.c,$(BUILD_DIR)/%.o,$(C_SRCS))
OBJS     := $(ASM_OBJS) $(C_OBJS)
#==========================
#GRUB SECTION
#=========================
$(GRUB_DIR)/grub.cfg: | $(GRUB_DIR)
	@printf '%s\n' \
	'set timeout=0' \
	'set default=0' \
	'' \
	'menuentry "my kernel" {' \
	'    multiboot /boot/$(KERNEL_BIN)' \
	'    boot' \
	'}' > $@

$(GRUB_DIR):
	mkdir -p $(GRUB_DIR)

$(ISO_DIR)/boot:
	mkdir -p $(ISO_DIR)/boot
iso_root: kernel.elf | $(ISO_DIR)/boot $(GRUB_DIR)
	cp $(KERNEL_BIN) $(ISO_DIR)/boot/$(KERNEL_BIN)
	$(MAKE) $(GRUB_DIR)/grub.cfg






# =========================
# Default
# =========================
all: kernel.elf

# =========================
# Create build directories
# =========================
$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)/boot $(BUILD_DIR)/kernel $(BUILD_DIR)/mm $(BUILD_DIR)/lib $(BUILD_DIR)/arch/x86 $(BUILD_DIR)/tests

# =========================
# Rust build
# =========================
$(RUST_LIB):
	cd $(RUST_DIR) && cargo build --target i686-os.json

# =========================
# Compile rules
# =========================
# Assembly
$(BUILD_DIR)/%.o: $(SRC_DIR)/%.s | $(BUILD_DIR)
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) -c $< -o $@

# C
$(BUILD_DIR)/%.o: $(SRC_DIR)/%.c | $(BUILD_DIR)
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) -c $< -o $@

# =========================
# Link
# =========================
kernel.elf: $(OBJS) $(RUST_LIB)
	$(CC) $(LDFLAGS) -o $@ $(OBJS) $(RUST_LIB) -lgcc

# =========================
# Run
# =========================
run: kernel.elf
	$(QEMU) -kernel kernel.elf -serial stdio

debug: kernel.elf
	$(QEMU) -kernel kernel.elf -serial file:serial.log -s -S

gdb: kernel.elf
	$(GDB) -q -nh -nx  -x gdbinit-qemu
run-test:
	qemu-system-i386 -kernel kernel.elf -device isa-debug-exit,iobase=0xf4,iosize=0x04

# =========================
# Clean
# =========================
clean:
	rm -rf $(BUILD_DIR) kernel.elf
	cd rust && cd crates && cd kernel &&  cargo clean

.PHONY: all run debug gdb clean
