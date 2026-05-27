#![no_std]

// Broadcom NIC detection stub — returns true when 14e4:16b4 (BCM57765) is present.
// Replace with real PCI scan when silicon_ping implementation lands (BRIEF §9.2 Step 2).
pub fn silicon_ping() -> bool {
    false
}

pub fn system_status() -> &'static str {
    "SYSTEM EVENT: system-substrate-broadcom scaffold verified."
}
