#![no_std]
#![no_main]

//! # os-network-admin
//! The central routing policy engine and MBA authority.

use system_core::{PointSavResult, MachineIdentity};
use system_substrate::Substrate;
use system_security::{CapabilityMonitor, Capability};

pub struct AuthorityNode<S: Substrate> {
    pub substrate: S,
    pub monitor: CapabilityMonitor,
}

impl<S: Substrate> AuthorityNode<S> {
    pub fn authenticate_node<M: MachineIdentity>(&self, node: &M, _cap: Capability) -> PointSavResult<bool> {
        let challenge = [0x55u8; 32];
        self.monitor.verify_node(node, &challenge)
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Authority Node Logic Loop
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
