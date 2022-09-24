	.section .text
    .code64
    .global _start

_start:
    rdtscp
    mov     $60, %rax               # system call 60 is exit
    xor     %rdi, %rdi              # we want return code 0
    syscall

