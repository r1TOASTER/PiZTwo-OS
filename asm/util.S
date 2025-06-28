.section .text

.global _exit_qemu
_exit_qemu:
    mov     x0, #0x18               // =24
    mov     x1, #0x20000            // =131072
    add     x1, x1, #0x26
    hlt     #0xf000

.global _prints_qemu
_prints_qemu:
    // get user's string data from register
    ldur    x1, [x0] 
    mov     x0, #0x04
    hlt     #0xf000
