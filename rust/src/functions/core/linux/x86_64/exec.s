.global exec
exec:
    mov rax, 59
    mov rdi, rsi
    mov rsi, rdx
    mov rdx, rcx
    syscall
    ret