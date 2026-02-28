#![no_std]
#![no_main]

use core::panic::PanicInfo;
// We use the 'sel4' crate to interface with the microkernel
use sel4::{BootInfo, debug_println};

/// Entry point for the PointSav Root Task.
/// The seL4 kernel passes the BootInfo structure to us.
#[no_mangle]
pub extern "C" fn _start(bootinfo: *const BootInfo) -> ! {
    // 1. Capture the 'Foundry' rights (BootInfo)
    // This contains the list of all hardware resources we are allowed to use.
    let _info = unsafe { &*bootinfo };

    // 2. Signal Sovereign Status
    // 'debug_println' sends text to the seL4 kernel log (visible in QEMU/Serial)
    debug_println!("[PointSav] system-substrate: Sovereign Handshake Complete.");
    debug_println!("[PointSav] system-substrate: Running on Laptop B (pc99).");

    // 3. Enter the Governance Loop
    // This keeps the substrate alive while we wait for instructions.
    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    debug_println!("[PointSav] CRITICAL: Substrate Panic: {}", info);
    loop {
        core::hint::spin_loop();
    }
}
