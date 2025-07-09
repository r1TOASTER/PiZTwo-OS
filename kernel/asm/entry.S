/* 
    EVERTYTHING HERE: for all cores
    TODO: different PC and SP based on defined stack size per core, and kernel schedualing
    TODO: clear bss
    TODO: mov to .data whatever defined as rodata / data (maybe not rodata)
    TODO: stack pointer alignment and assignment (changed for cores)
    TODO: check if any other init needed
    TODO: reset GPRs?
    TODO: MMU and caches init?
    TODO: NEON and FP init?
    TODO: change exception level? or still in kernel mode
*/

.section .text

.global _start
_start:
    // Starting at EL2

    // Configuring EVT location in memory inside VBAR_EL1 register
    mrs     x0, VBAR_EL1
    adrp    x1, _evt
    lsl     x1, x1, #0xb
    and     x0, x0, x1
    msr     VBAR_EL1, x0

    // TODO: interrup enable
    // TODO: set stacks accordingly to core
    // TODO: Pass control to EL1?

    mrs     x0, MPIDR_EL1 // read the MPIDR_EL1 system reg to x0 - 64 bit
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
