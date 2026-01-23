# paging.s – GAS syntax, 32-bit

    .text

    .globl loadPageDirectory
loadPageDirectory:
    push %ebp
    mov  %esp, %ebp

    mov  8(%esp), %eax      # first argument: page directory physical address
    mov  %eax, %cr3         # load CR3

    mov  %ebp, %esp
    pop  %ebp
    ret

    .globl enablePaging
enablePaging:
    push %ebp
    mov  %esp, %ebp

    mov  %cr0, %eax
    or   $0x80000000, %eax  # set PG bit
    mov  %eax, %cr0

    mov  %ebp, %esp
    pop  %ebp
    ret
