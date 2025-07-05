ENTRY(_start)

/*
    TODO: add .evt section (inside the .text)
    TODO: add .evt section variable (for the entry.S to handle configuration)
    TODO: add the RAM memory range for the RPi z2w
*/

SECTIONS {
    . = 0x80000;

    /* Code segment: readable and executable */
    .text : {
        *(.text)
        _evt = .;
        *(.evt)
    }
    . = ALIGN(4K);

    /* Read-only data segment */
    .rodata : {
        *(.rodata)
    }
    . = ALIGN(4K);

    /* Writable data segment */
    .data : {
        *(.data)
    }
    . = ALIGN(4K);

    /* Uninitialized data segment */
    .bss : {
        *(.bss)
    }
    . = ALIGN(4K);

    /* Discard unnecessary sections */
    /DISCARD/ : {
        *(.comment)
        *(.gnu*)
        *(.note*)
        *(.test_cases)
    }
}