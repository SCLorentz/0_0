.global write_armx
write_armx:
    mov x0, 1
    mov x16, #4
    svc 0x80
    ret
