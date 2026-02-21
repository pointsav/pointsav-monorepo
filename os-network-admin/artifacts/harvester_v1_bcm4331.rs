#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

// --- INSTITUTIONAL UEFI REQUISITES ---

#[repr(C)]
pub struct SimpleTextOutputProtocol {
    pub reset: extern "efiapi" fn(*const SimpleTextOutputProtocol, bool) -> usize,
    pub output_string: extern "efiapi" fn(*const SimpleTextOutputProtocol, *const u16) -> usize,
    pub _p: [usize; 3],
    pub set_attribute: extern "efiapi" fn(*const SimpleTextOutputProtocol, usize) -> usize,
    pub clear_screen: extern "efiapi" fn(*const SimpleTextOutputProtocol) -> usize,
}

#[repr(C)]
pub struct SystemTable {
    pub header: [u64; 3],
    pub firmware_vendor: *const u16,
    pub firmware_revision: u32,
    pub _pad: u32,
    pub console_in_handle: usize,
    pub con_in: usize,
    pub console_out_handle: usize,
    pub con_out: *const SimpleTextOutputProtocol,
}

// --- HARDWARE COMMUNICATIONS ---

unsafe fn pci_write_32(addr: u32, val: u32) {
    asm!("out dx, eax", "mov dx, 0xCFC", "mov eax, {0:e}", "out dx, eax", in(reg) val, in("eax") addr, in("dx") 0xCF8 as u16);
}

fn tsc_stall(cycles: u64) {
    unsafe {
        let start: u64;
        let mut current: u64;
        asm!("rdtsc", "shl rdx, 32", "or rax, rdx", out("rax") start, out("rdx") _);
        loop {
            asm!("rdtsc", "shl rdx, 32", "or rax, rdx", out("rax") current, out("rdx") _);
            if current - start >= cycles { break; }
        }
    }
}

// --- ENTRY POINT ---

#[no_mangle]
pub extern "efiapi" fn efi_main(_handle: usize, st: *const SystemTable) -> usize {
    unsafe {
        let con_out = (*st).con_out;
        let wifi_base: u32 = 0x80000000 | (2 << 16) | (0 << 11) | (0 << 8);

        // 1. SILICON WAKE
        pci_write_32(wifi_base | 0x44, 0x0000); 
        pci_write_32(wifi_base | 0x04, 0x0006); 
        pci_write_32(wifi_base | 0x10, 0x90000000); 
        pci_write_32(wifi_base | 0x80, 0x18000000); 
        tsc_stall(2_000_000_000);

        let mmio_base = 0x90000000 as *mut u32;
        core::ptr::write_volatile(mmio_base.add(0x120 / 4), 0x00000001); 

        loop {
            let status = core::ptr::read_volatile(mmio_base.add(0x120 / 4));
            
            if (status & 0x1) != 0 {
                ((*con_out).set_attribute)(con_out, 0x2F); // Green
                ((*con_out).clear_screen)(con_out);

                let mut out_idx = 0;
                let mut display_buffer = [0u16; 65]; 
                
                // 2. THE ALIGNED HARVESTER
                for i in 0..128 {
                    let reg_val = core::ptr::read_volatile(mmio_base.add((0x200 + (i * 4)) / 4));
                    
                    // We shift the read by 8 bits to correct for the 1-byte header ghost
                    let bytes = [
                        ((reg_val >> 8) & 0xFF) as u8,
                        ((reg_val >> 16) & 0xFF) as u8,
                        ((reg_val >> 24) & 0xFF) as u8,
                        (reg_val & 0xFF) as u8,
                    ];

                    for &c in bytes.iter() {
                        if ((c >= 48 && c <= 57) || (c >= 65 && c <= 90) || (c >= 97 && c <= 122) || c == 32 || c == 45 || c == 95) && out_idx < 64 {
                            display_buffer[out_idx] = c as u16;
                            out_idx += 1;
                        }
                    }
                }
                display_buffer[out_idx] = 0;

                if out_idx > 0 {
                    ((*con_out).output_string)(con_out, display_buffer.as_ptr());
                }
                
                tsc_stall(5_000_000_000);
            } else {
                ((*con_out).set_attribute)(con_out, 0x1F); // Blue
                ((*con_out).clear_screen)(con_out);
            }
            tsc_stall(500_000_000);
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop {} }
