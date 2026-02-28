#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// THE SOVEREIGN ENTRY POINT
/// This is where the seL4 kernel hands control over to our Rust logic.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Phase 1: Initialize System Core (Capability-Based Manager)
    // Phase 2: Probe Hardware (Broadcom BCM4322)
    // Phase 3: Expose VirtIO Network Bridge
    
    // For now, we halt the CPU in a safe spin-loop.
    loop {}
}

/// MANDATORY PANIC HANDLER
/// Without an operating system to catch crashes, we must define how to fail.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // In the future, this will dump an error to the EFI Framebuffer.
    loop {}
}
