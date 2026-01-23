.global gdt_flush
gdt_flush:
    # expects nothing; just reloads using your selectors

    ljmp $0x08, $1f          # reload CS (kernel code selector)

1:
    mov $0x10, %ax           # kernel data selector
    mov %ax, %ds
    mov %ax, %es
    mov %ax, %fs
    mov %ax, %gs
    mov %ax, %ss

    ret
