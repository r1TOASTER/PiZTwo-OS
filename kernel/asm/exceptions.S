.section .evt

/*
    @brief - the evt is the exception vectors table - holding the instructions on each exception that should happen - on aarch64 mode
    the entry evt is 2048 (2k) bytes aligned, each entry 0x80 (32 instructions)
    every exception label has it's own explenations (divided to 4 groups - each has Sync, IRQ, FIQ and SError)

    first group - cur_el_sp_el0 - exception made from the same EL (EL1 -> EL1), and using the SP of EL0
    second group - cur_el_sp_elx - exception made from the same EL (EL1 -> EL1), and using the SP of the same EL (EL1 -> EL1)
    third group - low_el_aarch64 - exception made from lower EL (only EL0), the arch that made the exception is aarch64
    fourth group - low_el_aarch32 - exception made from lower EL (only EL0), the arch that made the exception is aarch32
 */

.global exception_vector_table
.balign 2048 // makes the entire EVT 2048 bytes aligned - aarch64 specs
exception_vector_table:

.balign 0x80
cur_el_sp_el0_sync:
    b .

.balign 0x80
cur_el_sp_el0_irq:
    b .

.balign 0x80
cur_el_sp_el0_fiq:
    b .

.balign 0x80
cur_el_sp_el0_serror:
    b .

.balign 0x80
cur_el_sp_elx_sync:
	b .

.balign 0x80
cur_el_sp_elx_irq:
	b .

.balign 0x80
cur_el_sp_elx_fiq:
	b .

.balign 0x80
cur_el_sp_elx_serror:
	b .

.balign 0x80
low_el_aarch64_sync:
	b .

.balign 0x80
low_el_aarch64_irq:
	b .

.balign 0x80
low_el_aarch64_fiq:
	b .

.balign 0x80
low_el_aarch64_serror:
	b .

.balign 0x80
low_el_aarch32_sync:
	b .

.balign 0x80
low_el_aarch32_irq:
	b .

.balign 0x80
low_el_aarch32_fiq:
	b .

.balign 0x80
low_el_aarch32_serror:
	b .
