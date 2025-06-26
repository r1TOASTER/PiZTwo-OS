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
    // printing and exiting test
    // string print (0x04 swi)
    mov     x0, #0x4                // =4
    adr     x1, str_hello
    hlt     #0xf000
    
    mov     x0, #0x18               // =24
    mov     x1, #0x20000            // =131072
    add     x1, x1, #0x26
    hlt     #0xf000

    //    bl kernel_main

str_hello:
    .string "Hello World!\n"