ENTRY(_start)

SECTIONS
{
    /* PointSav Standard Load Address */
    . = 1M;

    .text : {
        /* Force the header to the very first byte! */
        KEEP(*(.multiboot2_header))
        *(.text .text.*)
    }

    .rodata : { *(.rodata .rodata.*) }
    .data   : { *(.data .data.*) }
    .bss    : { *(.bss .bss.*) }
}
