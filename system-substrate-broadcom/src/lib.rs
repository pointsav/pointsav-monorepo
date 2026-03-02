#![no_std]

pub const BCM4322_BASE: usize = 0xd3200000;

// Broadcom Silicon Backplane (SSB) Control Registers
pub const SSB_TMSLOW: usize = 0x0F08; 
pub const SSB_TMSLOW_RESET: u32 = 0x0001;
pub const SSB_TMSLOW_CLOCK: u32 = 0x0002;
pub const SSB_TMSLOW_FGC:   u32 = 0x0004;

// 802.11 Core MAC Control Register
pub const DOT11_MAC_CONTROL: usize = 0x120;

pub unsafe fn write_reg(offset: usize, val: u32) {
    let ptr = (BCM4322_BASE + offset) as *mut u32;
    ptr.write_volatile(val);
}

pub unsafe fn read_reg(offset: usize) -> u32 {
    let ptr = (BCM4322_BASE + offset) as *const u32;
    ptr.read_volatile()
}

pub unsafe fn core_reset() {
    write_reg(SSB_TMSLOW, SSB_TMSLOW_RESET | SSB_TMSLOW_CLOCK | SSB_TMSLOW_FGC);
    let _ = read_reg(SSB_TMSLOW);
    write_reg(SSB_TMSLOW, SSB_TMSLOW_CLOCK | SSB_TMSLOW_FGC);
    let _ = read_reg(SSB_TMSLOW);
    write_reg(SSB_TMSLOW, SSB_TMSLOW_CLOCK);
    let _ = read_reg(SSB_TMSLOW);
}

pub unsafe fn silicon_ping() -> bool {
    core_reset();
    let status = read_reg(DOT11_MAC_CONTROL);
    status != 0xFFFFFFFF
}
