.section .data
.align 4

msg:
    .asciz "Hello, World!\n"
end_msg:

len = . - msg

.section .text
.global _start
_start:
    mov x0, #1
    ldr x1, =msg
    mov x2, len
    mov x8, #64
    svc #0

    mov x0, #0
    mov x8, #93
    svc #0
