//! amp-recovery — Recovery utilities for embedded firmware.
//!
//! Shared Amperstrand embedded utility. See module docs for
//! provenance and licensing background (also LICENSES.md at the repo root).

#![no_std]

pub mod recovery;

// Include usb_phy module for both MCU builds (with PAC access) and host tests
// (mock-only, no PAC dependency)
#[cfg(any(feature = "stm32f4", feature = "stm32f7", test))]
pub mod usb_phy;

pub use recovery::InitRecoveryTracker;
#[cfg(any(feature = "stm32f4", feature = "stm32f7"))]
pub use usb_phy::reset_usb_otg_phy;
