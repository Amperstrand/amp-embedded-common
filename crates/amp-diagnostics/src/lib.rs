//! amp-diagnostics — Runtime diagnostics counters for CCID firmware.
//!
//! Shared Amperstrand embedded utility. See module docs for
//! provenance and licensing background (also LICENSES.md at the repo root).
//!
//! Extracted verbatim from ccid-firmware-rs 76f1e8af929130f61b19daf2c8b045a083304d79.

#![no_std]

pub mod diagnostics;

pub use diagnostics::Diagnostics;
