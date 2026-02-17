#![no_std]
#![no_main]

//! # os-infrastructure
//! The stateless hypervisor providing compute and RAM to the Private Network.

use system_core::{PointSavResult, MachineIdentity};
use system_substrate::Substrate;

#[cfg(feature = "wifi-broadcom")]
use system_substrate_broadcom::BroadcomSubstrate;

pub struct InfrastructureNode<S: Substrate> {
    pub substrate: S,
    pub hardware_id: [u8; 32],
}

impl<S: Substrate> InfrastructureNode<S> {
    pub fn boot(&self) -> PointSavResult<()> {
        self.substrate.boot_sequence()?;
        #[cfg(feature = "wifi-broadcom")]
        {
            let _wifi = BroadcomSubstrate;
        }
        Ok(())
    }
}

impl<S: Substrate> MachineIdentity for InfrastructureNode<S> {
    fn hardware_key(&self) -> [u8; 32] { self.hardware_id }
    fn authorize(&self, challenge: &[u8; 32]) -> PointSavResult<[u8; 32]> {
        let mut response = [0u8; 32];
        for i in 0..32 { response[i] = self.hardware_id[i] ^ challenge[i]; }
        Ok(response)
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // This is where the seL4 kernel will hand over control on Laptop B.
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
