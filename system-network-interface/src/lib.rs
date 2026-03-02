#![no_std]

use system_substrate_broadcom::{read_reg, write_reg, DOT11_MAC_CONTROL};

pub const DOT11_RX_FILTER: usize = 0x124;
pub const FILTER_PROMISC: u32 = 0x0001;
pub const FILTER_BEACON: u32  = 0x0004;
pub const MAC_ENABLE: u32     = 0x0001;

pub const DMA_RX_CTRL: usize      = 0x240;
pub const DMA_RX_ADDR_LOW: usize  = 0x248;
pub const DMA_RX_ADDR_HIGH: usize = 0x24C;

#[repr(C, align(4))]
pub struct DmaDescriptor {
    pub control: u32,
    pub address: u32,
}

pub const RX_RING_SIZE: usize = 4;
pub const PACKET_SIZE: usize = 2048;

pub static mut RX_RING: [DmaDescriptor; RX_RING_SIZE] = [
    DmaDescriptor { control: 0, address: 0 },
    DmaDescriptor { control: 0, address: 0 },
    DmaDescriptor { control: 0, address: 0 },
    DmaDescriptor { control: 0, address: 0 },
];

// The public array holding the actual raw Wi-Fi data
pub static mut RX_BUFFERS: [[u8; PACKET_SIZE]; RX_RING_SIZE] = [[0; PACKET_SIZE]; RX_RING_SIZE];

pub unsafe fn enable_monitor_mode() {
    let mut mac_ctrl = read_reg(DOT11_MAC_CONTROL);
    mac_ctrl &= !MAC_ENABLE;
    write_reg(DOT11_MAC_CONTROL, mac_ctrl);
    write_reg(DOT11_RX_FILTER, FILTER_PROMISC | FILTER_BEACON);
    mac_ctrl |= MAC_ENABLE;
    write_reg(DOT11_MAC_CONTROL, mac_ctrl);
}

pub unsafe fn is_monitor_mode_active() -> bool {
    (read_reg(DOT11_RX_FILTER) & FILTER_PROMISC) != 0
}

pub unsafe fn init_dma_engine() -> bool {
    for i in 0..RX_RING_SIZE {
        let buffer_ptr = core::ptr::addr_of!(RX_BUFFERS[i]) as u32;
        RX_RING[i].address = buffer_ptr;
        RX_RING[i].control = (PACKET_SIZE as u32) & 0x1FFF; 
    }
    RX_RING[RX_RING_SIZE - 1].control |= 0x8000_0000;

    let ring_ptr = core::ptr::addr_of!(RX_RING) as u32;
    write_reg(DMA_RX_ADDR_LOW, ring_ptr);
    write_reg(DMA_RX_ADDR_HIGH, 0); 
    write_reg(DMA_RX_CTRL, 0x01); 
    
    read_reg(DMA_RX_ADDR_LOW) == ring_ptr
}

// Now returns the index of the buffer holding the handshake!
pub unsafe fn hunt_for_eapol() -> Option<usize> {
    for _ in 0..20_000_000 { // Increased hunt duration slightly
        for i in 0..RX_RING_SIZE {
            let buf = &RX_BUFFERS[i];
            for j in 0..200 { 
                if buf[j] == 0x88 && buf[j + 1] == 0x8E {
                    return Some(i); // WE GOT IT. Return the index.
                }
            }
        }
    }
    None
}
