use core::ptr::{read_volatile, write_volatile};

// 1. You MUST define the struct first!
pub struct PowerManager;

// 2. Constants for MMIO
const SMC_BASE: usize = 0x300;
const SMC_DATA: usize = SMC_BASE + 0x00;
const SMC_COMMAND: usize = SMC_BASE + 0x04;
const SMC_CMD_READ: u8 = 0x10;
const SMC_CMD_WRITE: u8 = 0x11;

const BCM_RX_READY: usize = 0x4000_1014;
const BCM_TX_FIFO:  usize = 0x4000_2018;

impl PowerManager {
    pub fn init_server_mode() {
        Self::smc_write_key(*b"MSLD", 0);
        Self::smc_write_key(*b"F0Tg", 4000);
    }

    pub fn read_cpu_temp() -> u8 {
        (Self::smc_read_key(*b"TC0D") >> 16) as u8
    }

    pub fn read_fan_rpm() -> u16 {
        Self::smc_read_key(*b"F0Ac") as u16
    }

    pub fn has_incoming_telemetry_request() -> bool {
        unsafe {
            (read_volatile(BCM_RX_READY as *const u32) & 0x01) != 0
        }
    }

    pub fn respond_to_admin(temp: u8, _rpm: u16) {
        unsafe {
            let tx_ptr = BCM_TX_FIFO as *mut u8;
            for &b in b"T: " { write_volatile(tx_ptr, b); }
            write_volatile(tx_ptr, (temp / 10) + 48);
            write_volatile(tx_ptr, (temp % 10) + 48);
            write_volatile(tx_ptr, b'C');
            // Flush TX
            write_volatile(0x4000_1020 as *mut u32, 0x01);
        }
    }

    // --- Private SMC Helpers ---
    fn smc_write_key(key: [u8; 4], value: u32) {
        unsafe {
            while (read_volatile(SMC_COMMAND as *const u8) & 0x01) != 0 {}
            write_volatile(SMC_COMMAND as *mut u8, SMC_CMD_WRITE);
            for &b in &key {
                while (read_volatile(SMC_COMMAND as *const u8) & 0x04) != 0 {}
                write_volatile(SMC_DATA as *mut u8, b);
            }
            write_volatile(SMC_DATA as *mut u8, (value >> 24) as u8);
            write_volatile(SMC_DATA as *mut u8, (value >> 16) as u8);
            write_volatile(SMC_DATA as *mut u8, (value >> 8) as u8);
            write_volatile(SMC_DATA as *mut u8, value as u8);
        }
    }

    fn smc_read_key(key: [u8; 4]) -> u32 {
        unsafe {
            while (read_volatile(SMC_COMMAND as *const u8) & 0x01) != 0 {}
            write_volatile(SMC_COMMAND as *mut u8, SMC_CMD_READ);
            for &b in &key {
                while (read_volatile(SMC_COMMAND as *const u8) & 0x04) != 0 {}
                write_volatile(SMC_DATA as *mut u8, b);
            }
            let mut res: u32 = 0;
            res |= (read_volatile(SMC_DATA as *const u8) as u32) << 24;
            res |= (read_volatile(SMC_DATA as *const u8) as u32) << 16;
            res |= (read_volatile(SMC_DATA as *const u8) as u32) << 8;
            res |= read_volatile(SMC_DATA as *const u8) as u32;
            res
        }
    }
}
