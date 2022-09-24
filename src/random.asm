.section .text
.code64

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

system_rand:
    push rbx 
    push rdx
    rdtsc                    
    mov rbx, rax            
    mov rax, rdi            
    div rbx
    mul rdx
    pop rdx
    pop rbx
    ret

.section .data
rand_file:
    .asciz "/dev/random"
