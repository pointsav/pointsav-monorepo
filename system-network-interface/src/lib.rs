#![no_std]

// Network interface stubs — all replaced when Genesis Protocol impl lands (BRIEF §9.2 Step 3).
pub fn enable_monitor_mode() {}

pub fn init_dma_engine() -> bool {
    false
}

pub fn hunt_for_eapol() -> Option<usize> {
    None
}

pub static RX_BUFFERS: [[u8; 128]; 1] = [[0u8; 128]; 1];

pub fn system_status() -> &'static str {
    "SYSTEM EVENT: system-network-interface scaffold verified."
}
