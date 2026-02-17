#![no_std]

//! # system-security
//! Proprietary Capability Monitor for Machine-Based Authorization (MBA).

use system_core::{PointSavResult, MachineIdentity};

/// Core capability types for PointSav nodes.
pub enum Capability {
    Compute,
    Network,
    Storage,
}

pub struct CapabilityMonitor;

impl CapabilityMonitor {
    /// Validates the identity and challenge-response of a connecting node.
    pub fn verify_node<M: MachineIdentity>(&self, node: &M, challenge: &[u8; 32]) -> PointSavResult<bool> {
        let response = node.authorize(challenge)?;
        // Simple XOR check for the prototype handshake
        for i in 0..32 {
            if response[i] != (node.hardware_key()[i] ^ challenge[i]) {
                return Ok(false);
            }
        }
        Ok(true)
    }
}
