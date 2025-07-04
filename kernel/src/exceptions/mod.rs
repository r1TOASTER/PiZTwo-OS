/// 4 byte instructions per vector entry
/// Vector Table holding entries for the exception types:
/// Synchronous, IRQ, FIEQ, SError, SoftwareInterrupt (SWI, System call)
/// 
/// TODO: check execution mode (aarch64 vs aarch32 and accordingly do the setup, info under)
/// TODO: check if needed for every exeception level (rings, EL0 [usermode] to EL3 [kernel]) and mode (AArch64 vs AArch32)
/// TODO: test all with SP, PC, and such
/// TODO: Memory alignment for the EVT with enough space for entries - for aarch64: 2KB, for aarch32: 1KB
/// TODO: When EVT all setup - change Makefile to use -device loader instead of -kernel (real hardware mode with EL3 and self EVT)

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

/// INFO:
/// https://www.linkedin.com/advice/0/how-do-you-configure-exception-vector-table-armv8-a

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

// for AArch64
#[repr(C)]
pub struct ExceptionVectorTable64 {

}

// for AArch32
#[repr(C)]
pub struct ExceptionVectorTable32 {

}

// TODO: EVT per EL -> holded by VBAR_ELx register (needed to elevate from ELx to ELx+1 and decrease when ret from ELx+1 to ELx)
/*
    EVT_EL1
    EVT_EL2
    EVT_EL3

    I Think! (no need for EVT_EL0 cause when exception occures on EL0 it will go to EVT_EL1)
    Needed to check for exception on EL3 (where it goes? EVT_EL3? or is it an EVT_EL0?)

    TODO: EVTs for AArch32 in EL1 (so user apps in EL0 can support AArch32)
*/

// link section at the format of '.evtX.elX' where the first x is the ES and the second X is the EL

#[used]
#[link_section = ".evt32.el1"] // maybe not pub?
pub static EVT32_EL1: ExceptionVectorTable32 = ExceptionVectorTable32 {

};

#[used]
#[link_section = ".evt64.el1"] // maybe not pub?
pub static EVT64_EL1: ExceptionVectorTable64 = ExceptionVectorTable64 {

};

#[used]
#[link_section = ".evt64.el2"] // maybe not pub?
pub static EVT64_EL2: ExceptionVectorTable64 = ExceptionVectorTable64 {

};

#[used]
#[link_section = ".evt64.el3"] // maybe not pub?
pub static EVT64_EL3: ExceptionVectorTable64 = ExceptionVectorTable64 {

};