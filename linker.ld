ENTRY(_start)

SECTIONS {
    . = 0x80000;

    .text : {
        *(.text)
    }

    .data : {

    }

    /* export entry_bss / end_bss (so I can zero it) */
    .bss : {

    }

    /* copy from entry to end of rodata? */
    .rodata : {

    }


    /*
        check if these segments are needed
    */
    .stab : {

    }

    .stabstr : {

    }
}