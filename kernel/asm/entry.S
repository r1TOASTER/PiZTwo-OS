/* 
    EVERTYTHING HERE: for all cores
    TODO: different PC and SP based on defined stack size per core, and kernel schedualing
    TODO: clear bss
    TODO: mov to .data whatever defined as rodata / data (maybe not rodata)
    TODO: stack pointer alignment and assignment (changed for cores)
    TODO: check if any other init needed
    TODO: MMU and caches init?
    TODO: NEON and FP init?
*/

.section .text

.global _start
_start:
    // Starting at EL2 (each core ritual)
    
    // reseting GPRs to make sure
    mov     x0, XZR
    mov     x1, XZR
    mov     x2, XZR
    mov     x3, XZR
    mov     x4, XZR
    mov     x5, XZR
    mov     x6, XZR
    mov     x7, XZR
    mov     x8, XZR
    mov     x9, XZR
    mov     x10, XZR
    mov     x11, XZR
    mov     x12, XZR
    mov     x13, XZR
    mov     x14, XZR
    mov     x15, XZR
    mov     x16, XZR
    mov     x17, XZR
    mov     x18, XZR
    mov     x19, XZR
    mov     x20, XZR
    mov     x21, XZR
    mov     x22, XZR
    mov     x23, XZR
    mov     x24, XZR
    mov     x25, XZR
    mov     x26, XZR
    mov     x27, XZR
    mov     x28, XZR
    mov     x29, XZR
    mov     x30, XZR

    // Configuring EVT location in memory inside VBAR_EL1 register
    mrs     x0, VBAR_EL1
    adrp    x1, _evt
    lsl     x1, x1, #0xB
    and     x0, x0, x1
    msr     VBAR_EL1, x0

    // TODO: interrup enable
    // TODO: set stacks accordingly to core


    // TODO: Pass control to EL1?
    // TODO: Pass DTB to kernel
    // TODO: Zero bss / Pass it to kernel to zero it

    // define a new sp for the current EL (2) - maybe not if not used?
    // set EL1 as aarch64 execution state
    // set EL1 as non-secure?
    // check no trapping (never going to EL2)

    // should set HCR_EL2.RW = 1 (check what it meant)
    

    // check core using affinity 0

    mrs     x0, MPIDR_EL1 // read the MPIDR_EL1 system reg to x0 - 64 bit
    // bits 0-7 (Aff0) holding the current core id
    and     x0, x0, #0xFF
    // if the core is zero, branch to kernel_main
    // otherwise, wait for wake up interrupt
    cbz     x0, core_0
    // if not core 0, wait for interrupt to wake up (enter idle state)
idle:
    wfi
    b       idle
core_0:
    mov     sp, #0x8000 // check why SP like that
    bl      kernel_main
    b       . // halting here
