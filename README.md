# amp-embedded-common

Shared embedded firmware utilities for Amperstrand MCU projects — DWT
watchdog now; diagnostics, recovery/self-healing, and USB PHY reset planned.

## Crates

| Crate | Purpose | License |
|---|---|---|
| [`amp-dwt-watchdog`](crates/amp-dwt-watchdog) | Wall-clock timeouts on Cortex-M3+ via the DWT `CYCCNT` cycle counter; host-buildable (AtomicU32 test double) with 14 unit tests | GPL-2.0-or-later |

Licensing is decided **per component from provenance** — see
[LICENSES.md](LICENSES.md) for the decision rule and evidence trails.

## Why

The same DWT cycle-counter watchdog pattern was independently inlined in
multiple Amperstrand firmware repos (ccid-firmware-rs F469/F746 paths,
gm65-scanner). This repo gives the reusable module a single home so timing
code stops being copy-pasted between firmwares.

## Consumers

- **ccid-firmware-rs** (GPL-2.0-or-later) — the extraction source; the module
  `firmware/ccid-firmware/src/dwt_watchdog.rs` can now delegate here.
- gm65-scanner (MIT OR Apache-2.0) — **cannot link `amp-dwt-watchdog`** (GPL);
  it keeps its own inline pattern. Permissive components added to this repo
  later remain consumable.
- microfips — license currently unspecified; GPL components are consumable
  only under GPL terms.

## Usage

```rust
use amp_dwt_watchdog::dwt_watchdog::{self, DwtWatchdog};

unsafe { dwt_watchdog::init() }; // once at startup (no-op on host)
let mut wd = DwtWatchdog::from_ms(200, 168_000_000);
wd.start();
// ... poll:
if wd.expired() { /* recover */ }
```

CYCCNT wraps every ~23.8 s at 180 MHz — keep timeouts well under one wrap.

## Build / test

```bash
cargo test --workspace                                   # 14 host tests
cargo build --target thumbv7em-none-eabihf --features cortex-m  # ARM build
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```
