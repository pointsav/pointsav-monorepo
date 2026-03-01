ENTRY(_start)

SECTIONS {
    /* The 1MB physical address required by Multiboot2 */
    . = 1M;

    .boot : {
        /* Force the header to the very first byte of the file */
        KEEP(*(.multiboot))
    }

    .text : {
        *(.text .text.*)
    }

    .rodata : { *(.rodata .rodata.*) }
    .data : { *(.data .data.*) }
    .bss : { *(.bss .bss.*) }
}
