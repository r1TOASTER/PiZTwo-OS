.section .text

.global _exit_qemu
_exit_qemu:
    mov     x0, #0x18
    mov     x1, #0x20000
    add     x1, x1, #0x26
    hlt     #0xf000

// working for SWI semihosting alone - for release build it should be using the miniUart connected to console
.global _prints_qemu 
_prints_qemu:
    // get user's string ptr from x0 (passing *const u8)
    mov     x1, x0 
    mov     x0, #0x04
    hlt     #0xf000
    ret
