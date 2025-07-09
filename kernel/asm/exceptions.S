.section .evt

.global exception_vector_table
exception_vector_table:
.org 0x0000
cur_el_sp_el0_sync:

.org 0x0080
cur_el_sp_el0_irq:

.org 0x0100
cur_el_sp_el0_fiq:

.org 0x0180
cur_el_sp_el0_serror:

.org 0x0200
cur_el_sp_elx_sync:

.org 0x0280
cur_el_sp_elx_irq:

.org 0x0300
cur_el_sp_elx_fiq:

.org 0x0380
cur_el_sp_elx_serror:

.org 0x0400
low_el_aarch64_sync:

.org 0x0480
low_el_aarch64_irq:

.org 0x0500
low_el_aarch64_fiq:

.org 0x0580
low_el_aarch64_serror:

.org 0x0600
low_el_aarch32_sync:

.org 0x0680
low_el_aarch32_irq:

.org 0x0700
low_el_aarch32_fiq:

.org 0x0780
low_el_aarch32_serror:
