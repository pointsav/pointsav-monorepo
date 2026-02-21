ENTRY(efi_main)
SECTIONS
{
    . = 0x1000;
    .text : {
        *(.text .text.*)
    }
    .data : {
        *(.data .data.*)
        *(.rodata .rodata.*)
    }
    .firmware : {
        KEEP(*(.firmware))
    }
}
