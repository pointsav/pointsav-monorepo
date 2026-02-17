#![no_std]

//! # system-core
//! The shared substrate for the PointSav Monorepo.

/// Universal Result type for all six tiers.
pub type PointSavResult<T> = core::result::Result<T, CoreError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreError {
    /// Failure in the hardware abstraction layer.
    SubstrateFailure,
    /// Unauthorized attempt to access a capability.
    CapabilityViolation,
    /// Failure during Machine-Based Authorization (MBA) handshake.
    AuthenticationError,
    /// Physical resource (Memory/CPU) exhaustion.
    ResourceExhaustion,
}

/// The Machine-Based Authorization trait.
/// Implemented by any node wishing to join the Private Network.
pub trait MachineIdentity {
    /// Returns the unique hardware-bound capability key.
    fn hardware_key(&self) -> [u8; 32];
    
    /// Executes a cryptographic handshake with the Authority Node.
    fn authorize(&self, challenge: &[u8; 32]) -> PointSavResult<[u8; 32]>;
}
