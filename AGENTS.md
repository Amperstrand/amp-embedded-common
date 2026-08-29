# AGENTS.md — amp-embedded-common

## Project

Shared embedded firmware utilities for Amperstrand MCU projects. First
component: `crates/amp-dwt-watchdog` (DWT cycle-counter watchdog, extracted
verbatim from ccid-firmware-rs `adc6682`).

Licensing is per-component, decided from provenance — read `LICENSES.md`
before adding any component. A component extracted from a GPL repo is GPL;
only textual derivatives of permissive-licensed source may be
`MIT OR Apache-2.0`. Fail closed.

## Commands

```bash
# Host tests (14 in amp-dwt-watchdog)
cargo test --workspace

# ARM build (real DWT impl is compiled; host impl excluded)
cargo build --target thumbv7em-none-eabihf --features cortex-m

# Lint (CI parity)
cargo fmt --check
cargo clippy --all-targets -- -D warnings

# Formatting fix
cargo fmt
```

## Layout

```
crates/amp-dwt-watchdog/   # DWT watchdog (GPL-2.0-or-later)
.github/workflows/ci.yml   # fmt + clippy + test + thumbv7em build
LICENSES.md                # per-component license decisions + evidence
```

## Rules

- The DWT module is a **verbatim code move** from ccid-firmware-rs. Do not
  "improve" its API; changes flow through ccid-firmware-rs first or get an
  explicit re-sync commit.
- `cortex-m` cargo feature is a marker only — the ARM/host split is cfg-gated
  on `all(target_arch = "arm", target_os = "none")` exactly as in
  ccid-firmware-rs. Do not redesign the gating.
- Toolchain pinned to Rust 1.92 in CI (matches ccid-firmware-rs).
