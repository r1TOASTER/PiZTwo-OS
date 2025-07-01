/// 4 byte instructions per vector entry
/// Vector Table holding entries for the exception types:
/// Synchronous, IRQ, FIEQ, SError, SoftwareInterrupt (SWI, System call)
/// 
/// TODO: check execution mode (aarch64 vs aarch32 and accordingly do the setup, info under)
/// TODO: check if needed for every exeception level (rings, EL0 [usermode] to EL3 [kernel]) and mode (AArch64 vs AArch32)
/// TODO: test all with SP, PC, and such
/// TODO: Memory alignment for the EVT with enough space for entries - for aarch64: 2KB, for aarch32: 1KB

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

#[repr(C)]
pub struct ExceptionVectorTable {

}

#[used]
#[link_section = ".evt"]
pub static EVT: ExceptionVectorTable = ExceptionVectorTable {

};