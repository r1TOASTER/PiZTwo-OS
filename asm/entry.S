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
    mov     sp, 0x8000 // check why SP like that
    bl      kernel_main
