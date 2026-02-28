ENTRY(_start)

SECTIONS {
    . = 1M; /* Load the kernel at 1MB */

    .text : {
        KEEP(*(.multiboot)) /* Ensure the header is FIRST */
        *(.text .text.*)
    }

    .rodata : { *(.rodata .rodata.*) }
    .data : { *(.data .data.*) }
    .bss : { *(.bss .bss.*) }
}
