/* 
    TODO: clear bss
    TODO: mov to x_data whatever defined as rodata / data / edata
    TODO: stack pointer alignment
    TODO: check interrupt needs
    TODO: check if any other init needed
*/

.section .text

.global _start
_start:
    mrs     x0, MPIDR_EL1 // read the MPIDR_EL1 system reg to x5 - 64 bit
    // bits 0-7 (Aff0) holding the current core id
    and     x0, x0, #0xff
    // if the core is zero, branch to kernel_main
    // otherwise, wait for wake up interrupt
    cbz     x0, core_0
    // if not core 0, wait for interrupt to wake up (enter idle state)
idle:
    wfi
    b       idle
core_0:
    mov     sp, 0x8000 // check why SP like that
    bl      kernel_main
    b       . // halting here
