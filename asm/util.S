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
    ldur    x0, [x1] 
    mov     x0, #0x04
    hlt     0xf000

.global memcpy
memcpy:
    // x0: destination, x1: source, x2: length
    cbz x2, .Lmemcpy_done
.Lmemcpy_loop:
    ldrb w3, [x1], #1
    strb w3, [x0], #1
    subs x2, x2, #1
    bne .Lmemcpy_loop
.Lmemcpy_done:
    ret
