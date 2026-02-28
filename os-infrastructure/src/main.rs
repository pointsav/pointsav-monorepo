#![no_std]
#![no_main]

use core::arch::{global_asm, asm};
use core::panic::PanicInfo;

// MULTIBOOT2 HANDSHAKE
global_asm!(
    ".section .multiboot",
    ".align 8",
    "multiboot_header_start:",
    ".long 0xe85250d6",
    ".long 0",
    ".long 16",
    ".long 0x100000000 - (0xe85250d6 + 0 + 16)",
    ".short 0", ".short 0", ".long 8",
    "multiboot_header_end:"
);

fn outb(port: u16, val: u8) { unsafe { asm!("outb %al, %dx", in("dx") port, in("al") val, options(att_syntax)); } }
fn outl(port: u16, val: u32) { unsafe { asm!("outl %eax, %dx", in("dx") port, in("eax") val, options(att_syntax)); } }
fn inl(port: u16) -> u32 { let val: u32; unsafe { asm!("inl %dx, %eax", out("eax") val, in("dx") port, options(att_syntax)); } val }

fn print_serial(s: &str) { for b in s.bytes() { outb(0x3f8, b); } }

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    print_serial("\n--- SOVEREIGN PCI SCAN ---\n");

    for slot in 0..32 {
        let address = (1 << 31) | (slot << 11) | (0x0);
        outl(0xCF8, address);
        let val = inl(0xCFC);
        let vendor = (val & 0xFFFF) as u16;
        let device = (val >> 16) as u16;

        if vendor != 0xFFFF {
            print_serial("Found Device: ");
            // In a real build, we'd format hex here; for now, we just flag the Broadcom ID
            if vendor == 0x14e4 && device == 0x432b {
                print_serial("!!! BROADCOM BCM4322 DETECTED !!!\n");
            } else {
                print_serial("Unknown Hardware\n");
            }
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop {} }
