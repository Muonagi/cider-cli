# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Cider is a CLI tool for signing and sideloading iOS apps, written in Rust (2024 edition). It handles Apple Developer authentication, certificate/provisioning profile management, code signing, and on-device installation. Targets macOS, Linux, and Windows.

## Workspace Structure

Cargo workspace with 5 crates:

```
apps/                          # CLI binary ("cider") — clap + dialoguer
  src/commands/
    sign/                      # Sign + install workflows
    refresh/                   # Refresh registration flows
    account.rs                 # Apple account auth
    device.rs                  # Device management
    session.rs                 # Session handling
    macho.rs                   # Mach-O inspection
crates/
  plume_core/                  # Core library (MPL-2.0 licensed)
    src/auth/                  # Apple auth (GSA/Anisette)
    src/developer/             # Developer portal sessions + provisioning
    src/utils/                 # Certificates, Mach-O parsing, provisioning profiles
  plume_utils/                 # Higher-level helpers
    src/                       # Bundle, package, device, signer, tweak, install
  plume_store/                 # Persistence layer (~/.config/Cider/ or %APPDATA%)
    src/                       # Account storage, refresh tokens, key-value store
  plume_gestalt/               # macOS-only: MobileGestalt wrapper (Mac UDID retrieval)
```

### Dependency graph

`apps` depends on all four library crates. `plume_utils` depends on `plume_core` and `plume_store`. `plume_store` depends on `plume_core`. `plume_gestalt` is standalone (no Rust deps, links native frameworks).

## Build & Run

```sh
cargo build -p cider                   # Debug build
PROFILE=release make dist              # Release build → dist/cider
make clean                             # Remove dist/ and target/
```

Requires: Rust (via rustup), CMake, C++ compiler.

## CLI Commands

```
cider sign       # Sign an iOS app bundle
cider account    # Manage Apple Developer account auth
cider device     # Device management
cider refresh    # Manage refresh registrations
cider inspect    # Inspect Mach-O binaries
```

## Custom Forks

Dependencies under the PlumeImpactor GitHub org, pinned to specific revisions:

| Crate          | Fork repo                                     |
|----------------|-----------------------------------------------|
| `idevice`      | `PlumeImpactor/plume-idevice`                 |
| `apple-codesign`| `PlumeImpactor/plume-apple-platform-rs`       |
| `omnisette`    | `PlumeImpactor/omnisette`                     |
| `decompress`   | `PlumeImpactor/decompress`                    |

When updating these, always pin to a specific `rev` in Cargo.toml.

## Platform-Conditional Compilation

`plume_gestalt` is macOS-only. It links `MobileGestalt` and `CoreFoundation` via `build.rs`. Guard all references with `#[cfg(target_os = "macos")]`. The `apps` crate already does this in its `Cargo.toml` via a `[target.'cfg(target_os = "macos")'.dependencies]` section.

## Key Technical Details

- **Crypto stack**: ring (via rustls), plus aes/aes-gcm/cbc/pbkdf2/hmac/sha2/rsa/srp for Apple auth flows.
- **HTTP**: reqwest 0.11 with rustls-tls (no native TLS).
- **Device communication**: `idevice` crate (custom fork) for USB/network communication with iOS devices.
- **Code signing**: `apple-codesign` crate (custom fork) for Mach-O signing.
- **Feature flag**: `tweaks` — enables optional tweak injection in plume_core/plume_utils. The `apps` crate enables it on plume_core by default.

## Conventions

- **Commits**: conventional commits (`feat:`, `fix:`, `chore:`, `refactor:`, `docs:`, etc.)
- **Release profile**: optimized for binary size — LTO, `panic=abort`, `strip=symbols`, `opt-level="s"`, split debuginfo.
- **Error handling**: `thiserror` for library crates, `anyhow` in the CLI app.
- **Async runtime**: tokio (full features).
- **Formatting**: `rustfmt.toml` with 2024 style edition, 100-char max width, Unix line endings, trailing commas on match block arms, field init shorthand.
