# Reusable CI Workflows

This repository contains reusable GitHub Actions workflows for Rust embedded projects in the Amperstrand ecosystem.

## rust-embedded.yml

A reusable workflow that provides common linting and host-test jobs for Rust embedded projects.

### Usage

In your consumer repository's `.github/workflows/ci.yml`:

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  ci:
    uses: Amperstrand/amp-embedded-common/.github/workflows/rust-embedded.yml@main
    with:
      toolchain: "1.92"
      clippy-target: "thumbv7em-none-eabihf"
      apt-packages: "libpcsclite-dev"
      test-flags: "--workspace --exclude esp32-crate"
```

### Configuration

The workflow accepts the following inputs:

| Input | Type | Default | Description |
|-------|------|---------|-------------|
| `toolchain` | string | `"stable"` | Rust toolchain version (e.g., `"1.92"`, `"nightly"`) |
| `clippy-target` | string | `""` | Optional cross-compile target for clippy (empty = host only) |
| `test-target` | string | `"x86_64-unknown-linux-gnu"` | Build target for host tests |
| `apt-packages` | string | `""` | Space-separated apt packages to install |
| `test-flags` | string | `""` | Additional flags for cargo test |
| `workspace-path` | string | `"."` | Working directory path (default: repo root) |

### Jobs

The workflow provides two jobs:

1. **lint** - Runs `cargo fmt --check` and `cargo clippy -- -D warnings`
   - Runs clippy on host target by default
   - If `clippy-target` is provided, also runs clippy on that target

2. **host-test** - Runs `cargo test` with the specified target and flags
   - Installs system dependencies if `apt-packages` is provided
   - Sets `CARGO_BUILD_TARGET` environment variable

### Setup Requirements

Consumer repos must allow this repository to use reusable workflows:

1. Go to your consumer repo's Settings → Actions → General
2. Under "Workflow permissions", add `Amperstrand/amp-embedded-common` to "Allow reusable workflows from"
3. Ensure the workflow file has `secrets: inherit` if needed

### Examples

#### Simple repo (no system deps, host-only clippy)

```yaml
jobs:
  ci:
    uses: Amperstrand/amp-embedded-common/.github/workflows/rust-embedded.yml@main
    with:
      toolchain: "stable"
```

#### Embedded repo with PC/SC deps and cross-compile clippy

```yaml
jobs:
  ci:
    uses: Amperstrand/amp-embedded-common/.github/workflows/rust-embedded.yml@main
    with:
      toolchain: "1.92"
      clippy-target: "thumbv7em-none-eabihf"
      apt-packages: "libpcsclite-dev"
      test-flags: "--workspace --exclude esp32-crate"
```

#### Workspace in subdirectory

```yaml
jobs:
  ci:
    uses: Amperstrand/amp-embedded-common/.github/workflows/rust-embedded.yml@main
    with:
      toolchain: "nightly"
      workspace-path: "firmware/"
```

### Design Notes

- The workflow is intentionally generic and input-driven
- Matrix builds (e.g., multiple device profiles) stay repo-local by design
- Only common jobs (fmt, clippy, host test) are centralized
- Uses `Swatinem/rust-cache@v2` for efficient dependency caching
- Uses `dtolnay/rust-toolchain@stable` for reliable toolchain installation