#![no_std]

//! # system-substrate-wifi
//! Standardized Wireless Interface for PointSav Private Networks.

use system_core::PointSavResult;

pub trait WirelessSubstrate {
    fn scan_ssids(&self) -> PointSavResult<()>;
    fn connect(&self, ssid: &str, psk: &str) -> PointSavResult<()>;
}
