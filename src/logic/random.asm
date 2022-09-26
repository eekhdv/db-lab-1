.section .text
.code64


# First param:  RCX
# Second param: RDX
# Return:       RAX

pe_rand:
    push rbx 
    push rdx
    rdtsc                    
    mov rbx, rax            
    mov rax, rcx            
    div rbx
    mul rdx
    pop rdx
    pop rbx
    ret

efi_rand:
    call pe_rand

win64_rand:
    call pe_rand


# First param:  RDI
# Second param: RSI
# Return:       RAX

system_rand:
    push rbx 
    push rdx

    push rsi
    rdtsc                    
    mov rbx, rax            
    mov rax, rdi            
    div rbx
    mul rdx
    div rcx
    mov rax, rdx
    pop rsi
    mov rbx, rsi
    xor rdx, rdx
    div rbx
    mov rax, rdx

    pop rdx
    pop rbx
    ret

.section .data
rand_file:
    .asciz "/dev/random"
