#![no_std]

//! # system-substrate
//! Standardized drivers and hardware access primitives.
//! Updated with Wireless Substrate for Air-Gapped Bootstrap.

use system_core::PointSavResult;

pub trait Substrate {
    fn boot_sequence(&self) -> PointSavResult<()>;
}

pub trait WirelessHardware {
    fn connect_wifi(&self, ssid: &str, psk: &str) -> PointSavResult<()>;
    fn get_signal_dbm(&self) -> i8;
}

pub struct SeL4Substrate;

impl Substrate for SeL4Substrate {
    fn boot_sequence(&self) -> PointSavResult<()> {
        Ok(())
    }
}
