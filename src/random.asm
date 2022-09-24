.section .text
.code64
.global random

random:
    xor rax, rax
    rdtsc
    ret

