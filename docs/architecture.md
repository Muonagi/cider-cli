# Architecture

Cider is organized as a Cargo workspace with five crates.

## Crate overview

```
apps/                  CLI binary ("cider")
crates/
  plume_core/          Core library — auth, developer portal, crypto, Mach-O
  plume_utils/         Higher-level helpers — bundle, signer, device, tweaks
  plume_store/         Persistence — accounts, refresh registrations
  plume_gestalt/       macOS-only — MobileGestalt FFI for Mac UDID
```

## Dependency graph

```
apps
├── plume_core
├── plume_utils
├── plume_store
└── plume_gestalt   (macOS only)

plume_utils
├── plume_core
└── plume_store

plume_store
└── plume_core

plume_gestalt        (standalone, links native frameworks)
```

## Crate responsibilities

### apps

The `cider` CLI application. Uses `clap` for argument parsing, `dialoguer` for interactive prompts, and `indicatif` for progress bars. Error handling uses `anyhow`. Each command lives in its own module under `src/commands/`.

### plume_core

Low-level building blocks shared across the workspace:

- **auth/** — Apple authentication via SRP and GSA protocols with Anisette support (via the `omnisette` fork).
- **developer/** — Developer portal session management, team listing, device/app registration, provisioning profile requests. Supports both the v1 portal API and the QueryHub (QH) API.
- **utils/** — Certificate generation and parsing, Mach-O binary inspection and patching, provisioning profile reading.

Licensed under MPL-2.0.

### plume_utils

Higher-level abstractions used by the CLI:

- **Bundle** — wraps an `.app` directory, reads/writes `Info.plist`, collects nested bundles.
- **Package** — wraps an `.ipa` archive, extracts to a temp directory, re-zips on export.
- **Signer** — orchestrates the signing pipeline: bundle modification, Apple registration, code signing, and installation.
- **Device** — wraps `idevice` for pairing, installing, and querying connected iOS devices.
- **Tweak** — extracts `.deb` files and injects dylibs/frameworks into bundles.

### plume_store

Persistence layer backed by a JSON file at `~/.config/Cider/accounts.json` (see [Configuration & Storage](configuration.md)). Manages:

- Saved Apple accounts and their tokens.
- The currently selected account.
- Refresh registrations per device.

### plume_gestalt

Minimal macOS-only crate that links `MobileGestalt.framework` and `CoreFoundation` via `build.rs` to retrieve the local Mac's UDID for Apple Silicon sideloading. Guarded by `#[cfg(target_os = "macos")]` everywhere it's referenced.

## Custom forks

Several dependencies are maintained as forks under the PlumeImpactor GitHub organization, pinned to specific revisions:

| Crate | Fork |
|-------|------|
| `idevice` | `PlumeImpactor/plume-idevice` |
| `apple-codesign` | `PlumeImpactor/plume-apple-platform-rs` |
| `omnisette` | `PlumeImpactor/omnisette` |
| `decompress` | `PlumeImpactor/decompress` |

## Feature flags

- **`tweaks`** — enables tweak/deb injection in `plume_core` and `plume_utils`. Enabled by default in the `apps` crate.

## Platform-conditional compilation

- `#[cfg(target_os = "macos")]` gates `plume_gestalt` usage and the `--mac` flag.
- `#[cfg(all(target_os = "macos", target_arch = "aarch64"))]` restricts Mac sideloading to Apple Silicon.
- The `apps` crate uses a `[target.'cfg(target_os = "macos")'.dependencies]` section to conditionally depend on `plume_gestalt`.
