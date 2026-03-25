# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Impactor is a CLI iOS signing and sideloading tool written in Rust (2024 edition). It signs and installs iOS apps on macOS, Linux, and Windows.

## Workspace Structure

This is a Cargo workspace with 5 crates:

- `apps` — CLI application (binary name: `impactor`), uses clap + dialoguer
- `crates/plume_core` — Auth, provisioning, certificates, Mach-O helpers, signing (MPL-2.0)
- `crates/plume_utils` — Bundle, package, device, signing, tweak, install helpers
- `crates/plume_store` — Local account and refresh persistence (~/.config/Impactor/)
- `crates/plume_gestalt` — macOS-only wrapper for libMobileGestalt (Mac UDID for Apple Silicon sideloading)

## Build Commands

```sh
cargo build -p impactor          # Build the CLI
PROFILE=release make dist        # Release build → dist/
make install                     # Install to /usr/local/bin (PREFIX configurable)
cargo test --workspace           # Run all tests
```

Requires: Rust (rustup), CMake, C++ compiler.

## Custom Forks

Several dependencies use custom forks under the PlumeImpactor GitHub org. When updating dependencies, note these are pinned to specific revisions:

- `idevice` → plume-idevice
- `apple-codesign` → plume-apple-platform-rs
- `omnisette` → custom fork
- `decompress` → custom fork

## Platform-Conditional Compilation

`plume_gestalt` is macOS-only — it links against MobileGestalt.framework and CoreFoundation via a build script. Guard any references to it with `#[cfg(target_os = "macos")]`.

## Conventions

- Use conventional commits: `feat:`, `fix:`, `chore:`, `refactor:`, `docs:`, etc.
- Release profile is optimized for binary size (LTO, panic=abort, strip=symbols, opt-level="s").
- The `tweaks` feature flag enables optional functionality in plume_core/plume_utils.
