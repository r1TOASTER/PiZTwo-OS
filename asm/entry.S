/* 
    EVERTYTHING HERE: for all cores
    TODO: different PC and SP based on defined stack size per core, and kernel schedualing
    TODO: clear bss
    TODO: mov to .data whatever defined as rodata / data (maybe not rodata)
    TODO: stack pointer alignment and assignment (changed for cores)
    TODO: EVT check? ISRs? check execution levels!
    TODO: check if any other init needed
    TODO: reset GPRs?
    TODO: MMU and caches init?
    TODO: NEON and FP init?
    TODO: change exception level? or still in kernel mode
*/

.section .text

.global _start
_start:
    // TODO: MPIDR_EL3 maybe?
    mrs     x0, MPIDR_EL1 // read the MPIDR_EL1 (3??) system reg to x5 - 64 bit
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
