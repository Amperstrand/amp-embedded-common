# LICENSES.md — per-component licensing decisions

This repository is the shared home for embedded-firmware utilities extracted
from Amperstrand MCU projects. Each component carries the license decided by
its **provenance**, recorded below with the evidence trail. The decision rule
(fail-closed):

- Code that is a textual derivative of permissive-licensed source
  (`MIT OR Apache-2.0`) may be `MIT OR Apache-2.0`.
- Code authored inside a GPL repository (or whose provenance is murky)
  is `GPL-2.0-or-later`.

## Consumer license inventory

| Consumer repo | Declared license | Source |
|---|---|---|
| ccid-firmware-rs (`firmware/ccid-firmware`) | `GPL-2.0-or-later` | `Cargo.toml` `license` field |
| ccid-firmware-rs (`crates/ccid-core`) | `GPL-2.0-or-later` | `Cargo.toml` `license` field |
| gm65-scanner (workspace) | `MIT OR Apache-2.0` | root `Cargo.toml` `[workspace.package]` |
| microfips (workspace) | **unspecified** — no `license` field in the workspace manifest or member crate manifests, no top-level LICENSE file | inspected 2026-08-30 |

## Component: `amp-dwt-watchdog` (DWT cycle-counter watchdog)

**Decision: `GPL-2.0-or-later`.**

### What was investigated

`ccid-firmware-rs` documents `firmware/ccid-firmware/src/dwt_watchdog.rs`
(329 lines, 14 tests) as "sourced from gm65-scanner commit 1d7fddc" (ccid
AGENTS.md, Sibling-Repo Improvement Pass table). gm65-scanner is
`MIT OR Apache-2.0`, so if the ccid file were a derivative of gm65 code, this
crate could have been `MIT OR Apache-2.0`.

### Evidence trail (captured 2026-08-30)

- `git fetch origin` in the local gm65-scanner clone: `163ff6f..11e1d90 main -> origin/main`
- `git cat-file -t 1d7fddc` → `commit` (present locally)
- `gh api repos/Amperstrand/gm65-scanner/commits/1d7fddc --jq .sha` →
  `1d7fddc6791f04d94d285abd8846174a1f585a91` (present on GitHub)
- `git log --all --oneline -S'DWT' -- .` → exactly one commit: `1d7fddc`
- `git log --all --oneline -- '**/dwt*'` → **empty** — no DWT module file has
  ever existed anywhere in gm65-scanner history
- Worktree grep for `DWT|cyccnt|CYCCNT` in `*.rs` → **empty** on current main

`git show 1d7fddc` touches only `examples/stm32f469i-disco/src/main.rs`
(+8/−9 lines): it replaced an iteration-count watchdog with 8 inline lines
using the `cortex_m::peripheral::DWT` API
(`dwt.enable_cycle_counter()`, `DWT::cycle_count().wrapping_sub(start) >=
180_000_000 * 6`) — no struct, no module, no host implementation, no tests.

### Analysis

The ccid file and the gm65 commit share only the **idea** (use the DWT CYCCNT
hardware register plus `wrapping_sub` for a wall-clock timeout — an
uncopyrightable method of operation; register addresses and the ~23.8 s wrap
figure are hardware facts/arithmetic). The 329-line ccid file is different
expression on every axis:

| | gm65 `1d7fddc` | ccid `dwt_watchdog.rs` |
|---|---|---|
| Shape | 8 inline lines in `main.rs` | standalone reusable module |
| Register access | `cortex_m::peripheral::DWT` crate API | raw `0xE000_1000` / `0xE000_EDFC` volatile access |
| API | local variables | `DwtWatchdog` struct, `from_ms`, `expired`, `elapsed_cycles`, `remaining_cycles` |
| Host/testing | none | AtomicU32 CYCCNT test double + 14 unit tests |

Git history of the ccid file (`git log --follow -- firmware/ccid-firmware/src/dwt_watchdog.rs`)
shows it was authored entirely inside ccid-firmware-rs by Amperstrand
(`1c69a6b` add, `1f3fbff` wire + clippy, `eb3b61c` Wave 1 pass; all 2026-08-11).

### Conclusion

`dwt_watchdog.rs` is **ccid-original** expression (pattern *inspired by*
permissive gm65 code, no copied expression), authored inside the
`GPL-2.0-or-later` ccid-firmware-rs repository → per the decision rule the
crate is **`GPL-2.0-or-later`**. This is also the fail-closed answer.

**Consumer consequence:** gm65-scanner (`MIT OR Apache-2.0`) cannot link this
crate without relicensing; it retains its own inline 8-line pattern
(`1d7fddc`). ccid-firmware-rs (GPL) and future shared components whose
licensing permits can consume it freely.

## Component: diagnostics (placeholder)

Not yet extracted (ccid `crates/ccid-core/src/diagnostics.rs`). Decide at
extraction time: authored in the GPL ccid repo → presumed
`GPL-2.0-or-later` unless permissive provenance is proven.

## Component: recovery / self-healing (placeholder)

Not yet extracted (ccid SmartcardWrapper re-init pattern, esp32 MFRC522
driver re-init). Decide at extraction time; cross-repo merges default to
`GPL-2.0-or-later` unless every merged source is permissive.

## Component: usb-phy reset (placeholder)

Not yet extracted (ccid issue #22 USB OTG FS PHY reset sequence; ccid AGENTS.md
records the pattern as "sourced from the microfips project" — microfips
declares **no license**, which would fail-closed to `GPL-2.0-or-later`).
Re-run the gate against microfips history before extraction.
