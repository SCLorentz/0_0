@msg = private unnamed_addr constant [3 x i8] c"oi\0A", align 1

define i64 @_start() {
entry:
  %msg_ptr = getelementptr inbounds [3 x i8], [3 x i8]* @msg, i32 0, i32 0

  call void asm sideeffect "
    mov x8, #64      // syscall: write
    mov x0, #1       // stdout
    mov x1, $0       // msg ptr
    mov x2, #3       // length
    svc #0
    mov x8, #93      // syscall: exit
    mov x0, #0
    svc #0
  ", "r"(i8* %msg_ptr)

  ret i64 0
}