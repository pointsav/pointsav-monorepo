ENTRY(_start)

SECTIONS
{
    . = 0x100000; /* Standard entry point for 64-bit boot */
    .text : {
        *(.text._start)
        *(.text*)
    }
    .rodata : { *(.rodata*) }
    .data : { *(.data*) }
    .bss : { *(.bss*) }
}
