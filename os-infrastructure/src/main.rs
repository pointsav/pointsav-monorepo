#![no_std]
#![no_main]

use core::arch::{global_asm, asm};
use core::panic::PanicInfo;
use system_substrate_broadcom::silicon_ping;
use system_network_interface::{enable_monitor_mode, init_dma_engine, hunt_for_eapol, RX_BUFFERS};

global_asm!(
    ".section .multiboot",
    ".align 8",
    "header_start:",
    ".long 0xe85250d6",
    ".long 0",
    ".long header_end - header_start",
    ".long 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))",
    ".short 0", ".short 0", ".long 8",
    "header_end:",
    ".section .text",
    ".global _start",
    "_start:",
    "lea rsp, [rip + stack_top]",
    "call rust_main",
    "halt_loop:",
    "hlt",
    "jmp halt_loop",
    ".section .bss",
    ".align 16",
    "stack_bottom:",
    ".skip 16384",
    "stack_top:"
);

const FONT: [[u8; 8]; 16] = [
    [0x3C, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3C], [0x18, 0x38, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C],
    [0x3C, 0x66, 0x06, 0x0C, 0x18, 0x30, 0x60, 0x7E], [0x3C, 0x66, 0x06, 0x1C, 0x06, 0x06, 0x66, 0x3C],
    [0x0C, 0x1C, 0x3C, 0x6C, 0x7E, 0x0C, 0x0C, 0x0C], [0x7E, 0x60, 0x7C, 0x06, 0x06, 0x06, 0x66, 0x3C],
    [0x3C, 0x66, 0x60, 0x7C, 0x66, 0x66, 0x66, 0x3C], [0x7E, 0x06, 0x0C, 0x18, 0x30, 0x30, 0x30, 0x30],
    [0x3C, 0x66, 0x66, 0x3C, 0x66, 0x66, 0x66, 0x3C], [0x3C, 0x66, 0x66, 0x66, 0x3E, 0x06, 0x66, 0x3C],
    [0x3C, 0x66, 0x66, 0x7E, 0x66, 0x66, 0x66, 0x66], [0x7C, 0x66, 0x66, 0x7C, 0x66, 0x66, 0x66, 0x7C],
    [0x3C, 0x66, 0x60, 0x60, 0x60, 0x60, 0x66, 0x3C], [0x78, 0x6C, 0x66, 0x66, 0x66, 0x66, 0x6C, 0x78],
    [0x7E, 0x60, 0x60, 0x78, 0x60, 0x60, 0x60, 0x7E], [0x7E, 0x60, 0x60, 0x78, 0x60, 0x60, 0x60, 0x60],
];

unsafe fn draw_byte(fb: u64, x: u32, y: u32, val: u8) {
    let nibbles = [val >> 4, val & 0x0F];
    for (i, &n) in nibbles.iter().enumerate() {
        let glyph = FONT[n as usize];
        for row in 0..8 {
            for col in 0..8 {
                if (glyph[row] & (1 << (7 - col))) != 0 {
                    let px_x = x + (i as u32 * 9) + col;
                    let px_y = y + row as u32;
                    let ptr = (fb + (px_y as u64 * 1280 * 4) + (px_x as u64 * 4)) as *mut u32;
                    ptr.write_volatile(0x00FF00); // Hacker Green
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    unsafe {
        if silicon_ping() && enable_monitor_mode() == () && init_dma_engine() {
            if let Some(idx) = hunt_for_eapol() {
                // Brute force all common Apple framebuffers
                for &fb in &[0x80000000u64, 0x90000000u64, 0xc0000000u64] {
                    let packet = &RX_BUFFERS[idx];
                    let mut cur_x = 40;
                    let mut cur_y = 40;
                    for i in 0..128 {
                        draw_byte(fb, cur_x, cur_y, packet[i]);
                        cur_x += 22;
                        if (i + 1) % 16 == 0 { cur_x = 40; cur_y += 12; }
                    }
                }
            }
        }
        loop {}
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop {} }
