.global exit
exit:
    mov rcx, -1
    mov rdx, rdi
    mov rax, 0x2D
    syscall
    ret
