#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

// THE SOVEREIGN HANDSHAKE (Refined for P8600)
// We use a fixed-length header to ensure GRUB 2.06+ compatibility.
global_asm!(
    ".section .multiboot",
    ".align 8",
    "multiboot_header_start:",
    ".long 0xe85250d6",                // Magic: Multiboot2
    ".long 0",                         // Architecture: i386
    ".long multiboot_header_end - multiboot_header_start", // Header Length
    ".long 0x100000000 - (0xe85250d6 + 0 + (multiboot_header_end - multiboot_header_start))", // Checksum
    
    // Required End Tag
    ".short 0", 
    ".short 0", 
    ".long 8",
    "multiboot_header_end:"
);

/// THE SOVEREIGN ENTRY POINT
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // Immediate Serial Output (Handshake Confirmation)
    unsafe {
        core::arch::asm!(
            "outb %al, %dx",
            in("dx") 0x3f8u16,
            in("al") b'H',
            options(att_syntax)
        );
        core::arch::asm!(
            "outb %al, %dx",
            in("dx") 0x3f8u16,
            in("al") b'I',
            options(att_syntax)
        );
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
