// 4 byte instructions per vector entry
// Vector Table holding entries for the exception types:
// Synchronous, IRQ, FIEQ, SError, SoftwareInterrupt (SWI, System call)
// 
// TODO: check execution mode (aarch64 vs aarch32 and accordingly do the setup, info under)
// TODO: test all with SP, PC, and such
// TODO: When EVT all setup - change Makefile to use -device loader instead of -kernel (real hardware mode with EL3 and self EVT)

/*  -- can be kernel8.img instead of kernel.elf
    -- peripherals would be at different locations, beware - need to test on real hardware
    qemu-system-aarch64 \
        -M virt \
        -cpu cortex-a53 \
        -nographic \
        -m 512M \
        -smp 1 \
        -bios none \
        -device loader,file=kernel.elf,addr=0x80000,cpu-num=0 \
        -semihosting-config enable=on,target=native \
        -S -gdb tcp::9999
*/

// INFO:
// https://www.linkedin.com/advice/0/how-do-you-configure-exception-vector-table-armv8-a

/*
The exception vector table is a contiguous block of memory that contains the addresses of the exception handlers for each type of exception.
The format of the exception vector table depends on the mode of the processor.
In AArch64 mode, the exception vector table consists of 16 128-bit entries, each containing four 32-bit addresses for:
- synchronous
- IRQ
- FIQ
- SError exception types. 
The exception vector table base address is stored in the VBAR_ELx register, where x is the exception level. 

In AArch32 mode, the exception vector table consists of eight 32-bit entries, each containing one address for:
- reset
- undefined instruction
- SVC
- prefetch abort
- data abort
- reserved
- IRQ
- FIQ exception types.
The exception vector table base address is stored in the VBAR register.
*/

// for AArch64 - no need for aarch32 implementation - in the EVT there is a subgroup for exception taken from lower EL using aarch32 (only EL0)
// each entry is 32 instructions: save corruptible registers, call exception specific handler, restore registers, call eret.

// TODO: check for masking / unmasking
// first - saving registers - using SP of current EL (SP_ELx) - 1 or 3, whether EL the exception is taken to
// then - switch to SP_EL0 for any further processing (should be cleaned later?)
// then - after the handling, pop every register used in SP_EL0
// then - switch back to SP_ELx and pop every saved register
// then - ERET