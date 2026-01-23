# gdbinit-qemu
set pagination off
set confirm off
set disassemble-next-line on
set print pretty on

# Tell gdb what binary has the symbols
file kernel.elf

# Connect to QEMU's gdb stub (-s defaults to :1234)
target remote :1234

# Optional: if your linker places kernel at 1MB and you feel symbols are off,
# you might need an add-symbol-file. Try without first.
# add-symbol-file kernel.elf 0x00100000

# Nice UI (optional)
tui enable
layout split
layout regs

# Always show instruction + regs + top of stack
display/i $pc
display/x $esp
display/x $ebp
display/x $eip
display/x $eax
display/x $ebx
display/x $ecx
display/x $edx
display/16wx $esp

define st
  printf "\n=== STACK ===\n"
  printf "EIP=%#x  ESP=%#x  EBP=%#x\n", $eip, $esp, $ebp
  printf "[ebp+4] return address:\n"
  x/wx $ebp+4
  printf "[ebp+8] arg1:\n"
  x/wx $ebp+8
  printf "Stack (from ESP):\n"
  x/24wx $esp
end


# Put a breakpoint somewhere sensible
# Change this to your kernel entry C function (kmain, kernel_main, etc.)
break kernel_main

# Continue from QEMU's -S pause to the breakpoint
continue
