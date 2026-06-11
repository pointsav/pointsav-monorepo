#![no_std]

/// Probe for a Broadcom PCI NIC (vendor 14e4, device 16b4 — BCM57765).
/// Returns the MMIO base address if the controller is found, None otherwise.
/// Step 2 (Genesis Protocol) implements the real PCI config-space scan via MMIO.
pub fn probe_nic() -> Option<u64> {
    None
}

pub fn system_status() -> &'static str {
    "system-substrate-broadcom: scaffold (Step 2 pending)"
}
