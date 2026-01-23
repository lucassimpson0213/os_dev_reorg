.intel_syntax noprefix

.global sum3


.section .data
    input:
        .long 42
    sum:
        .long 0

.section .text



sum3:
    push ebp
    mov ebp, esp

    mov eax, [ebp + 8]
    add eax, [ebp + 12]
    add eax, [ebp + 16]


    mov esp, ebp
    pop ebp

    ret







