# Cider

[![CI](https://github.com/Muonagi/cider-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/Muonagi/cider-cli/actions/workflows/ci.yml)
[![Release](https://github.com/Muonagi/cider-cli/actions/workflows/release.yml/badge.svg)](https://github.com/Muonagi/cider-cli/actions/workflows/release.yml)

A CLI tool for signing and sideloading iOS apps on macOS, Linux, and Windows.

## Quick Start

```sh
# Build
cargo build -p cider

# Login, trust a device, sign and install
cider account login
cider device trust
cider sign MyApp.ipa --apple-id --install
```

## Installation

### Homebrew

```sh
brew tap Muonagi/cider-cli https://github.com/Muonagi/cider-cli
brew install cider
```

### Shell Script

```sh
curl -fsSL https://raw.githubusercontent.com/Muonagi/cider-cli/main/install.sh | sh
```

### From Source

Requires [Rust](https://rustup.rs/) and [CMake](https://cmake.org/download/). See [Getting Started](docs/getting-started.md) for platform-specific prerequisites.

## Features

- Sign and install `.ipa` and `.app` bundles using Apple ID, ad-hoc, or explicit PEM certificates.
- Customize bundle metadata, entitlements, icons, and compatibility flags.
- Inject tweaks (`.deb`, `.dylib`) and extra bundles (`.framework`, `.appex`).
- Manage Apple Developer accounts, teams, and certificates.
- Pair devices, push pairing files, and install apps directly.
- Save signed apps for scheduled refresh before provisioning profiles expire.
- Inspect and patch Mach-O binaries.

## Commands

| Command | Description |
|---------|-------------|
| `cider sign` | Sign an app, install it, or export a re-signed IPA |
| `cider account` | Manage Apple Developer accounts and teams |
| `cider device` | List devices, pair, install apps, push pairing files |
| `cider refresh` | Manage saved refresh registrations |
| `cider inspect` | Inspect and patch Mach-O binaries |

Run any command with `--help` for usage details, or see the full [Command Reference](docs/commands/).

## Documentation

Full documentation is in the [`docs/`](docs/) directory:

- [Getting Started](docs/getting-started.md) — installation, first steps, common workflows
- [Command Reference](docs/commands/) — every command, subcommand, and flag
- [How Signing Works](docs/signing.md) — the signing pipeline explained
- [Configuration & Storage](docs/configuration.md) — where data lives on disk
- [Architecture](docs/architecture.md) — crate structure and dependency graph
- [Platform Notes](docs/platform-notes.md) — macOS, Linux, and Windows specifics

## Credits

This project is a modified version of [Impactor by CLARATION](https://github.com/CLARATION/Impactor).

## License

This project is licensed under MIT. See `LICENSE` for details. Some bundled components may use different licenses in their own directories.
