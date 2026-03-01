#![no_std]
#![no_main]

use core::arch::{global_asm, asm};
use core::panic::PanicInfo;

// REFINED MULTIBOOT2 HANDSHAKE
global_asm!(
    ".section .multiboot",
    ".align 8",
    "header_start:",
    ".long 0xe85250d6",                // Magic: Multiboot2
    ".long 0",                         // Architecture: i386 (protected mode)
    ".long header_end - header_start", // Header Length
    ".long 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))", // Checksum
    
    // TAG: Information Request (Requesting Memory Map)
    ".short 1",                        // Type 1
    ".short 0",                        // Flags
    ".long 8",                         // Size
    
    // TAG: End
    ".short 0", 
    ".short 0", 
    ".long 8",
    "header_end:"
);

fn outb(port: u16, val: u8) { unsafe { asm!("outb %al, %dx", in("dx") port, in("al") val, options(att_syntax)); } }
fn print_serial(s: &str) { for b in s.bytes() { outb(0x3f8, b); } }

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // IMMEDIATE HANDSHAKE
    print_serial("\n--- SOVEREIGN KERNEL ACTIVE ---\n");
    print_serial("Initializing PCI Scan...\n");

    // PCI scan logic would go here...
    
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print_serial("\n!!! KERNEL PANIC !!!\n");
    loop {}
}
