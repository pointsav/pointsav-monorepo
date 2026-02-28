#![no_std]

/// THE SOVEREIGN NETWORK INTERFACE
/// This trait defines the universal behavior for any network device
/// in the PointSav ecosystem, whether physical (Broadcom) or virtual (VirtIO).
pub trait NetworkDevice {
    /// Initialize the device and perform the hardware handshake.
    fn init(&mut self) -> Result<(), NetworkError>;
    
    /// Send a raw packet through the interface.
    fn transmit(&self, buffer: &[u8]) -> Result<(), NetworkError>;
    
    /// Receive a raw packet from the interface.
    fn receive(&self, buffer: &mut [u8]) -> Result<usize, NetworkError>;
}

/// SOVEREIGN ERROR TYPES
#[derive(Debug)]
pub enum NetworkError {
    DeviceNotFound,
    InitializationFailed,
    TransmitBufferFull,
    HardwareFault,
}

/// DISCOVERY: Target Hardware Fingerprints
/// These IDs are physically anchored to our 3-Node Mesh forensics.
pub const TARGET_NIC_LAPTOP_B: (u16, u16) = (0x14e4, 0x432b); // Broadcom BCM4322
pub const TARGET_NIC_IMAC: (u16, u16) = (0x14e4, 0x16b4);     // Broadcom BCM57765
